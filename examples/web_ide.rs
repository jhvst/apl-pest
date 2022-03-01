use std::{process::Command, str::from_utf8, io::Read};

use salvo::{prelude::*, extra::serve::StaticFile};
use apl_pest::{parse, phases, vk_compute};
use apl_pest::{Phase, Phases};
use rivi_loader::DebugOption;

#[fn_handler]
async fn compute(req: &mut Request, res: &mut Response) {
    let form = req.form_data().await.unwrap();
    let phases = form.fields.get("phases").unwrap().trim();
    let input = form.fields.get("input").unwrap().trim();
    let vk: crate::Phases = serde_json::from_str(phases).unwrap();
    println!("{}, {}", phases, input);
    let result = vk_compute(vk, input);
    res.render_plain_text(&format!("{:?}", result));
}

#[fn_handler]
async fn caps(_req: &mut Request, res: &mut Response) {
    let vk = rivi_loader::new(DebugOption::None).unwrap();
    res.render_plain_text(&format!("{}", vk));
}

#[fn_handler]
async fn savilerow(_req: &mut Request, res: &mut Response) {
    let mut cursor = std::io::Cursor::new(&include_bytes!("./vulkan.eprime")[..]);
    let mut model = String::new();
    cursor.read_to_string(&mut model).unwrap();
    res.render_plain_text(&model);
}

#[fn_handler]
async fn solve(req: &mut Request, res: &mut Response) {
    let form = req.form_data().await.unwrap();
    let param = form.fields.get("param").unwrap().trim();

    let idris = Command::new("podman")
        .arg("run")
        .arg("savilerow")
        .arg(format!("{}/examples/vulkan.eprime", env!("CARGO_MANIFEST_DIR")))
        .arg("-params")
        .arg(param)
        .arg("-run-solver")
        .arg("-solutions-to-stdout")
        .output()
        .expect("failed to execute savilerow");

    res.render_plain_text(&from_utf8(&idris.stdout).unwrap().trim());
}

#[fn_handler]
async fn hello_world(req: &mut Request, res: &mut Response) {
    let form = req.form_data().await.unwrap();
    let input = form.fields.get("input").unwrap().trim();
    let idris_fmt = parse(input).unwrap();

    let phases = phases(&idris_fmt)
        .iter()
        .enumerate()
        .map(|(idx, phase)| {
            let sub_idris = Command::new("idris")
                .arg(format!("{}/examples/Shaped.idr", env!("CARGO_MANIFEST_DIR")))
                .arg("-e")
                .arg(&phase)
                .output()
                .expect("failed to execute idris");

            let output = from_utf8(&sub_idris.stdout).unwrap().trim().to_owned();

            let abstract_type = output.split(':').next().unwrap();
            let mut abstract_type_iter = abstract_type.split(' ');

            let kind = abstract_type_iter.next().unwrap();

            let size = match kind {
                "SomeScalar" => {
                    1
                }
                "SomeVect" => {
                    abstract_type_iter.next().unwrap().parse::<usize>().unwrap()
                }
                _ => unreachable!{}
            };

            Phase {
                idx: idx,
                input: phase.to_string(),
                output: output,
                size: size,
            }
        })
        .collect::<Vec<_>>();

    res.render_json(&Phases { phases });
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .get(StaticFile::new("examples/index.html"))
        .post(hello_world)
    .push(Router::with_path("vk")
        .get(caps)
        .post(compute)
    )
    .push(Router::with_path("smt")
        .get(savilerow)
        .post(solve)
    );
    Server::new(TcpListener::bind("127.0.0.1:7878")).serve(router).await;
}
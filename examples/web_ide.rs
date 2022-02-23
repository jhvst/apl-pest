use std::{process::Command, str::from_utf8, io::Read};

use salvo::{prelude::*, extra::serve::StaticFile};
use apl_pest::parse;
use rivi_loader::DebugOption;

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

    let idris = Command::new("idris")
        .arg(format!("{}/examples/Shaped.idr", env!("CARGO_MANIFEST_DIR")))
        .arg("-e")
        .arg(&idris_fmt)
        .output()
        .expect("failed to execute idris");

    let output = format!(
        "{}\n{}\n{}",
        idris_fmt,
        from_utf8(&idris.stdout).unwrap().trim(),
        input,
    );
    res.render_plain_text(&output);
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .get(StaticFile::new("examples/index.html"))
        .post(hello_world)
    .push(Router::with_path("vk")
        .get(caps)
    )
    .push(Router::with_path("smt")
        .get(savilerow)
        .post(solve)
    );
    Server::new(TcpListener::bind("127.0.0.1:7878")).serve(router).await;
}
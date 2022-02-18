use std::{process::Command, str::from_utf8};

use salvo::{prelude::*, extra::serve::StaticFile};
use apl_pest::parse;

#[fn_handler]
async fn hello_world(req: &mut Request, res: &mut Response) {
    let form = req.form_data().await.unwrap();
    let input = form.fields.get("input").unwrap().trim();
    let idris_fmt = parse(input).unwrap();

    let idris = Command::new("idris")
        .arg(format!("{}/src/Shaped.idr", env!("CARGO_MANIFEST_DIR")))
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
        .post(hello_world);
    Server::new(TcpListener::bind("127.0.0.1:7878")).serve(router).await;
}
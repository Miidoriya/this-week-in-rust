use anyhow::Result;
use wasm_workers_rs::{
    worker,
    http::{self, Request, Response},
    Content, Cache,
};

#[worker(cache)]
fn handler(_req: Request<String>, cache: &mut Cache) -> Result<Response<Content>> {
    Ok(http::Response::builder()
        .status(200)
        .header("x-generated-by", "wasm-workers-server")
        .body(String::from("Hello wasm!").into())?)
}
use anyhow::Result;
use wasm_workers_rs::{
    worker,
    http::{self, Request, Response},
    Content
};

#[worker]   // <-- This is the magic
fn reply(req: Request<String>) -> Result<Response<Content>> {
    Ok(http::Response::builder()
        .status(200)
        .header("x-generated-by", "wasm-workers-rs")
        .body(String::from("Hello wasm!").into())?)
}
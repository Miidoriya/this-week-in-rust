use anyhow::Result;
use wasm_workers_rs::{
    worker,
    http::{self, HeaderValue, Request, Response},
    Content
};

#[worker]   // <-- This is the magic
fn reply(req: Request<String>) -> Result<Response<Content>> {
    let response = format!(
        "<!DOCTYPE html>
<body>
<h1>Hello World</h1>
<p>Replying to {}</p>
<p>Method: {}</p>
<p>User Agent: {}</p>
<p>Body: {}</p>
<p>This page was generated by a Wasm modules built from Rust.</p>
</body>",
        req.uri(),
        req.method().as_str(),
        req.headers().get("user-agent").unwrap_or(&HeaderValue::from_str("None").unwrap()).to_str().unwrap(),
        req.body()
    );

    Ok(http::Response::builder()
        .status(200)
        .header("x-generated-by", "wasm-workers-rs")
        .body(response.into())?)
}
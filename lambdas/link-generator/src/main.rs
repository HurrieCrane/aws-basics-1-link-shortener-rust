mod generator;
mod store;

use lambda_http::{
    http::StatusCode, service_fn, Body, Body::Text, Error, Request, RequestExt, Response,
};
use serde_json::json;

use generator::generate;

#[derive(Debug)]
pub struct ServiceError {
    message: String,
    status: StatusCode,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(handler)).await?;
    Ok(())
}

async fn handler(event: Request) -> Result<Response<Body>, Error> {
    let path_params = event.query_string_parameters();
    let uri = match path_params.first("uri") {
        Some(s) => s,
        None => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Text(
                    json!( { "errorMsg": "uri is a required parameter" } ).to_string(),
                ))
                .expect("unable to create response error for missing uri"))
        }
    };

    let tiny_url = match generate(uri).await {
        Ok(u) => u,
        Err(e) => {
            return Ok(Response::builder()
                .status(e.status)
                .body(Text(json!( { "errorMsg": e.message } ).to_string()))
                .expect("unable to create error body"))
        }
    };

    return Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Text(json!({ "uri": tiny_url }).to_string()))
        .expect("unable to return body"));
}

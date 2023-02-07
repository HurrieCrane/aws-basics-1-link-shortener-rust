mod generator;
mod store;

use lambda_http::Body::Text;
use lambda_http::{http, service_fn, Body, Error, Request, RequestExt, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

use generator::generate;

#[derive(Debug)]
pub struct ServiceError {
    message: String,
    status: http::StatusCode,
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
                .status(400)
                .body(Body::Text(
                    " { \"errorMsg\": \"uri is a required parameter\" } ".to_string(),
                ))
                .expect("unable to create response error for missing uri"))
        }
    };

    let tiny_url = generate(uri).await.unwrap();

    return Ok(Response::builder()
        .status(200)
        .body(Text(json!({ "uri": tiny_url }).to_string()))
        .expect("unable to return body"));
}

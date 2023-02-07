mod resolver;
mod store;

use crate::resolver::resolve_hash;
use lambda_http::Body::Empty;
use lambda_http::{
    http::StatusCode, service_fn, Body, Body::Text, Error, Request, RequestExt, Response,
};
use serde_json::json;

#[derive(Debug)]
pub struct ServiceError {
    message: String,
    status: StatusCode,
}

const QUERY_PARAM_NAME: &'static str = "hash";

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(handler)).await?;
    Ok(())
}

async fn handler(event: Request) -> Result<Response<Body>, Error> {
    let path_params = event.path_parameters();
    let hash = match path_params.first(QUERY_PARAM_NAME) {
        Some(h) => h,
        None => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Text(
                    json!( { "errorMsg": "you must provide a hash" } ).to_string(),
                ))
                .expect(""))
        }
    };

    let redirect_uri = match resolve_hash(hash).await {
        Ok(u) => u,
        Err(e) => {
            return Ok(Response::builder()
                .status(e.status)
                .body(Text(json!( { "errorMsg": e.message } ).to_string()))
                .expect("unable to create error body"))
        }
    };

    return Ok(Response::builder()
        .status(StatusCode::MOVED_PERMANENTLY)
        .header("Location", redirect_uri)
        .body(Empty)
        .expect(""));
}

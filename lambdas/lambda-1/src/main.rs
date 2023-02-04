use lambda_http::{service_fn, Body, Error, Request, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ServiceError {
    message: String,
    status: u16,
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(handler)).await?;
    Ok(())
}

async fn handler(_event: Request) -> Result<Response<Body>, Error> {
    return Ok(Response::builder()
        .status(200)
        .body(Body::Empty)
        .expect("unable to return body"))
}

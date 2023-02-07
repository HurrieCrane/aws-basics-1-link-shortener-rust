use crate::store::store_uri;
use crate::ServiceError;

use std::env;

use crypto::digest::Digest;
use crypto::sha1::Sha1;
use http::StatusCode;
use lambda_http::http;
use url::Url;

const DEFAULT_TINY_DOMAIN: &'static str = "https://link.thestudio.com/";
const DOMAIN_ENV_KEY: &'static str = "LINK_DOMAIN";

pub async fn generate(uri: &str) -> Result<String, ServiceError> {
    let link = match Url::parse(uri) {
        Ok(l) => l,
        Err(e) => {
            return Err(ServiceError {
                message: e.to_string(),
                status: StatusCode::BAD_REQUEST,
            })
        }
    };

    let mut hasher = Sha1::new();
    hasher.input_str(link.as_str());

    let hash = hasher.result_str();

    let domain = env::var(DOMAIN_ENV_KEY).unwrap_or(DEFAULT_TINY_DOMAIN.to_string());

    let short_uri = format!("{domain}link/{hash}");

    store_uri(hash, uri).await?;

    return Ok(short_uri);
}

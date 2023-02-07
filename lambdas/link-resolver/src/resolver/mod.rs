use crate::store::get_uri_for_hash;
use crate::ServiceError;

pub async fn resolve_hash(hash: &str) -> Result<String, ServiceError> {
    return Ok(get_uri_for_hash(hash).await?);
}

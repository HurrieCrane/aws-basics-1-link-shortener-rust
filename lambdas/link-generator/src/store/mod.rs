use crate::ServiceError;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb as dynamodb;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::model::AttributeValue::S;
use lambda_http::http;
use std::collections::HashMap;

#[cfg(debug_assertions)]
use aws_sdk_dynamodb::Credentials;

const DYNAMO_DB_TABLE_NAME: &'static str = "shortened-links";

#[cfg(debug_assertions)]
fn create_local_credentials() -> Credentials {
    Credentials::new("example", "example", None, None, "example")
}

#[cfg(debug_assertions)]
async fn load_dynamo_client() -> dynamodb::Client {
    let region_provider = RegionProviderChain::default_provider().or_else("localhost");
    let config = aws_config::from_env()
        .region(region_provider)
        .endpoint_url("http://localhost:8000")
        .credentials_provider(create_local_credentials())
        .load()
        .await;

    return dynamodb::Client::new(&config);
}

#[cfg(not(debug_assertions))]
async fn load_dynamo_client() -> dynamodb::Client {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    return dynamodb::Client::new(&config);
}

pub async fn store_uri(hash: String, uri: &str) -> Result<(), ServiceError> {
    let client = load_dynamo_client().await;

    let mut item: HashMap<String, AttributeValue> = HashMap::with_capacity(2);
    item.insert("link-hash".to_string(), S(hash));
    item.insert("link".to_string(), S(uri.to_string()));

    let put_item_request = client
        .put_item()
        .set_item(Option::from(item))
        .set_table_name(Some(DYNAMO_DB_TABLE_NAME.to_string()));

    return match put_item_request.send().await {
        Ok(_i) => Ok(()),
        Err(e) => Err(ServiceError {
            message: e.to_string(),
            status: http::StatusCode::INTERNAL_SERVER_ERROR,
        }),
    };
}

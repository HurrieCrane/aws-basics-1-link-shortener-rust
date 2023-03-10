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

pub async fn get_uri_for_hash(hash: &str) -> Result<String, ServiceError> {
    let client = load_dynamo_client().await;

    let mut item: HashMap<String, AttributeValue> = HashMap::with_capacity(1);
    item.insert("link-hash".to_string(), S(hash.parse().unwrap()));

    let get_item_request = client
        .get_item()
        .set_key(Option::from(item))
        .set_table_name(Some(DYNAMO_DB_TABLE_NAME.to_string()));

    let items = match get_item_request.send().await {
        Ok(i) => i,
        Err(e) => {
            return Err(ServiceError {
                message: e.to_string(),
                status: http::StatusCode::INTERNAL_SERVER_ERROR,
            })
        }
    };

    let uri = match items.item() {
        Some(i) => match i["link"].as_s() {
            Ok(l) => l,
            Err(_e) => {
                return Err(ServiceError {
                    message: "unable to retrieve link from table".to_string(),
                    status: http::StatusCode::INTERNAL_SERVER_ERROR,
                })
            }
        },
        None => {
            return Err(ServiceError {
                message: "requested tiny link does not exist".to_string(),
                status: http::StatusCode::NOT_FOUND,
            })
        }
    };

    return Ok(uri.clone());
}

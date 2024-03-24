use lambda_http::tracing::subscriber::fmt::format;
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use qdrant_client::prelude::*;
use qdrant_client::qdrant::{value::Kind, Struct};
use qdrant_client::qdrant::{
    vectors_config::Config, CreateCollection, Distance, FieldType, PointId, PointStruct, Value,
    VectorParams, Vectors, VectorsConfig,
};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
// make sure to have cohere and qdrant setup

const SEARCH_LIMIT: u64 = 5;
// check
fn error_response(code: u16, message: &str) -> Response<Body> {
    Response::builder()
        .status(code)
        .body(Body::from(message))
        .unwrap()
}

// needed because of the reqeust, although we're not using most of them
#[derive(Debug, serde::Deserialize)]
struct CohereResponse {
    id: String,
    texts: Vec<String>,
    embeddings: Vec<Vec<f32>>,
    meta: Meta,
    response_type: String,
}

#[derive(Debug, serde::Deserialize)]
struct Meta {
    api_version: ApiVersion,
    billed_units: BilledUnits,
}

#[derive(Debug, serde::Deserialize)]
struct ApiVersion {
    version: String,
}

#[derive(Debug, serde::Deserialize)]
struct BilledUnits {
    input_tokens: u32,
}

async fn send_request(client: &Client, cohere_api_key: &str) -> Result<CohereResponse, Error> {
    let response: CohereResponse = client
        .post("https://api.cohere.ai/embed")
        .header("Authorization", format!("Bearer {}", cohere_api_key))
        .header("Content-Type", "application/json")
        .header("Cohere-Version", "2021-11-08")
        .json(&serde_json::json!({
            "texts": ["your_text_here"],
            "model": "small"
        }))
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}

async fn function_handler(
    event: Request,
    client: &Client,
    cohere_api_key: &str,
    qdrant_client: &QdrantClient,
    collection_name: &str,
) -> Result<Response<Body>, Error> {
    println!("Request URL: {:?}", event);
    // do a crud operation based on this or query or viz
    let method = event.method();
    let r_type = method.to_string();
    let mut response_body: String = String::new();
    // have to make this mutable, not needed anymore
    let mut cohere_response = CohereResponse {
        id: String::new(),
        texts: Vec::new(),
        embeddings: Vec::new(),
        meta: Meta {
            api_version: ApiVersion {
                version: String::new(),
            },
            billed_units: BilledUnits { input_tokens: 42 },
        },
        response_type: String::new(),
    };
    // check type
    println!("method is: {}", r_type);
    // need to match method type
    let Some(params) = event.query_string_parameters_ref() else {
        return Ok(error_response(400, "Missing query string parameters"));
    };
    let Some(query) = params.first("sports") else {
        return Ok(error_response(400, "Missing query string parameter `sports`"));
    };
    println!("check is: {}", r_type);

    println!("Request params: {:?}", params);
    println!("Request query: {}", query);
    let response = client
        .post("https://api.cohere.ai/embed")
        .header("Authorization", &format!("Bearer {}", cohere_api_key))
        .header("Content-Type", "application/json")
        .header("Cohere-Version", "2021-11-08")
        .body(format!("{{\"texts\":[\"{}\"],\"model\":\"small\"}}", query))
        .send()
        .await?;

    // Debug: Print request parameters
    println!("Request URL: {}", response.url());
    println!("Request Headers: {:#?}", response.headers());
    println!("Request Body: {:?}", response.text().await);
    cohere_response = client
        .post("https://api.cohere.ai/embed")
        .header("Authorization", &format!("Bearer {cohere_api_key}"))
        .header("Content-Type", "application/json")
        .header("Cohere-Version", "2021-11-08")
        .body(format!("{{\"texts\":[\"{query}\"],\"model\":\"small\"}}"))
        .send()
        .await?
        .json()
        .await?;
    // grab similarity
    response_body = qdrant_client
        .search_points(&SearchPoints {
            collection_name: collection_name.to_string(),
            vector: cohere_response
                .embeddings
                .into_iter()
                .next()
                .ok_or("Empty output from embedding")?,
            limit: SEARCH_LIMIT as u64,
            with_payload: Some(true.into()),
            ..Default::default()
        })
        .await?
        .result
        .into_iter()
        .map(|p| {
            format!(
                "<div>{}</div>",
                Value {
                    kind: Some(Kind::StructValue(Struct { fields: p.payload }))
                },
            )
        })
        .collect();
    // build response, maybe make it prettier
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(response_body.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let client = Client::builder().build().unwrap();

    let cohere_api_key = std::env::var("COHERE_API_KEY").expect("need COHERE_API_KEY set");
    let collection_name: String = "Testing".to_string();
    let qdrant_uri = std::env::var("QDRANT_URI").expect("need QDRANT_URI set");
    let mut config = QdrantClientConfig::from_url(&qdrant_uri);
    config.api_key = std::env::var("QDRANT_API_KEY").ok();
    let qdrant_client = QdrantClient::new(Some(config)).expect("Failed to connect to Qdrant");
    if !qdrant_client
        .collection_exists(collection_name.clone())
        .await?
    {
        panic!("Collection {} not found", collection_name);
    }

    run(service_fn(|req| {
        function_handler(
            req,
            &client,
            &cohere_api_key,
            &qdrant_client,
            &collection_name,
        )
    }))
    .await
}

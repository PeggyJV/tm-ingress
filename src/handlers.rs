use abscissa_core::tracing::log::{error, info};
use abscissa_core::Application;
use axum::{
    http::HeaderValue,
    response::{IntoResponse, Response},
    Json,
};
use cosmrs::tx::Raw;
use hyper::{body, Body, Method, Request, StatusCode};
use reqwest::Url;
use tendermint_rpc::{endpoint::broadcast::tx_commit, HttpClient};

use crate::{
    prelude::APP,
    rpc::{JsonRpcRequest, JsonRpcResponse},
};

pub async fn execute_get(url: &str) -> Response {
    reqwest::get(url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
        .into_response()
}

pub async fn root_rpc_handler(req: Request<Body>) -> Response {
    let config = APP.config();
    info!("request: {:?}", req);
    let mut headers = req.headers().to_owned();
    let http_method = req.method().to_owned();
    let body = body::to_bytes(req).await.unwrap();
    info!("body: {:?}", std::str::from_utf8(&body));

    // If not a JSON-RPC request, return the response of a GET at /
    if http_method == Method::GET || body.is_empty() {
        let client = reqwest::Client::new();
        let url = config.node.rpc.clone();
        headers.insert("Content-Type", HeaderValue::from_static("text/html"));
        let request = client.get(&url).headers(headers).build().unwrap();

        let response = client
            .execute(request)
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
            .into_response();

        return Response::builder()
            .header("Content-Type", HeaderValue::from_static("text/html"))
            .body(response.into_body())
            .unwrap();
    }

    // If it *is* a JSON-RPC request, we relay the request, returning the JSON-RPC response as is
    let request = match serde_json::from_slice::<JsonRpcRequest>(&body) {
        Ok(req) => req,
        Err(_) => return JsonRpcResponse::parse_error().into_response(),
    };

    match request.method.as_str() {
        "status" | "broadcast_tx_commit" => {
            let client = reqwest::Client::new();
            let url = Url::parse(&config.node.rpc.clone()).unwrap_or_else(|_| {
                panic!("failed to parse node RPC url: {}", &config.node.rpc.clone())
            });
            let host = url.host().unwrap();
            let port = url.port_or_known_default().unwrap();
            let request = match client
                .post(&config.node.rpc.clone())
                .header("Content-Type", "application/json")
                .header("Content-Length", body.len())
                .header("Accept", "*/*")
                .header("Host", &format!("{}:{}", host, port))
                .body(body)
                .build()
            {
                Ok(r) => r,
                Err(e) => {
                    error!("failed to build JSON-RPC request: {}", e);
                    return JsonRpcResponse::internal_error(request.id).into_response();
                }
            };

            info!("JSON-RPC request: {:?}", request);

            return client
                .execute(request)
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap()
                .into_response();
        }
        _ => JsonRpcResponse::method_not_found(request.id, request.method).into_response(),
    }
}

pub async fn rpc_broadcast_tx_commit(
    mut req: Request<Body>,
) -> Result<Json<tx_commit::Response>, StatusCode> {
    info!("received broadcast_tx request!");
    let bytes: Vec<u8> = body::to_bytes(req.body_mut()).await.unwrap().into();
    let tx = Raw::from_bytes(&bytes).unwrap();
    let config = APP.config();
    let client = HttpClient::new(config.node.rpc.as_str())
        .expect("failed to establish connection to Tendermint RPC");

    Ok(Json(tx.broadcast_commit(&client).await.unwrap()))
}

pub async fn rpc_status(_: Request<Body>) -> Response {
    let config = APP.config();
    let mut url = config.node.rpc.clone();
    url.push_str("/status");

    execute_get(&url).await
}

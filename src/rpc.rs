use std::net::SocketAddr;

use abscissa_core::{tracing::log::info, Application};
use axum::{
    body::{Body, Bytes},
    http::{Request, StatusCode, response::{Builder, self}, Extensions},
    response::{Response, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use axum_jrpc::{JrpcResult, JsonRpcExtractor, JsonRpcRequest, JsonRpcResponse};
use cosmrs::tx::Raw;
use hyper::{body, Uri};
use serde::Deserialize;
use tendermint_rpc::{endpoint::broadcast::tx_commit, Client, HttpClient};
use tower_http::trace::TraceLayer;

use crate::application::APP;

pub async fn broadcast_tx_commit(
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

// pub async fn status(_: Bytes) -> Result<JsonRpcResponse, StatusCode> {
// }

pub async fn router(req: Request<Bytes>) -> Response {
    let config = APP.config();
    let url = config.node.rpc;
    let result = serde_json::from_slice::<JsonRpcRequest>(req.body()).ok();

    // If not a JSON-RPC request, return the response of a GET at /
    if result.is_none() {
        let client = hyper::Client::new();
        let res = client.get(Uri::from_static(&url)).await.unwrap();
        let (parts, body) = res.into_parts();
        let bytes = body::to_bytes(body).await.unwrap();

        return Response::from_parts(parts, body.into)
    }

    let request = result.unwrap();
    let client = HttpClient::new(config.node.rpc.as_str()).unwrap();
    match request.method.as_str() {
        "status" => {
            match client.status().await {
                Ok(res) => return JsonRpcResponse::success(request.id, res).into_response(),
                Err(err) => ,
            };
        },
        _ =>
    }

    Ok(())
    // Ok(JsonRpcResponse::success(-1, client.status().await.unwrap()))
}

/// Runs the tx server
pub async fn relay_rpc(address: &SocketAddr) -> Result<(), hyper::Error> {
    let app = Router::new()
        .route("/", get(router))
        .route("/", post(router))
        .route("/broadcast_tx_commit", post(broadcast_tx_commit))
        // .route("/status", post(status))
        .layer(TraceLayer::new_for_http());

    info!("listening at {}", &address);
    info!("and yes I'm REALLY updating");
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
}

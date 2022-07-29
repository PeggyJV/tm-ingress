use std::net::SocketAddr;

use abscissa_core::{
    tracing::log::{info},
};
use axum::{
    http::{StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};


use serde::{Deserialize, Serialize};
use serde_json::{value::RawValue, Value};
use tower_http::trace::TraceLayer;

use crate::{
    handlers::{rpc_broadcast_tx_commit, root_rpc_handler, rpc_status},
};

#[derive(Deserialize, Debug, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JsonRpcRequest {
    pub id: Value,
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JsonRpcResponse {
    pub id: Option<Value>,
    pub jsonrpc: String,
    pub error: Option<JsonRpcError>,
    pub result: Option<Value>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    pub data: Option<Box<RawValue>>,
}

impl JsonRpcResponse {
    pub fn parse_error() -> Self {
        JsonRpcResponse {
            id: None,
            jsonrpc: "2.0".to_string(),
            error: Some(JsonRpcError {
                code: -32700,
                message: String::from("Parse error"),
                data: None,
            }),
            result: None,
        }
    }

    pub fn method_not_found(id: Value, method: String) -> Self {
        JsonRpcResponse {
            id: Some(id),
            jsonrpc: "2.0".to_string(),
            error: Some(JsonRpcError {
                code: -32601,
                message: format!("Method {} not found", method),
                data: None,
            }),
            result: None,
        }
    }

    pub fn internal_error(id: Value) -> Self {
        JsonRpcResponse {
            id: Some(id),
            jsonrpc: "2.0".to_string(),
            error: Some(JsonRpcError {
                code: -32603,
                message: format!("Internal error"),
                data: None,
            }),
            result: None,
        }
    }
}

impl IntoResponse for JsonRpcResponse {
    fn into_response(self) -> Response {
        if let Some(_) = &self.error {
            // TO-DO: more accurate response codes depending on error type
            return (
                StatusCode::BAD_REQUEST,
                serde_json::ser::to_vec(&self).unwrap(),
            )
                .into_response();
        }

        return (StatusCode::OK, serde_json::ser::to_vec(&self).unwrap()).into_response();
    }
}

/// Runs the rpc server
pub async fn serve(address: &SocketAddr) -> Result<(), hyper::Error> {
    let app = Router::new()
        .route("/", get(root_rpc_handler).post(root_rpc_handler))
        .route("/broadcast_tx_commit", post(rpc_broadcast_tx_commit))
        .route("/status", get(rpc_status).post(rpc_status))
        .layer(TraceLayer::new_for_http());

    info!("listening at {}", &address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
}

use std::net::SocketAddr;

use abscissa_core::tracing::log::info;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};

use serde::{Deserialize, Serialize};
use serde_json::{value::RawValue, Value};
use tower_http::trace::TraceLayer;

use crate::handlers::{root_rpc_handler, rpc_broadcast_tx_commit, rpc_status};

#[derive(Deserialize, Debug, Serialize)]
#[serde(deny_unknown_fields)]
/// Represent a JSON-RPC request object. For more information see https://www.jsonrpc.org/specification#request_object
pub struct JsonRpcRequest {
    pub id: Value,
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(deny_unknown_fields)]
/// Represents a JSON-RPC response object. For more inforamation see https://www.jsonrpc.org/specification#response_object
pub struct JsonRpcResponse {
    pub id: Option<Value>,
    pub jsonrpc: String,
    pub error: Option<JsonRpcError>,
    pub result: Option<Value>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(deny_unknown_fields)]
/// Represents the `error` struct field of a response
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    pub data: Option<Box<RawValue>>,
}

impl JsonRpcResponse {
    /// Represents error caused by a JSON-RPC request that could not be parsed
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

    /// Represents an error caused by the request sending an unrecognized `method` field
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

    /// Represents an error caused by some operation internal to the server
    pub fn internal_error(id: Value) -> Self {
        JsonRpcResponse {
            id: Some(id),
            jsonrpc: "2.0".to_string(),
            error: Some(JsonRpcError {
                code: -32603,
                message: "Internal error".to_string(),
                data: None,
            }),
            result: None,
        }
    }
}

impl IntoResponse for JsonRpcResponse {
    fn into_response(self) -> Response {
        if self.error.is_some() {
            // TO-DO: more accurate response codes depending on error type
            return (
                StatusCode::BAD_REQUEST,
                serde_json::ser::to_vec(&self).unwrap(),
            )
                .into_response();
        }

        (StatusCode::OK, serde_json::ser::to_vec(&self).unwrap()).into_response()
    }
}

/// Defines server routes, assigns them handlers, and runs the RPC server
pub async fn serve(address: &SocketAddr) -> Result<(), hyper::Error> {
    let app = Router::new()
        .route("/", get(root_rpc_handler).post(root_rpc_handler))
        .route("/broadcast_tx_commit", post(rpc_broadcast_tx_commit))
        .route("/status", get(rpc_status).post(rpc_status))
        .layer(TraceLayer::new_for_http());

    info!("listening at {}", &address);
    axum::Server::bind(address)
        .serve(app.into_make_service())
        .await
}

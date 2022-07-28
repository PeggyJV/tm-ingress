// use std::net::SocketAddr;

// use abscissa_core::{tracing::log::info, Application};
// use cosmrs::proto::cosmos::tx::v1beta1::{service_client::ServiceClient, service_server::{self, ServiceServer}, GetTxResponse, GetTxRequest, BroadcastTxResponse, BroadcastTxRequest, GetTxsEventResponse, GetTxsEventRequest, GetBlockWithTxsRequest, GetBlockWithTxsResponse, SimulateRequest, SimulateResponse};
// use tonic::{async_trait, Request, Response, Status};

// use crate::application::APP;

// /// Handler for the tx server messages
// pub struct ServiceHandler;

// #[async_trait]
// impl service_server::Service for ServiceHandler {
//     async fn simulate(&self, _request: Request<SimulateRequest>) ->  Result<Response<SimulateResponse> ,Status>
//     {
//         todo!()
//     }

//     async fn get_tx(&self, _request: Request<GetTxRequest> ,) -> Result<Response<GetTxResponse> ,Status> {
//         todo!()
//     }

//     async fn broadcast_tx(&self, request: Request<BroadcastTxRequest>) ->  Result<Response<BroadcastTxResponse> ,Status> {
//         let config = APP.config();
//         let mut client = match ServiceClient::connect(config.node.rpc.clone()).await {
//             Ok(c) => c,
//             Err(err) => return Err(Status::unavailable(err.to_string())),
//         };

//         info!("forwarding request!");

//         let response = client.broadcast_tx(request).await?;

//         Ok(response)
//     }

//     async fn get_txs_event(&self, _request: Request<GetTxsEventRequest>) ->  Result<Response<GetTxsEventResponse>, Status> {
//         todo!()
//     }

//     async fn get_block_with_txs(&self, _request: Request<GetBlockWithTxsRequest>) ->  Result<Response<GetBlockWithTxsResponse> ,Status> {
//         todo!()
//     }
// }

// /// Runs the tx server
// #[allow(dead_code)]
// pub async fn relay_grpc(address: &SocketAddr) -> Result<(), tonic::transport::Error> {
//     info!("listening on {}", address);
//     tonic::transport::Server::builder()
//         .add_service(ServiceServer::new(ServiceHandler))
//         // .add_service(ReflectionServiceServer::new(ReflectionHandler))
//         .serve(*address)
//         .await
// }

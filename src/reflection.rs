use cosmos_sdk_proto::cosmos::base::reflection::v2alpha1::{reflection_service_server::{self, ReflectionService}, GetQueryServicesDescriptorRequest, GetTxDescriptorRequest, GetConfigurationDescriptorRequest, GetCodecDescriptorRequest, GetChainDescriptorRequest, GetAuthnDescriptorRequest, GetAuthnDescriptorResponse, GetChainDescriptorResponse, GetCodecDescriptorResponse, GetConfigurationDescriptorResponse, GetQueryServicesDescriptorResponse, GetTxDescriptorResponse};
use tonic::{async_trait, Status, Request, Response};

pub struct ReflectionHandler;

#[async_trait]
impl ReflectionService for ReflectionHandler {
    async fn get_authn_descriptor(&self, _request: Request<GetAuthnDescriptorRequest>) ->  Result<Response<GetAuthnDescriptorResponse>, Status> {
        todo!()
    }

    async fn get_chain_descriptor(&self, _request: Request<GetChainDescriptorRequest>) ->  Result<Response<GetChainDescriptorResponse>, Status> {
        todo!()
    }

    async fn get_codec_descriptor(&self, _request: Request<GetCodecDescriptorRequest>) ->  Result<Response<GetCodecDescriptorResponse>, Status> {
        todo!()
    }

    async fn get_configuration_descriptor(&self, _request: Request<GetConfigurationDescriptorRequest>) ->  Result<Response<GetConfigurationDescriptorResponse>, Status> {
        todo!()
    }

    async fn get_query_services_descriptor(&self, _request: Request<GetQueryServicesDescriptorRequest>) ->  Result<Response<GetQueryServicesDescriptorResponse>, Status> {
        todo!()
    }

    async fn get_tx_descriptor(&self, _request: Request<GetTxDescriptorRequest>) ->  Result<Response<GetTxDescriptorResponse>, Status> {
        todo!()
    }
}

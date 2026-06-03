// src/edge-api-rs/src/grpc/edge_service.rs
use summer::plugin::service::Service;
use tonic::{Request, Response, Status};

use crate::grpc::edgeapi::{
    edge_api_server::{EdgeApi, EdgeApiServer},
    HealthRequest,
    HealthResponse,
};

#[derive(Clone, Service)]
#[service(grpc = "EdgeApiServer")]
pub struct EdgeApiGrpcService;

#[tonic::async_trait]
impl EdgeApi for EdgeApiGrpcService {
    async fn health(
        &self,
        _request: Request<HealthRequest>,
    ) -> Result<Response<HealthResponse>, Status> {
        Ok(Response::new(HealthResponse {
            status: "ok".to_string(),
        }))
    }
}
mod proto_registry_main_server {
    tonic::include_proto!("registry_main_server");
}

use cloud_api::error::RegistryError;
use proto_registry_main_server::{
    registry_data_node_service_server::{RegistryDataNodeService, RegistryDataNodeServiceServer},
    RegistryRequest, RegistryResponse,
};
use tonic::{Request, Response, Status};

pub struct RegistryController {}

impl RegistryController {
    pub async fn get_service() -> RegistryDataNodeServiceServer<Self> {
        RegistryDataNodeServiceServer::new(Self {})
    }
}

#[tonic::async_trait]
impl RegistryDataNodeService for RegistryController {
    async fn registry(
        &self,
        request: Request<RegistryRequest>,
    ) -> Result<Response<RegistryResponse>, Status> {
        let request = request.into_inner();

        if request.block_size != 32 {
            return Err(RegistryError::WrongBlockSize(request.block_size as usize, 32, 64).into());
        }

        Ok(Response::new(RegistryResponse {}))
    }
}

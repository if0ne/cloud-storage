pub mod proto_main_server {
    tonic::include_proto!("main_server");
}

use proto_main_server::{
    namespace_service_server::{NamespaceService, NamespaceServiceServer},
    CreateFileRequest, CreateSmallFileResponse, MakeDirRequest, MakeDirResponse, NewBlockRequest,
    NewBlockResponse,
};
use std::path::Path;
use tonic::{Request, Response, Status};

use super::namespace_service::NamespaceServiceImpl;

pub struct NamespaceController {
    namespace_service: NamespaceServiceImpl,
}

impl NamespaceController {
    pub async fn get_service() -> NamespaceServiceServer<Self> {
        NamespaceServiceServer::new(Self {
            namespace_service: NamespaceServiceImpl::new(),
        })
    }
}

#[tonic::async_trait]
impl NamespaceService for NamespaceController {
    async fn make_dir(
        &self,
        request: Request<MakeDirRequest>,
    ) -> Result<Response<MakeDirResponse>, Status> {
        Ok(Response::new(MakeDirResponse {}))
    }

    async fn create_small_file(
        &self,
        request: Request<CreateFileRequest>,
    ) -> Result<Response<CreateSmallFileResponse>, Status> {
        let path = request.into_inner().path;
        let path = Path::new(&path);
        self.namespace_service.create_small_file(path).await;
        Ok(Response::new(CreateSmallFileResponse {}))
    }

    async fn new_small_block(
        &self,
        request: Request<NewBlockRequest>,
    ) -> Result<Response<NewBlockResponse>, Status> {
        Ok(Response::new(NewBlockResponse {
            block_id: vec![0, 0, 0, 0],
        }))
    }
}

use proto_main_server::{
    namespace_service_server::{NamespaceService, NamespaceServiceServer},
    MakeDirRequest, MakeDirResponse, CreateFileRequest, CreateSmallFileResponse,
    NewBlockRequest, NewBlockResponse
};
use tonic::{Request, Response, Status};

pub mod proto_main_server {
    tonic::include_proto!("main_server");
}

pub struct NamespaceController {

}

impl NamespaceController {
    pub async fn get_service() -> NamespaceServiceServer<Self> {
        NamespaceServiceServer::new(Self {

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
        Ok(Response::new(CreateSmallFileResponse {}))
    }

    async fn new_small_block(
        &self,
        request: Request<NewBlockRequest>,
    ) -> Result<Response<NewBlockResponse>, Status> {
        Ok(Response::new(NewBlockResponse {
            block_id: vec![0, 0, 0, 0]
        }))
    }
}

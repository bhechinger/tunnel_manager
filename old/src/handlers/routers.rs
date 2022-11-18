use crate::api::router_server::Router;
use crate::api::RouterListResponse;
use crate::api::{RouterAddRequest, RouterAddResponse};
use crate::api::{RouterData, RouterResponse};
use crate::api::{RouterDeleteRequest, RouterDeleteResponse};
use crate::api::{RouterGetRequest, RouterGetResponse};
use crate::api::{RouterUpdateRequest, RouterUpdateResponse};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct RouterService {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl RouterService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl Router for RouterService {
    async fn list(
        &self,
        request: Request<()>, // Accept request of type HelloRequest
    ) -> Result<Response<RouterListResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a list request: {:?}", request);

        let reply = RouterListResponse {
            response: Some(RouterResponse {
                error: false,
                message: format!("Router list"), // We must use .into_inner() as the fields of gRPC requests and responses are private
            }),
            routers: vec![
                RouterData {
                    id: 1,
                    agent: 2,
                    snmp_community: "letmein".to_string(),
                    ssh_username: "root".to_string(),
                    ssh_password: "password!".to_string(),
                    conn_type: "GRE".to_string(),
                    router_type: "HUB".to_string(),
                },
                RouterData {
                    id: 2,
                    agent: 2,
                    snmp_community: "letmein".to_string(),
                    ssh_username: "root".to_string(),
                    ssh_password: "password!".to_string(),
                    conn_type: "GRE".to_string(),
                    router_type: "HUB".to_string(),
                },
            ],
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn get(
        &self,
        request: Request<RouterGetRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<RouterGetResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a get request: {:?}", request);

        let reply = RouterGetResponse {
            response: Some(RouterResponse {
                error: false,
                message: format!("Router get {}!", request.into_inner().id).into(),
            }),
            router: Some(RouterData {
                id: 1,
                agent: 2,
                snmp_community: "letmein".to_string(),
                ssh_username: "root".to_string(),
                ssh_password: "password!".to_string(),
                conn_type: "GRE".to_string(),
                router_type: "HUB".to_string(),
            }),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn add(
        &self,
        request: Request<RouterAddRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<RouterAddResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got am add request: {:?}", request);

        let router = request.into_inner().router.unwrap_or_default();

        let reply = RouterAddResponse {
            response: Some(RouterResponse {
                error: false,
                message: format!("Router add {} {}!", router.id, router.agent).into(),
            }),
            id: 2,
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn delete(
        &self,
        request: Request<RouterDeleteRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<RouterDeleteResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a delete request: {:?}", request);

        let reply = RouterDeleteResponse {
            response: Some(RouterResponse {
                error: false,
                message: format!("Router delete {}!", request.into_inner().id).into(),
            }),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn update(
        &self,
        request: Request<RouterUpdateRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<RouterUpdateResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got an update request: {:?}", request);

        let router = request.into_inner().router.unwrap_or_default();

        let reply = RouterUpdateResponse {
            response: Some(RouterResponse {
                error: false,
                message: format!("Router update {}!", router.id).into(),
            }),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

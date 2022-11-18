use crate::api::tunnel_server::Tunnel;
use crate::api::TunnelListResponse;
use crate::api::{TunnelAddRequest, TunnelAddResponse};
use crate::api::{TunnelData, TunnelResponse};
use crate::api::{TunnelDeleteRequest, TunnelDeleteResponse};
use crate::api::{TunnelGetRequest, TunnelGetResponse};
use crate::api::{TunnelUpdateRequest, TunnelUpdateResponse};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct TunnelService {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl TunnelService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl Tunnel for TunnelService {
    async fn list(
        &self,
        request: Request<()>, // Accept request of type HelloRequest
    ) -> Result<Response<TunnelListResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a list request: {:?}", request);

        let reply = TunnelListResponse {
            response: Some(TunnelResponse {
                error: false,
                message: format!("Tunnel list"), // We must use .into_inner() as the fields of gRPC requests and responses are private
            }),
            tunnels: vec![
                TunnelData {
                    id: 1,
                    version: 2,
                    router: 3,
                    ip: "10.0.0.1".to_string(),
                    dynamic_ip: false,
                    ip_class: "ipv4".to_string(),
                    hostname: "host.name".to_string(),
                    description: "fake router".to_string(),
                    source: "int0/0".to_string(),
                    cost: 4,
                    tunnel_type: "GRE".to_string(),
                    topology_type: "HUB".to_string(),
                },
                TunnelData {
                    id: 2,
                    version: 2,
                    router: 4,
                    ip: "10.0.0.2".to_string(),
                    dynamic_ip: false,
                    ip_class: "ipv4".to_string(),
                    hostname: "host2.name".to_string(),
                    description: "fake router".to_string(),
                    source: "int0/0".to_string(),
                    cost: 4,
                    tunnel_type: "GRE".to_string(),
                    topology_type: "HUB".to_string(),
                },
            ],
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn get(
        &self,
        request: Request<TunnelGetRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<TunnelGetResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a get request: {:?}", request);

        let reply = TunnelGetResponse {
            response: Some(TunnelResponse {
                error: false,
                message: format!("Tunnel get {}!", request.into_inner().id).into(),
            }),
            tunnel: Some(TunnelData {
                id: 1,
                version: 2,
                router: 3,
                ip: "10.0.0.1".to_string(),
                dynamic_ip: false,
                ip_class: "ipv4".to_string(),
                hostname: "host.name".to_string(),
                description: "fake router".to_string(),
                source: "int0/0".to_string(),
                cost: 4,
                tunnel_type: "GRE".to_string(),
                topology_type: "HUB".to_string(),
            }),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn add(
        &self,
        request: Request<TunnelAddRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<TunnelAddResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got am add request: {:?}", request);

        let tunnel = request.into_inner().tunnel.unwrap_or_default();

        let reply = TunnelAddResponse {
            response: Some(TunnelResponse {
                error: false,
                message: format!("Tunnel add {} {}!", tunnel.id, tunnel.ip).into(),
            }),
            id: 1,
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn delete(
        &self,
        request: Request<TunnelDeleteRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<TunnelDeleteResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a delete request: {:?}", request);

        let reply = TunnelDeleteResponse {
            response: Some(TunnelResponse {
                error: false,
                message: format!("Tunnel delete {}!", request.into_inner().id).into(),
            }),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn update(
        &self,
        request: Request<TunnelUpdateRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<TunnelUpdateResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got an update request: {:?}", request);

        let tunnel = request.into_inner().tunnel.unwrap_or_default();

        let reply = TunnelUpdateResponse {
            response: Some(TunnelResponse {
                error: false,
                message: format!("Tunnel update {}!", tunnel.id).into(),
            }),
            version: 3,
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

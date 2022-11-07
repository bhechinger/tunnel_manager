use tonic::{Request, Response, Status};
use crate::api::agent_server::Agent;
use crate::api::{AgentListResponse};
use crate::api::{AgentGetRequest, AgentGetResponse};
use crate::api::{AgentAddRequest, AgentAddResponse};
use crate::api::{AgentDeleteRequest, AgentDeleteResponse};
use crate::api::{AgentUpdateRequest, AgentUpdateResponse};
use sqlx::postgres::PgPool;
use crate::api::{AgentResponse, AgentData};

#[derive(Debug, Default)]
pub struct AgentService {
    pool: PgPool
}

impl AgentService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }
}

#[tonic::async_trait]
impl Agent for AgentService {
    async fn list(
        &self,
        request: Request<()>, // Accept request of type HelloRequest
    ) -> Result<Response<AgentListResponse>, Status> { // Return an instance of type HelloReply
        println!("Got a list request: {:?}", request);

        let reply = AgentListResponse {
            response: Some(AgentResponse {
                error: false,
                message: format!("Agent list"), // We must use .into_inner() as the fields of gRPC requests and responses are private
            }),
            agents: vec![
                AgentData {
                    uuid: "fake-uuid-1".to_string(),
                    owner: 1,
                },
                AgentData {
                    uuid: "fake-uuid-2".to_string(),
                    owner: 2,
                },
            ],
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn get(
        &self,
        request: Request<AgentGetRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<AgentGetResponse>, Status> { // Return an instance of type HelloReply
        println!("Got a get request: {:?}", request);

        let reply = AgentGetResponse {
            response: Some(AgentResponse {
                error: false,
                message: format!("Agent get {}!", request.into_inner().uuid).into(),
            }),
            agent: Some(AgentData {
                uuid: "fake-uuid-1".to_string(),
                owner: 1,
            }),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn add(
        &self,
        request: Request<AgentAddRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<AgentAddResponse>, Status> { // Return an instance of type HelloReply
        println!("Got am add request: {:?}", request);

        let agent = request.into_inner().agent.unwrap_or_default();

        let reply = AgentAddResponse {
            response: Some(AgentResponse {
                error: false,
                message: format!("Agent add {} {}!", agent.uuid, agent.owner).into(),
            }),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn delete(
        &self,
        request: Request<AgentDeleteRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<AgentDeleteResponse>, Status> { // Return an instance of type HelloReply
        println!("Got a delete request: {:?}", request);

        let reply = AgentDeleteResponse {
            response: Some(AgentResponse {
                error: false,
                message: format!("Agent delete {}!", request.into_inner().uuid).into(),
            }),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn update(
        &self,
        request: Request<AgentUpdateRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<AgentUpdateResponse>, Status> { // Return an instance of type HelloReply
        println!("Got an update request: {:?}", request);

        let agent = request.into_inner().agent.unwrap_or_default();

        let reply = AgentUpdateResponse {
            response: Some(AgentResponse {
                error: false,
                message: format!("Agent update {}!", agent.uuid).into(),
            }),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}
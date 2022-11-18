use crate::api::agent_get_request::UuidOrOwner;
use crate::api::agent_server::Agent;
use crate::api::{AgentData, AgentDeleteRequest, AgentGetRequest, AgentsData};
use crate::storage::agents::Agents;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct AgentService {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl AgentService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl Agent for AgentService {
    async fn list(
        &self,
        request: Request<()>, // Accept request of type HelloRequest
    ) -> Result<Response<AgentsData>, Status> {
        // Return an instance of type HelloReply
        println!("Got a list request: {:?}", request);

        let agents = Agents::all(&self.pool).await.unwrap();

        Ok(Response::new(AgentsData { agents }))
    }

    async fn get(
        &self,
        request: Request<AgentGetRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<AgentsData>, Status> {
        // Return an instance of type HelloReply
        println!("Got a get request: {:?}", request);

        let req = request.into_inner();

        match req.uuid_or_owner {
            Some(UuidOrOwner::Uuid(uuid)) => Ok(Response::new(AgentsData {
                agents: Agents::get_by_uuid(&self.pool, uuid).await.unwrap(),
            })),
            Some(UuidOrOwner::Owner(owner)) => Ok(Response::new(AgentsData {
                agents: Agents::get_by_owner(&self.pool, owner).await.unwrap(),
            })),
            None => Err(Status::invalid_argument("UUID or owner required")),
        }
    }

    async fn add(
        &self,
        request: Request<AgentData>, // Accept request of type HelloRequest
    ) -> Result<Response<AgentData>, Status> {
        // Return an instance of type HelloReply
        println!("Got an add request: {:?}", request);
        let req = request.into_inner();
        let agent = Agents::add(&self.pool, req.uuid, req.owner).await.unwrap();

        Ok(Response::new(agent))
    }

    async fn delete(
        &self,
        request: Request<AgentDeleteRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<AgentData>, Status> {
        // Return an instance of type HelloReply
        println!("Got a delete request: {:?}", request);

        let agent = Agents::delete(&self.pool, request.into_inner().uuid)
            .await
            .unwrap();

        Ok(Response::new(agent))
    }

    async fn update(
        &self,
        request: Request<AgentData>, // Accept request of type HelloRequest
    ) -> Result<Response<AgentData>, Status> {
        // Return an instance of type HelloReply
        println!("Got an update request: {:?}", request);

        let req = request.into_inner();

        let agent = Agents::update(&self.pool, req.uuid, req.owner)
            .await
            .unwrap();

        Ok(Response::new(agent))
    }
}

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

use crate::api::{AgentData, AgentRequest, AgentsData};
use crate::api::agent_server::Agent;
use crate::storage::agents;

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
    #[instrument]
    async fn list(&self, request: Request<()>) -> Result<Response<AgentsData>, Status> {
        info!(message = "Got a list request", ?request);

        match agents::Agent::all(&self.pool).await {
            Ok(result) => Ok(Response::new(AgentsData { agents: result })),
            Err(status) => {
                error!(
                    message = "Error getting list of agents",
                    status = status.message()
                );
                return Err(status);
            }
        }
    }

    #[instrument]
    async fn get(&self, request: Request<AgentRequest>) -> Result<Response<AgentsData>, Status> {
        info!(message = "Got a get request", ?request);

        let req = request.into_inner();

        match req.id_uuid_or_owner {
            Some(id_uuid_or_owner) => match agents::Agent::get(&self.pool, &id_uuid_or_owner).await {
                Ok(result) => Ok(Response::new(AgentsData { agents: result })),
                Err(status) => {
                    error!(
                        message = "Error getting agent",
                        status = status.message()
                    );
                    return Err(status);
                }
            },
            None => Err(Status::invalid_argument("Agent id, uuid or owner required")),
        }
    }

    #[instrument]
    async fn add(&self, request: Request<AgentData>) -> Result<Response<AgentData>, Status> {
        info!(message = "Got an add request", ?request);

        let req = request.into_inner();

        if req.uuid.is_empty() {
            return Err(Status::invalid_argument("uuid is required"));
        }

        if req.owner == 0 {
            return Err(Status::invalid_argument("owner is required"));
        }

        match agents::Agent::add(&self.pool, req).await {
            Ok(result) => Ok(Response::new(result)),
            Err(status) => {
                error!(message = "Error adding agent", status = status.message());
                return Err(status);
            }
        }
    }

    #[instrument]
    async fn delete(&self, request: Request<AgentRequest>) -> Result<Response<AgentData>, Status> {
        info!(message = "Got a delete request", ?request);

        let req = request.into_inner();

        match req.id_uuid_or_owner {
            Some(id_uuid_or_owner) => match agents::Agent::delete(&self.pool, id_uuid_or_owner).await {
                Ok(_) => Ok(Response::new(AgentData::default())),
                Err(status) => {
                    error!(
                        message = "Error deleting agent",
                        status = status.message()
                    );
                    return Err(status);
                }
            },
            None => Err(Status::invalid_argument("Agent id or email required")),
        }
    }

    #[instrument]
    async fn update(&self, request: Request<AgentData>) -> Result<Response<AgentData>, Status> {
        info!(message = "Got an update request", ?request);

        let req = request.into_inner();
        if req.id.is_none() {
            return Err(Status::invalid_argument("Agent id required"));
        }

        match agents::Agent::update(&self.pool, req).await {
            Ok(result) => Ok(Response::new(result)),
            Err(status) => {
                error!(
                    message = "Error updating agent",
                    status = status.message()
                );
                return Err(status);
            }
        }
    }
}

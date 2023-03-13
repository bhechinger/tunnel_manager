use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

use crate::api::{RouterAddRequest, RouterRequest, RouterResponse, RoutersResponse, RouterUpdateRequest};
use crate::api::router_server::Router;
use crate::storage::routers;

#[derive(Debug)]
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
    #[instrument]
    async fn list(&self, request: Request<()>) -> Result<Response<RoutersResponse>, Status> {
        info!(message = "Got a list request", ?request);

        match routers::Router::all(&self.pool).await {
            Ok(result) => Ok(Response::new(RoutersResponse { routers: result })),
            Err(status) => {
                error!(
                    message = "Error getting list of routers",
                    status = status.message()
                );
                return Err(status);
            }
        }
    }

    #[instrument]
    async fn get(&self, request: Request<RouterRequest>) -> Result<Response<RouterResponse>, Status> {
        info!(message = "Got a get request", ?request);

        let req = request.into_inner();

        match req.id_or_agent {
            Some(id_or_agent) => match routers::Router::get(&self.pool, &id_or_agent).await {
                Ok(result) => Ok(Response::new(result)),
                Err(status) => {
                    error!(
                        message = "Error getting router",
                        status = status.message()
                    );
                    return Err(status);
                }
            },
            None => Err(Status::invalid_argument("Router id or agent required")),
        }
    }

    #[instrument]
    async fn add(&self, request: Request<RouterAddRequest>) -> Result<Response<RouterResponse>, Status> {
        info!(message = "Got an add request", ?request);

        let req = request.into_inner();

        if req.agent == 0 {
            return Err(Status::invalid_argument("agent is required"));
        }

        match routers::Router::add(&self.pool, req).await {
            Ok(result) => Ok(Response::new(result)),
            Err(status) => {
                error!(message = "Error adding router", status = status.message());
                return Err(status);
            }
        }
    }

    #[instrument]
    async fn delete(&self, request: Request<RouterRequest>) -> Result<Response<RouterResponse>, Status> {
        info!(message = "Got a delete request", ?request);

        let req = request.into_inner();

        match req.id_or_agent {
            Some(id_or_agent) => match routers::Router::delete(&self.pool, id_or_agent).await {
                Ok(_) => Ok(Response::new(RouterResponse::default())),
                Err(status) => {
                    error!(
                        message = "Error deleting router",
                        status = status.message()
                    );
                    return Err(status);
                }
            },
            None => Err(Status::invalid_argument("Router id or agent required")),
        }
    }

    #[instrument]
    async fn update(&self, request: Request<RouterUpdateRequest>) -> Result<Response<RouterResponse>, Status> {
        info!(message = "Got an update request", ?request);

        let req = request.into_inner();
        if req.id == 0 {
            return Err(Status::invalid_argument("Router id required"));
        }

        match routers::Router::update(&self.pool, req).await {
            Ok(result) => Ok(Response::new(result)),
            Err(status) => {
                error!(
                    message = "Error updating router",
                    status = status.message()
                );
                return Err(status);
            }
        }
    }
}

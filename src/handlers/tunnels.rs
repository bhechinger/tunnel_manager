use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

use crate::api::{TunnelAddRequest, TunnelRequest, TunnelResponse, TunnelsResponse, TunnelUpdateRequest};
use crate::api::tunnel_server::Tunnel;
use crate::storage::tunnels;

#[derive(Debug)]
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
    #[instrument]
    async fn list(&self, request: Request<()>) -> Result<Response<TunnelsResponse>, Status> {
        info!(message = "Got a list request", ?request);

        match tunnels::Tunnel::all(&self.pool).await {
            Ok(result) => Ok(Response::new(TunnelsResponse { tunnels: result })),
            Err(status) => {
                error!(
                    message = "Error getting list of tunnels",
                    status = status.message()
                );
                return Err(status);
            }
        }
    }

    #[instrument]
    async fn get(&self, request: Request<TunnelRequest>) -> Result<Response<TunnelResponse>, Status> {
        info!(message = "Got a get request", ?request);

        let req = request.into_inner();

        match req.id_or_router {
            Some(id_or_router) => match tunnels::Tunnel::get(&self.pool, &id_or_router).await {
                Ok(result) => Ok(Response::new(result)),
                Err(status) => {
                    error!(
                        message = "Error getting tunnel",
                        status = status.message()
                    );
                    return Err(status);
                }
            },
            None => Err(Status::invalid_argument("Tunnel id, uuid or owner required")),
        }
    }

    #[instrument]
    async fn add(&self, request: Request<TunnelAddRequest>) -> Result<Response<TunnelResponse>, Status> {
        info!(message = "Got an add request", ?request);

        let req = request.into_inner();

        if req.router == 0 {
            return Err(Status::invalid_argument("router is required"));
        }

        if req.ip.is_empty() {
            return Err(Status::invalid_argument("ip is required"));
        }

        if req.hostname.is_empty() {
            return Err(Status::invalid_argument("hostname is required"));
        }

        if req.description.is_empty() {
            return Err(Status::invalid_argument("description is required"));
        }

        if req.source.is_empty() {
            return Err(Status::invalid_argument("source is required"));
        }

        match tunnels::Tunnel::add(&self.pool, req).await {
            Ok(result) => Ok(Response::new(result)),
            Err(status) => {
                error!(message = "Error adding tunnel", status = status.message());
                return Err(status);
            }
        }
    }

    #[instrument]
    async fn delete(&self, request: Request<TunnelRequest>) -> Result<Response<TunnelResponse>, Status> {
        info!(message = "Got a delete request", ?request);

        // let req = request.into_inner();

        match request.into_inner().id_or_router {
            Some(id_or_router) => match tunnels::Tunnel::delete(&self.pool, id_or_router).await {
                Ok(_) => Ok(Response::new(TunnelResponse::default())),
                Err(status) => {
                    error!(
                        message = "Error deleting tunnel",
                        status = status.message()
                    );
                    return Err(status);
                }
            },
            None => Err(Status::invalid_argument("Tunnel id or email required")),
        }
    }

    #[instrument]
    async fn update(&self, request: Request<TunnelUpdateRequest>) -> Result<Response<TunnelResponse>, Status> {
        info!(message = "Got an update request", ?request);

        match tunnels::Tunnel::update(&self.pool, request.into_inner()).await {
            Ok(result) => Ok(Response::new(result)),
            Err(status) => {
                error!(
                    message = "Error updating tunnel",
                    status = status.message()
                );
                return Err(status);
            }
        }
    }
}

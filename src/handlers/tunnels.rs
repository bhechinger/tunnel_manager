use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

use crate::api::{TunnelData, TunnelRequest, TunnelsData};
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
    async fn list(&self, request: Request<()>) -> Result<Response<TunnelsData>, Status> {
        info!(message = "Got a list request", ?request);

        match tunnels::Tunnel::all(&self.pool).await {
            Ok(result) => Ok(Response::new(TunnelsData { tunnels: result })),
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
    async fn get(&self, request: Request<TunnelRequest>) -> Result<Response<TunnelsData>, Status> {
        info!(message = "Got a get request", ?request);

        let req = request.into_inner();

        match req.it_or_router {
            Some(it_or_router) => match tunnels::Tunnel::get(&self.pool, &it_or_router).await {
                Ok(result) => Ok(Response::new(TunnelsData { tunnels: result })),
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
    async fn add(&self, request: Request<TunnelData>) -> Result<Response<TunnelData>, Status> {
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
    async fn delete(&self, request: Request<TunnelRequest>) -> Result<Response<TunnelData>, Status> {
        info!(message = "Got a delete request", ?request);

        let req = request.into_inner();

        match req.it_or_router {
            Some(it_or_router) => match tunnels::Tunnel::delete(&self.pool, it_or_router).await {
                Ok(_) => Ok(Response::new(TunnelData::default())),
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
    async fn update(&self, request: Request<TunnelData>) -> Result<Response<TunnelData>, Status> {
        info!(message = "Got an update request", ?request);

        let req = request.into_inner();
        if req.id.is_none() {
            return Err(Status::invalid_argument("Tunnel id required"));
        }

        match tunnels::Tunnel::update(&self.pool, req).await {
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

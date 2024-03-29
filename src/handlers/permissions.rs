use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

use crate::api::{PermissionData, PermissionRequest, PermissionsData};
use crate::api::permission_server::Permission;
use crate::storage::permissions;

#[derive(Debug)]
pub struct PermissionService {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PermissionService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl Permission for PermissionService {
    #[instrument]
    async fn list(&self, request: Request<()>) -> Result<Response<PermissionsData>, Status> {
        info!(message = "Got a list request", ?request);

        match permissions::Permission::all(&self.pool).await {
            Ok(result) => Ok(Response::new(PermissionsData { permissions: result })),
            Err(status) => {
                error!(
                    message = "Error getting list of permissions",
                    status = status.message()
                );
                return Err(status);
            }
        }
    }

    #[instrument]
    async fn get(&self, request: Request<PermissionRequest>) -> Result<Response<PermissionData>, Status> {
        info!(message = "Got a get request", ?request);

        let req = request.into_inner();

        match req.id_or_name {
            Some(id_or_name) => match permissions::Permission::get(&self.pool, &id_or_name).await {
                Ok(result) => Ok(Response::new(result)),
                Err(status) => {
                    error!(
                        message = "Error getting permission",
                        status = status.message()
                    );
                    return Err(status);
                }
            },
            None => Err(Status::invalid_argument("Permission id or name required")),
        }
    }

    #[instrument]
    async fn add(&self, request: Request<PermissionData>) -> Result<Response<PermissionData>, Status> {
        info!(message = "Got an add request", ?request);

        let req = request.into_inner();

        if req.name.is_empty() {
            return Err(Status::invalid_argument("Permission name is required"));
        }

        if req.description.is_empty() {
            return Err(Status::invalid_argument("Permission description is required"));
        }

        match permissions::Permission::add(&self.pool, req).await {
            Ok(result) => Ok(Response::new(result)),
            Err(status) => {
                error!(message = "Error adding permission", status = status.message());
                return Err(status);
            }
        }
    }

    #[instrument]
    async fn delete(&self, request: Request<PermissionRequest>) -> Result<Response<PermissionData>, Status> {
        info!(message = "Got a delete request", ?request);

        let req = request.into_inner();

        match req.id_or_name {
            Some(id_or_name) => match permissions::Permission::delete(&self.pool, id_or_name).await {
                Ok(_) => Ok(Response::new(PermissionData::default())),
                Err(status) => {
                    error!(
                        message = "Error deleting permission",
                        status = status.message()
                    );
                    return Err(status);
                }
            },
            None => Err(Status::invalid_argument("Permission id or name required")),
        }
    }

    #[instrument]
    async fn update(&self, request: Request<PermissionData>) -> Result<Response<PermissionData>, Status> {
        info!(message = "Got an update request", ?request);

        let req = request.into_inner();

        match permissions::Permission::update(&self.pool, req).await {
            Ok(result) => Ok(Response::new(result)),
            Err(status) => {
                error!(
                    message = "Error deleting permission by name",
                    status = status.message()
                );
                return Err(status);
            }
        }
    }
}

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

use crate::api::{PermissionAddRequest, PermissionData, PermissionRequest, PermissionsData};
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
                        message = "Error getting permission by id",
                        status = status.message()
                    );
                    return Err(status);
                }
            },
            None => Err(Status::invalid_argument("Permission id or name required")),
        }
    }

    #[instrument]
    async fn add(&self, request: Request<PermissionAddRequest>) -> Result<Response<PermissionData>, Status> {
        info!(message = "Got an add request", ?request);

        let req = request.into_inner();

        match permissions::Permission::add(&self.pool, req.name.as_str(), req.description.as_str()).await {
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
                Ok(_) => Ok(Response::new(PermissionData { id: 0, name: "".to_string(), description: "".to_string() })), // I don't love this
                Err(status) => {
                    error!(
                        message = "Error deleting permission by id",
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

        match permissions::Permission::update(&self.pool, permissiondata_to_data(&req)).await {
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

fn permissiondata_to_data(permission_data: &PermissionData) -> permissions::Permission {
    permissions::Permission {
        id: permission_data.id,
        name: permission_data.name.clone(),
        description: permission_data.description.clone(),
    }
}
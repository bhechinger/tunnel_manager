use sqlx::postgres::PgPool;
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

use crate::api::permission_request::IdOrName;
use crate::api::permission_server::Permission;
use crate::api::{PermissionAddRequest, PermissionData, PermissionRequest, PermissionsData};
use crate::models::permissions::Permissions;

#[derive(Debug)]
pub struct PermissionService {
    pool: PgPool,
}

impl PermissionService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl Permission for PermissionService {
    #[instrument]
    async fn list(&self, request: Request<()>) -> Result<Response<PermissionsData>, Status> {
        info!(message = "Got a list request", ?request);

        match Permissions::all(&self.pool).await {
            Ok(result) => Ok(Response::new(PermissionsData {
                permissions: result,
            })),
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
    async fn get(
        &self,
        request: Request<PermissionRequest>,
    ) -> Result<Response<PermissionData>, Status> {
        info!(message = "Got a get request", ?request);

        let req = request.into_inner();

        match req.id_or_name {
            Some(IdOrName::Id(id)) => match Permissions::get_by_id(&self.pool, id).await {
                Ok(result) => Ok(Response::new(result)),
                Err(status) => {
                    error!(
                        message = "Error getting permission by id",
                        status = status.message()
                    );
                    return Err(status);
                }
            },
            Some(IdOrName::Name(name)) => match Permissions::get_by_name(&self.pool, name).await {
                Ok(result) => Ok(Response::new(result)),
                Err(status) => {
                    error!(
                        message = "Error getting permission by email",
                        status = status.message()
                    );
                    return Err(status);
                }
            },
            None => Err(Status::invalid_argument("Permission id or name required")),
        }
    }

    #[instrument]
    async fn add(
        &self,
        request: Request<PermissionAddRequest>,
    ) -> Result<Response<PermissionData>, Status> {
        info!(message = "Got an add request", ?request);

        let req = request.into_inner();

        match Permissions::add(&self.pool, req.name, req.description).await {
            Ok(result) => Ok(Response::new(result)),
            Err(status) => {
                error!(
                    message = "Error adding permission",
                    status = status.message()
                );
                return Err(status);
            }
        }
    }

    #[instrument]
    async fn delete(
        &self,
        request: Request<PermissionRequest>,
    ) -> Result<Response<PermissionData>, Status> {
        info!(message = "Got a delete request", ?request);

        let req = request.into_inner();

        match req.id_or_name {
            Some(IdOrName::Id(id)) => match Permissions::delete_by_id(&self.pool, id).await {
                Ok(result) => Ok(Response::new(result)),
                Err(status) => {
                    error!(
                        message = "Error deleting permission by id",
                        status = status.message()
                    );
                    return Err(status);
                }
            },
            Some(IdOrName::Name(name)) => {
                match Permissions::delete_by_name(&self.pool, name).await {
                    Ok(result) => Ok(Response::new(result)),
                    Err(status) => {
                        error!(
                            message = "Error deleting permission by email",
                            status = status.message()
                        );
                        return Err(status);
                    }
                }
            }
            None => Err(Status::invalid_argument("Permission id or email required")),
        }
    }

    #[instrument]
    async fn update(
        &self,
        request: Request<PermissionData>,
    ) -> Result<Response<PermissionData>, Status> {
        info!(message = "Got an update request", ?request);

        let req = request.into_inner();

        match Permissions::update(&self.pool, req.id, req.name, req.description).await {
            Ok(result) => Ok(Response::new(result)),
            Err(status) => {
                error!(
                    message = "Error updating permission",
                    status = status.message()
                );
                return Err(status);
            }
        }
    }
}

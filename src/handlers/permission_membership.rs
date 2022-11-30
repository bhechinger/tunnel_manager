use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

use crate::api::{PermissionMembershipData, PermissionMembershipRequest, PermissionMembershipsData};
use crate::api::permission_membership_server::PermissionMembership;
use crate::storage::permission_membership;

#[derive(Debug)]
pub struct PermissionMembershipService {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PermissionMembershipService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl PermissionMembership for PermissionMembershipService {
    #[instrument]
    async fn list(&self, request: Request<()>) -> Result<Response<PermissionMembershipsData>, Status> {
        info!(message = "Got a list request", ?request);

        match permission_membership::PermissionMembership::all(&self.pool).await {
            Ok(result) => Ok(Response::new(PermissionMembershipsData { memberships: result })),
            Err(status) => {
                error!(
                    message = "Error getting list of permission membership",
                    status = status.message()
                );
                return Err(status);
            }
        }
    }

    #[instrument]
    async fn get_permission_members(&self, request: Request<PermissionMembershipRequest>) -> Result<Response<PermissionMembershipsData>, Status> {
        info!(message = "Got a get request", ?request);

        let req = request.into_inner();

        match req.id_permission_or_userid {
            Some(id_permission_or_userid) => match permission_membership::PermissionMembership::get(&self.pool, &id_permission_or_userid).await {
                Ok(result) => Ok(Response::new(PermissionMembershipsData { memberships: result })),
                Err(status) => {
                    error!(
                        message = "Error getting permission membership by id",
                        status = status.message()
                    );
                    return Err(status);
                }
            },
            None => Err(Status::invalid_argument("PermissionMembership id, uuid or owner required")),
        }
    }

    #[instrument]
    async fn get_user_permissions(&self, request: Request<PermissionMembershipRequest>) -> Result<Response<PermissionMembershipsData>, Status> {
        info!(message = "Got a get request", ?request);

        let req = request.into_inner();

        match req.id_permission_or_userid {
            Some(id_permission_or_userid) => match permission_membership::PermissionMembership::get(&self.pool, &id_permission_or_userid).await {
                Ok(result) => Ok(Response::new(PermissionMembershipsData { memberships: result })),
                Err(status) => {
                    error!(
                        message = "Error getting permission membership by id",
                        status = status.message()
                    );
                    return Err(status);
                }
            },
            None => Err(Status::invalid_argument("PermissionMembership id, uuid or owner required")),
        }
    }

    #[instrument]
    async fn add(&self, request: Request<PermissionMembershipData>) -> Result<Response<PermissionMembershipData>, Status> {
        info!(message = "Got an add request", ?request);

        let req = request.into_inner();

        if req.permission == 0 {
            return Err(Status::invalid_argument("permission is required"));
        }

        if req.user_id == 0 {
            return Err(Status::invalid_argument("user id is required"));
        }

        match permission_membership::PermissionMembership::add(&self.pool, req).await {
            Ok(result) => Ok(Response::new(result)),
            Err(status) => {
                error!(message = "Error adding permission membership", status = status.message());
                return Err(status);
            }
        }
    }

    #[instrument]
    async fn delete(&self, request: Request<PermissionMembershipRequest>) -> Result<Response<PermissionMembershipData>, Status> {
        info!(message = "Got a delete request", ?request);

        let req = request.into_inner();

        match req.id_permission_or_userid {
            Some(id_permission_or_userid) => match permission_membership::PermissionMembership::delete(&self.pool, id_permission_or_userid).await {
                Ok(_) => Ok(Response::new(PermissionMembershipData::default())),
                Err(status) => {
                    error!(
                        message = "Error deleting permission membership",
                        status = status.message()
                    );
                    return Err(status);
                }
            },
            None => Err(Status::invalid_argument("PermissionMembership id or email required")),
        }
    }

    #[instrument]
    async fn update(&self, request: Request<PermissionMembershipData>) -> Result<Response<PermissionMembershipData>, Status> {
        info!(message = "Got an update request", ?request);

        let req = request.into_inner();
        if req.id.is_none() {
            return Err(Status::invalid_argument("PermissionMembership id required"));
        }

        match permission_membership::PermissionMembership::update(&self.pool, req).await {
            Ok(result) => Ok(Response::new(result)),
            Err(status) => {
                error!(
                    message = "Error updating permission membership",
                    status = status.message()
                );
                return Err(status);
            }
        }
    }
}

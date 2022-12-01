use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

use crate::api::{UserData, UserRequest, UsersData};
use crate::api::user_server::User;
use crate::storage::users;

#[derive(Debug)]
pub struct UserService {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl UserService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl User for UserService {
    #[instrument]
    async fn list(&self, request: Request<()>) -> Result<Response<UsersData>, Status> {
        info!(message = "Got a list request", ?request);

        match users::User::all(&self.pool).await {
            Ok(result) => Ok(Response::new(UsersData { users: result })),
            Err(status) => {
                error!(
                    message = "Error getting list of users",
                    status = status.message()
                );
                return Err(status);
            }
        }
    }

    #[instrument]
    async fn get(&self, request: Request<UserRequest>) -> Result<Response<UserData>, Status> {
        info!(message = "Got a get request", ?request);

        let req = request.into_inner();

        match req.id_or_email {
            Some(id_or_email) => match users::User::get(&self.pool, &id_or_email).await {
                Ok(result) => Ok(Response::new(result)),
                Err(status) => {
                    error!(
                        message = "Error getting user",
                        status = status.message()
                    );
                    return Err(status);
                }
            },
            None => Err(Status::invalid_argument("User id or email required")),
        }
    }

    #[instrument]
    async fn add(&self, request: Request<UserData>) -> Result<Response<UserData>, Status> {
        info!(message = "Got an add request", ?request);

        let req = request.into_inner();

        if req.email.is_empty() {
            return Err(Status::invalid_argument("email is required"));
        }

        match users::User::add(&self.pool, req).await {
            Ok(result) => Ok(Response::new(result)),
            Err(status) => {
                error!(message = "Error adding user", status = status.message());
                return Err(status);
            }
        }
    }

    #[instrument]
    async fn delete(&self, request: Request<UserRequest>) -> Result<Response<UserData>, Status> {
        info!(message = "Got a delete request", ?request);

        let req = request.into_inner();

        match req.id_or_email {
            Some(id_or_email) => match users::User::delete(&self.pool, id_or_email).await {
                Ok(_) => Ok(Response::new(UserData::default())),
                Err(status) => {
                    error!(
                        message = "Error deleting user",
                        status = status.message()
                    );
                    return Err(status);
                }
            },
            None => Err(Status::invalid_argument("User id or email required")),
        }
    }

    #[instrument]
    async fn update(&self, request: Request<UserData>) -> Result<Response<UserData>, Status> {
        info!(message = "Got an update request", ?request);

        let req = request.into_inner();
        if req.id.is_none() {
            return Err(Status::invalid_argument("User id required"));
        }

        match users::User::update(&self.pool, req).await {
            Ok(result) => Ok(Response::new(result)),
            Err(status) => {
                error!(
                    message = "Error deleting user by email",
                    status = status.message()
                );
                return Err(status);
            }
        }
    }
}

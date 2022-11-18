use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

use crate::api::user_request::IdOrEmail;
use crate::api::user_server::User;
use crate::api::{UserAddRequest, UserData, UserRequest, UsersData};
use crate::storage::users::Users;

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

        match Users::all(&self.pool).await {
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
            Some(IdOrEmail::Id(id)) => match Users::get_by_id(&self.pool, id).await {
                Ok(result) => Ok(Response::new(result)),
                Err(status) => {
                    error!(
                        message = "Error getting user by id",
                        status = status.message()
                    );
                    return Err(status);
                }
            },
            Some(IdOrEmail::Email(email)) => match Users::get_by_email(&self.pool, email).await {
                Ok(result) => Ok(Response::new(result)),
                Err(status) => {
                    error!(
                        message = "Error getting user by email",
                        status = status.message()
                    );
                    return Err(status);
                }
            },
            None => Err(Status::invalid_argument("User id or email required")),
        }
    }

    #[instrument]
    async fn add(&self, request: Request<UserAddRequest>) -> Result<Response<UserData>, Error> {
        info!(message = "Got an add request", ?request);

        match Users::add(&self.pool, request.into_inner().email).await {
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
            Some(IdOrEmail::Id(id)) => match Users::delete_by_id(&self.pool, id).await {
                Ok(result) => Ok(Response::new(result)),
                Err(status) => {
                    error!(
                        message = "Error deleting user by id",
                        status = status.message()
                    );
                    return Err(status);
                }
            },
            Some(IdOrEmail::Email(email)) => {
                match Users::delete_by_email(&self.pool, email).await {
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
            None => Err(Status::invalid_argument("User id or email required")),
        }
    }

    #[instrument]
    async fn update(&self, request: Request<UserData>) -> Result<Response<UserData>, Status> {
        info!(message = "Got an update request", ?request);

        let req = request.into_inner();

        match Users::update(&self.pool, req.id, req.email).await {
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
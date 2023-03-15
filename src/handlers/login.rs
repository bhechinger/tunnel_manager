// use diesel::prelude::*;
// use diesel::r2d2::{ConnectionManager, Pool};
// use tonic::{Request, Response, Status};
// use tracing::{error, info, instrument};
//
// use crate::api::{LoginResponse, UserRequest};
// use crate::api::user_server::User;
// use crate::storage::users;
//
// #[derive(Debug)]
// pub struct UserService {
//     pool: Pool<ConnectionManager<PgConnection>>,
// }
//
// impl UserService {
//     pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
//         Self { pool }
//     }
// }
//
// #[tonic::async_trait]
// impl User for UserService {
//     #[instrument]
//     async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
//         info!(message = "Got a login request", ?request);
//
//         let req = request.into_inner();
//
//         if req.email.is_empty() {
//             return Err(Status::invalid_argument("email is required"));
//         }
//
//         if req.password.is_empty() {
//             return Err(Status::invalid_argument("password is required"));
//         }
//
//         match users::User::login(&self.pool, req).await {
//             Ok(result) => Ok(Response::new(result)),
//             Err(status) => {
//                 error!(message = "Error logging in", status = status.message());
//                 return Err(status);
//             }
//         }
//     }
//
//     #[instrument]
//     async fn register(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
//         info!(message = "Got a register request", ?request);
//
//         let req = request.into_inner();
//
//         if req.email.is_empty() {
//             return Err(Status::invalid_argument("email is required"));
//         }
//
//         if req.password.is_empty() {
//             return Err(Status::invalid_argument("password is required"));
//         }
//
//         match users::User::register(&self.pool, req).await {
//             Ok(result) => Ok(Response::new(result)),
//             Err(status) => {
//                 error!(message = "Error adding user", status = status.message());
//                 return Err(status);
//             }
//         }
//     }
// }

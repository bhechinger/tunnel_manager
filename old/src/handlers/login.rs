use diesel::r2d2::Pool;
use diesel::r2d2::{ConnectionManager, Pool};
use tonic::{Request, Response, Status};

use crate::api::auth_server::Auth;
use crate::api::{LoginRequest, LoginResponse};

#[derive(Debug, Default)]
pub struct AuthService {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl AuthService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn login(
        &self,
        request: Request<LoginRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<LoginResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a login request: {:?}", request);

        let reply = LoginResponse {
            error: false,
            message: format!("Agent login"), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

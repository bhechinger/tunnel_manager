use tonic::{Request, Response, Status};
use crate::api::user_server::User;
use crate::api::{UserListResponse};
use crate::api::{UserGetRequest, UserGetResponse};
use crate::api::{UserAddRequest, UserAddResponse};
use crate::api::{UserDeleteRequest, UserDeleteResponse};
use crate::api::{UserUpdateRequest, UserUpdateResponse};
use crate::api::{UserResponse, UserData};

#[derive(Debug, Default)]
pub struct UserService;

#[tonic::async_trait]
impl User for UserService {
    async fn list(
        &self,
        request: Request<()>, // Accept request of type HelloRequest
    ) -> Result<Response<UserListResponse>, Status> { // Return an instance of type HelloReply
        println!("Got a list request: {:?}", request);

        let reply = UserListResponse {
            response: Some(UserResponse {
                error: false,
                message: format!("User list"), // We must use .into_inner() as the fields of gRPC requests and responses are private
            }),
            users: vec![
                UserData {
                    id: 1,
                    email: "fake@news.com".to_string(),
                },
                UserData {
                    id: 2,
                    email: "fake2@news.com".to_string(),
                },
            ],
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn get(
        &self,
        request: Request<UserGetRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<UserGetResponse>, Status> { // Return an instance of type HelloReply
        println!("Got a get request: {:?}", request);

        let reply = UserGetResponse {
            response: Some(UserResponse {
                error: false,
                message: format!("User get {}!", request.into_inner().id_or_email).into(),
            }),
            user: Some(UserData {
                id: 1,
                email: "fake@news.com".to_string(),
            }),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn add(
        &self,
        request: Request<UserAddRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<UserAddResponse>, Status> { // Return an instance of type HelloReply
        println!("Got am add request: {:?}", request);

        let user = request.into_inner().user.unwrap_or_default();

        let reply = UserAddResponse {
            response: Some(UserResponse {
                error: false,
                message: format!("User add {} {}!", user.id, user.email).into(),
            }),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn delete(
        &self,
        request: Request<UserDeleteRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<UserDeleteResponse>, Status> { // Return an instance of type HelloReply
        println!("Got a delete request: {:?}", request);

        let reply = UserDeleteResponse {
            response: Some(UserResponse {
                error: false,
                message: format!("User delete {}!", request.into_inner().id_or_email.as_deref().unwrap_or_default()).into(),
            }),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn update(
        &self,
        request: Request<UserUpdateRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<UserUpdateResponse>, Status> { // Return an instance of type HelloReply
        println!("Got an update request: {:?}", request);

        let user = request.into_inner().user.unwrap_or_default();

        let reply = UserUpdateResponse {
            response: Some(UserResponse {
                error: false,
                message: format!("User update {}!", user.id).into(),
            }),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}
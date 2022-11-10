use crate::api::user_get_request::IdOrEmail;
use crate::api::user_server::User;
use crate::api::{UserAddRequest, UserData, UserDeleteRequest, UserGetRequest, UsersData};
use crate::helpers::db::sql_err_to_grpc_error;
use crate::models::users::Users;
use sqlx::postgres::PgPool;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct UserService {
    pool: PgPool,
}

impl UserService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl User for UserService {
    async fn list(
        &self,
        request: Request<()>, // Accept request of type HelloRequest
    ) -> Result<Response<UsersData>, Status> {
        // Return an instance of type HelloReply
        println!("Got a list request: {:?}", request);

        // let users = Users::all(&self.pool).await.unwrap();
        // Ok(Response::new(UsersData { users }))
        Ok(Response::new(UsersData {
            users: Users::all(&self.pool).await.unwrap(),
        })) // Send back our formatted greeting
    }

    async fn get(
        &self,
        request: Request<UserGetRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<UserData>, Status> {
        // Return an instance of type HelloReply
        println!("Got a get request: {:?}", request);

        let req = request.into_inner();

        match req.id_or_email {
            Some(IdOrEmail::Id(id)) => match Users::get_by_id(&self.pool, id).await {
                Err(e) => {
                    println!("Error getting user by id: {:?}", e);
                    return Err(sql_err_to_grpc_error(e));
                }
                Ok(r) => Ok(Response::new(r)),
            },
            Some(IdOrEmail::Email(email)) => match Users::get_by_email(&self.pool, email).await {
                Err(e) => {
                    println!("Error getting user by email: {:?}", e);
                    return Err(sql_err_to_grpc_error(e));
                }
                Ok(r) => Ok(Response::new(r)),
            },
            None => Err(Status::invalid_argument("User id or email required")),
        }
    }

    async fn add(
        &self,
        request: Request<UserAddRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<UserData>, Status> {
        // Return an instance of type HelloReply
        println!("Got an add request: {:?}", request);

        let user = Users::add(&self.pool, "fake-uuid".to_string())
            .await
            .unwrap();

        Ok(Response::new(user))
    }

    async fn delete(
        &self,
        request: Request<UserDeleteRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<UserData>, Status> {
        // Return an instance of type HelloReply
        println!("Got a delete request: {:?}", request);

        // let user = Users::delete(&self.pool, request.into_inner().id_or_email)
        let user = Users::add(&self.pool, "fake-uuid".to_string())
            .await
            .unwrap();

        Ok(Response::new(user))
    }

    async fn update(
        &self,
        request: Request<UserData>, // Accept request of type HelloRequest
    ) -> Result<Response<UserData>, Status> {
        // Return an instance of type HelloReply
        println!("Got an update request: {:?}", request);

        let req = request.into_inner();

        let user = Users::update(&self.pool, req.id, req.email).await.unwrap();

        Ok(Response::new(user))
    }
}

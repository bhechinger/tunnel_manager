use sqlx::postgres::PgPool;
use tonic::Status;

use crate::api::UserData;
use crate::models::helpers::sql_err_to_grpc_error;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Users {
    pub id: i32,
    pub email: String, // this is a foreign key to users
}

impl Default for Users {
    fn default() -> Users {
        Users {
            id: 0,
            email: "".to_string(),
        }
    }
}

impl Users {
    pub async fn all(pool: &PgPool) -> Result<Vec<UserData>, Status> {
        match sqlx::query_as!(Users, "SELECT id, email FROM users ORDER by id")
            .fetch_all(pool)
            .await
        {
            Ok(users) => Ok(users.iter().map(|t| t.into_response()).collect()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<UserData, Status> {
        match sqlx::query_as!(Users, "SELECT id, email from users WHERE id = $1", id)
            .fetch_one(pool)
            .await
        {
            Ok(user) => Ok(user.into_response()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    pub async fn get_by_email(pool: &PgPool, email: String) -> Result<UserData, Status> {
        match sqlx::query_as!(Users, "SELECT id, email from users WHERE email = $1", email)
            .fetch_one(pool)
            .await
        {
            Ok(user) => Ok(user.into_response()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    pub async fn add(pool: &PgPool, email: String) -> Result<UserData, Status> {
        match sqlx::query_as!(
            Users,
            "INSERT INTO users (email) VALUES ( $1 ) RETURNING id, email",
            email,
        )
        .fetch_one(pool)
        .await
        {
            Ok(r) => Ok(r.into_response()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    pub async fn update(pool: &PgPool, id: i32, email: String) -> Result<UserData, Status> {
        match sqlx::query_as!(
            Users,
            "UPDATE users SET email = $1 WHERE id = $2 ",
            email,
            id
        )
        .fetch_one(pool)
        .await
        {
            Ok(_) => Ok(Users::default().into_response()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    pub async fn delete_by_id(pool: &PgPool, id: i32) -> Result<UserData, Status> {
        match sqlx::query_as!(Users, "DELETE FROM users WHERE id = $1", id)
            .fetch_one(pool)
            .await
        {
            Ok(_) => Ok(Users::default().into_response()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    pub async fn delete_by_email(pool: &PgPool, email: String) -> Result<UserData, Status> {
        match sqlx::query_as!(Users, "DELETE FROM users WHERE email = $1", email)
            .fetch_one(pool)
            .await
        {
            Ok(_) => Ok(Users::default().into_response()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    fn into_response(&self) -> UserData {
        UserData {
            id: self.id,
            email: self.email.clone(),
        }
    }
}

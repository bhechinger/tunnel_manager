use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use diesel::{PgConnection, QueryResult, RunQueryDsl};
use tonic::Status;
use tracing::{instrument, warn};

use crate::api::UserData;
use crate::models::NewUser;
use crate::schema::users;
use crate::storage::helpers::sql_err_to_grpc_error;

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

impl From<Users> for UserData {
    fn from(u: Users) -> UserData {
        UserData {
            id: u.id,
            email: u.email,
        }
    }
}

impl From<&Users> for UserData {
    fn from(u: &Users) -> UserData {
        UserData {
            id: u.id.clone(),
            email: u.email.clone(),
        }
    }
}

impl Users {
    #[instrument]
    pub async fn all(
        pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<Vec<UserData>, Status> {
        match sqlx::query_as!(Users, "SELECT id, email FROM users ORDER by id")
            .fetch_all(pool)
            .await
        {
            Ok(users) => Ok(users.iter().map(|t| t.into()).collect()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn get_by_id(
        pool: &Pool<ConnectionManager<PgConnection>>,
        id: i32,
    ) -> Result<UserData, Status> {
        match sqlx::query_as!(Users, "SELECT id, email from users WHERE id = $1", id)
            .fetch_one(pool)
            .await
        {
            Ok(user) => Ok(user.into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn get_by_email(
        pool: &Pool<ConnectionManager<PgConnection>>,
        email: crate::schema::users::columns::email,
    ) -> Result<UserData, Status> {
        match sqlx::query_as!(Users, "SELECT id, email from users WHERE email = $1", email)
            .fetch_one(pool)
            .await
        {
            Ok(user) => Ok(user.into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn add(
        pool: &Pool<ConnectionManager<PgConnection>>,
        email: &str,
    ) -> Result<QueryResult<usize>, Error> {
        let conn = &mut pool.get().unwrap();

        let new_user = NewUser { email };

        let rows_inserted = diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn);
        return Ok(rows_inserted);
        // match sqlx::query_as!(
        //     Users,
        //     "INSERT INTO users (email) VALUES ( $1 ) RETURNING id, email",
        //     email,
        // )
        // .fetch_one(pool)
        // .await
        // {
        //     Ok(r) => Ok(r.into()),
        //     Err(err) => Err(sql_err_to_grpc_error(err)),
        // }
    }

    #[instrument]
    pub async fn update(
        pool: &Pool<ConnectionManager<PgConnection>>,
        id: i32,
        email: String,
    ) -> Result<UserData, Status> {
        match sqlx::query_as!(
            Users,
            "UPDATE users SET email = $1 WHERE id = $2 ",
            email,
            id
        )
        .fetch_one(pool)
        .await
        {
            Ok(_) => Ok(Users::default().into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn delete_by_id(
        pool: &Pool<ConnectionManager<PgConnection>>,
        id: i32,
    ) -> Result<UserData, Status> {
        match sqlx::query_as!(Users, "DELETE FROM users WHERE id = $1", id)
            .fetch_one(pool)
            .await
        {
            Ok(_) => Ok(Users::default().into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn delete_by_email(
        pool: &Pool<ConnectionManager<PgConnection>>,
        email: String,
    ) -> Result<UserData, Status> {
        match sqlx::query_as!(Users, "DELETE FROM users WHERE email = $1", email)
            .fetch_one(pool)
            .await
        {
            Ok(_) => Ok(Users::default().into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }
}

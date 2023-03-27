use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tonic::Status;
use tracing::instrument;

use crate::api::{UserAddRequest, UserResponse, UserUpdateRequest};
use crate::api::user_request::IdOrEmail;
use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::storage::helpers::sql_err_to_grpc_error;

#[derive(Queryable, Default, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: &'a str,
}

#[derive(AsChangeset, Default)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub email: Option<String>,
}

impl From<User> for UserResponse {
    fn from(u: User) -> UserResponse {
        UserResponse {
            id: u.id,
            email: u.email,
        }
    }
}

impl From<&User> for UserResponse {
    fn from(u: &User) -> UserResponse {
        UserResponse {
            id: u.id,
            email: u.email.clone(),
        }
    }
}

impl User {
    #[instrument]
    pub async fn all(
        pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<Vec<UserResponse>, Status> {
        let conn = &mut pool.get().unwrap();

        match users.load::<User>(conn) {
            Ok(results) => Ok(results.iter().map(|t| t.into()).collect()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn get(
        pool: &Pool<ConnectionManager<PgConnection>>,
        id_or_email: &IdOrEmail,
    ) -> Result<UserResponse, Status> {
        let conn = &mut pool.get().unwrap();

        match id_or_email {
            IdOrEmail::Id(user_id) => match users.find(user_id).first::<User>(conn) {
                Ok(results) => Ok(results.into()),
                Err(err) => Err(sql_err_to_grpc_error(err)),
            },
            IdOrEmail::Email(user_email) => {
                match users.filter(email.eq(user_email)).first::<User>(conn) {
                    Ok(results) => Ok(results.into()),
                    Err(err) => Err(sql_err_to_grpc_error(err)),
                }
            }
        }
    }

    #[instrument]
    pub async fn add(
        pool: &Pool<ConnectionManager<PgConnection>>,
        user_data: UserAddRequest,
    ) -> Result<UserResponse, Status> {
        let new_user = NewUser {
            email: user_data.email.as_str(),
        };
        let conn = &mut pool.get().unwrap();

        match diesel::insert_into(users)
            .values(&new_user)
            .get_result::<User>(conn)
        {
            Ok(results) => Ok(results.into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn update(
        pool: &Pool<ConnectionManager<PgConnection>>,
        user_data: UserUpdateRequest,
    ) -> Result<UserResponse, Status> {
        let conn = &mut pool.get().unwrap();
        let mut update = UpdateUser::default();

        if user_data.id == 0 {
            return Err(Status::invalid_argument("User id is required"));
        }

        if user_data.email.is_some() {
            update.email = user_data.email;
        }

        match diesel::update(users.find(user_data.id))
            .set(update)
            .get_result::<User>(conn)
        {
            Ok(results) => Ok(results.into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn delete(
        pool: &Pool<ConnectionManager<PgConnection>>,
        id_or_email: IdOrEmail,
    ) -> Result<usize, Status> {
        let conn = &mut pool.get().unwrap();

        match id_or_email {
            IdOrEmail::Id(user_id) => match diesel::delete(users.find(user_id)).execute(conn) {
                Ok(results) => Ok(results),
                Err(err) => Err(sql_err_to_grpc_error(err)),
            },
            IdOrEmail::Email(user_email) => {
                match diesel::delete(users.filter(email.eq(user_email))).execute(conn) {
                    Ok(results) => Ok(results),
                    Err(err) => Err(sql_err_to_grpc_error(err)),
                }
            }
        }
    }
}

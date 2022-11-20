use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tonic::Status;

use crate::api::UserData;
use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::storage::helpers::sql_err_to_grpc_error;

use crate::api::user_request::IdOrEmail;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: &'a str,
}

impl Default for User {
    fn default() -> User {
        User {
            id: 0,
            email: "".to_string(),
        }
    }
}

impl From<User> for UserData {
    fn from(u: User) -> UserData {
        UserData {
            id: u.id,
            email: u.email,
        }
    }
}

impl From<&User> for UserData {
    fn from(u: &User) -> UserData {
        UserData {
            id: u.id.clone(),
            email: u.email.clone(),
        }
    }
}

impl User {
    // #[instrument]
    pub async fn all(pool: &Pool<ConnectionManager<PgConnection>>) -> Result<Vec<UserData>, Status> {
        let conn = &mut pool.get().unwrap();

        match users.load::<User>(conn) {
            Ok(results) => Ok(results.iter().map(|t| t.into()).collect()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    // #[instrument]
    pub async fn get(pool: &Pool<ConnectionManager<PgConnection>>, id_or_email: &IdOrEmail) -> Result<UserData, Status> {
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

    // #[instrument]
    pub async fn add(pool: &Pool<ConnectionManager<PgConnection>>, user_email: &str) -> Result<UserData, Status> {
        let new_user = NewUser { email: user_email };
        let conn = &mut pool.get().unwrap();

        match diesel::insert_into(users)
            .values(&new_user)
            .get_result::<User>(conn)
        {
            Ok(results) => Ok(results.into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    // #[instrument]
    pub async fn update(pool: &Pool<ConnectionManager<PgConnection>>, user_data: User) -> Result<UserData, Status> {
        let conn = &mut pool.get().unwrap();

        match diesel::update(users.find(user_data.id))
            .set(email.eq(user_data.email))
            .get_result::<User>(conn)
        {
            Ok(results) => Ok(results.into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    // #[instrument]
    pub async fn delete(
        pool: &Pool<ConnectionManager<PgConnection>>,
        id_or_email: IdOrEmail,
    ) -> Result<usize, Status> {
        let conn = &mut pool.get().unwrap();

        match id_or_email {
            IdOrEmail::Id(user_id) => match diesel::delete(users.find(user_id)).execute(conn) {
                Ok(results) => Ok(results.into()),
                Err(err) => Err(sql_err_to_grpc_error(err)),
            },
            IdOrEmail::Email(user_email) => {
                match diesel::delete(users.filter(email.eq(user_email))).execute(conn) {
                    Ok(results) => Ok(results.into()),
                    Err(err) => Err(sql_err_to_grpc_error(err)),
                }
            }
        }
    }
}

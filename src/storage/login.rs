// use bcrypt::DEFAULT_COST;
// use diesel::prelude::*;
// use diesel::r2d2::{ConnectionManager, Pool};
// use tonic::Status;
// use tracing::instrument;
//
// use crate::api::{LoginData, LoginResponse};
// use crate::api::user_request::IdOrEmail;
// use crate::schema::users;
// use crate::schema::users::dsl::*;
// use crate::storage::helpers::{bcrypt_err_to_grpc_error, sql_err_to_grpc_error};
//
// #[derive(Queryable, Default, Debug)]
// pub struct User {
//     pub id: i32,
//     pub email: String,
//     pub password: String,
// }
//
// #[derive(Insertable)]
// #[diesel(table_name = users)]
// pub struct NewUser<'a> {
//     pub email: &'a str,
//     pub password: &'a str,
// }
//
// #[derive(AsChangeset, Default)]
// #[diesel(table_name = users)]
// pub struct UpdatePassword {
//     pub password: String,
// }
//
// impl From<User> for LoginResponse {
//     fn from(u: User) -> LoginResponse {
//         LoginResponse {
//             id: Some(u.id),
//             email: u.email,
//             password: u.password,
//         }
//     }
// }
//
// impl From<&User> for LoginResponse {
//     fn from(u: &User) -> LoginResponse {
//         LoginResponse {
//             id: Some(u.id),
//             email: u.email.clone(),
//             password: u.password.clone(),
//         }
//     }
// }
//
// impl User {
//     #[instrument]
//     pub async fn login(
//         pool: &Pool<ConnectionManager<PgConnection>>,
//         login_data: &LoginData,
//     ) -> Result<LoginResponse, Status> {
//         let conn = &mut pool.get().unwrap();
//
//         match users.find(login_data.email).first::<User>(conn) {
//             Ok(results) => match bcrypt::verify(login_data.password, results.password) {
//                 Ok(valid) => {
//                     if valid {
//                         OK(results)
//                     } else {
//                         Err(Status::permission_denied("Invalid password"))
//                     }
//                 }
//                 Err(err) => Err(bcrypt_err_to_grpc_error(err)),
//             },
//             Err(err) => Err(sql_err_to_grpc_error(err)),
//         }
//     }
//
//     #[instrument]
//     pub async fn register(
//         pool: &Pool<ConnectionManager<PgConnection>>,
//         login_data: LoginData,
//     ) -> Result<LoginResponse, Status> {
//         let hash = bcrypt::hash(login_data.password, DEFAULT_COST);
//         let new_user = NewUser {
//             email: login_data.email.as_str(),
//             password: hash.unwrap().as_str(),
//         };
//         let conn = &mut pool.get().unwrap();
//
//         match diesel::insert_into(users)
//             .values(&new_user)
//             .get_result::<User>(conn)
//         {
//             Ok(results) => Ok(results.into()),
//             Err(err) => Err(sql_err_to_grpc_error(err)),
//         }
//     }
// }

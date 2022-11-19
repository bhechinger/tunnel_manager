use diesel::prelude::*;

use crate::models::users::{NewUser, Users};
use crate::schema::users::dsl::*;

pub fn create_user(conn: &mut PgConnection, user_email: &str) -> Users {
    let new_user = NewUser { email: user_email };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")
}

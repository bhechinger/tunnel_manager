use diesel::prelude::*;

use crate::models::users::{NewUser, Users};

pub fn create_user(conn: &mut PgConnection, email: &str) -> Users {
    use crate::schema::users;

    let new_user = NewUser { email };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")
}

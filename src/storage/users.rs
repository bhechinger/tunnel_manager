use diesel::prelude::*;
use diesel::result::Error;

use crate::models::users::{IdOrEmail, NewUser, User};
use crate::schema::users::dsl::*;

pub fn create_user(conn: &mut PgConnection, user_email: &str) -> User {
    let new_user = NewUser { email: user_email };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn get_users(conn: &mut PgConnection) -> Vec<User> {
    users.load::<User>(conn).expect("Error loading users")
}

pub fn get_user(conn: &mut PgConnection, id_or_email: &IdOrEmail) -> Result<Vec<User>, Error> {
    match id_or_email {
        IdOrEmail::Id(user_id) => users.find(user_id).load::<User>(conn),
        IdOrEmail::Email(user_email) => users.filter(email.eq(user_email)).load::<User>(conn),
    }
}

pub fn delete_user(conn: &mut PgConnection, user_id: &i32) -> usize {
    diesel::delete(users.find(user_id))
        .execute(conn)
        .expect("Error deleting posts")
}

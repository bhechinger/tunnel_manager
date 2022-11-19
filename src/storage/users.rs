use diesel::prelude::*;

use crate::models::users::{IdOrEmail, NewUser, User};
use crate::schema::users::dsl::*;

pub fn create_user(conn: &mut PgConnection, user_email: &str) -> QueryResult<User> {
    let new_user = NewUser { email: user_email };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
}

pub fn get_users(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
    users.load::<User>(conn)
}

pub fn get_user(conn: &mut PgConnection, id_or_email: &IdOrEmail) -> QueryResult<Vec<User>> {
    match id_or_email {
        IdOrEmail::Id(user_id) => users.find(user_id).load::<User>(conn),
        IdOrEmail::Email(user_email) => users.filter(email.eq(user_email)).load::<User>(conn),
    }
}

pub fn update_user(conn: &mut PgConnection, user_data: User) -> QueryResult<User> {
    diesel::update(users.find(user_data.id))
        .set(email.eq(user_data.email))
        .get_result::<User>(conn)
}

pub fn delete_user(conn: &mut PgConnection, user_id: &i32) -> QueryResult<usize> {
    diesel::delete(users.find(user_id)).execute(conn)
}

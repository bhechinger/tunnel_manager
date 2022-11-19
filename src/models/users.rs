use diesel::prelude::*;

use crate::api::UserData;
use crate::schema::users;

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

pub enum IdOrEmail {
    Id(i32),
    Email(String),
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
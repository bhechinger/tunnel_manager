use diesel::prelude::*;

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

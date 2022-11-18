use diesel::prelude::*;

use crate::schema::permissions;

#[derive(Queryable)]
pub struct Permissions {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Insertable)]
#[diesel(table_name = permissions)]
pub struct NewPermission<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

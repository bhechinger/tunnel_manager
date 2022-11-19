use diesel::prelude::*;

use crate::schema::agents;

#[derive(Queryable)]
pub struct Agents {
    pub id: i32,
    pub uuid: String,
    pub description: String,
    pub owner: i32,
}

#[derive(Insertable)]
#[diesel(table_name = agents)]
pub struct NewAgent<'a> {
    pub uuid: &'a str,
    pub description: &'a str,
    pub owner: i32,
}

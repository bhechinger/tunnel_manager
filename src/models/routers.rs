use diesel::prelude::*;

use crate::schema::routers;

#[derive(Queryable)]
pub struct Routers {
    pub id: i32,
    pub agent: i32,
    pub snmp_community: String,
    pub ssh_username: String,
    pub ssh_password: String,
    pub conn_type: String,
    pub router_type: String,
}

#[derive(Insertable)]
#[diesel(table_name = routers)]
pub struct NewRouter<'a> {
    pub agent: i32,
    pub snmp_community: &'a str,
    pub ssh_username: &'a str,
    pub ssh_password: &'a str,
    pub conn_type: &'a str,
    pub router_type: &'a str,
}

use diesel::prelude::*;

use crate::schema::routers;

#[derive(Queryable, Default)]
pub struct Router {
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

#[derive(AsChangeset, Default)]
#[diesel(table_name = routers)]
pub struct UpdateRouter {
    pub agent: Option<i32>,
    pub snmp_community: Option<String>,
    pub ssh_username: Option<String>,
    pub ssh_password: Option<String>,
    pub conn_type: Option<String>,
    pub router_type: Option<String>,
}

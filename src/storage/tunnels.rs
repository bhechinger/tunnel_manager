use diesel::prelude::*;

use crate::schema::tunnels;

#[derive(Queryable)]
pub struct Tunnel {
    pub id: i32,
    pub version: i32,
    pub router: i32,
    pub ip: String,
    pub dynamic_ip: bool,
    pub ip_class: i32,
    pub hostname: String,
    pub description: String,
    pub source: String,
    pub cost: i32,
    pub tunnel_type: String,
    pub topology_type: String,
}

#[derive(Insertable)]
#[diesel(table_name = tunnels)]
pub struct NewTunnel<'a> {
    pub version: i32,
    pub router: i32,
    pub ip: &'a str,
    pub dynamic_ip: bool,
    pub ip_class: i32,
    pub hostname: &'a str,
    pub description: &'a str,
    pub source: &'a str,
    pub cost: i32,
    pub tunnel_type: &'a str,
    pub topology_type: &'a str,
}

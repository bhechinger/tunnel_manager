use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tonic::Status;
use tracing::instrument;

use crate::api::{TunnelAddRequest, TunnelResponse, TunnelUpdateRequest};
use crate::api::tunnel_request::IdOrRouter;
use crate::schema::tunnels;
use crate::schema::tunnels::dsl::*;
use crate::storage::helpers::sql_err_to_grpc_error;

#[derive(Queryable, Default, Debug)]
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

#[derive(AsChangeset, Default)]
#[diesel(table_name = tunnels)]
pub struct UpdateTunnel {
    pub version: Option<i32>,
    pub router: Option<i32>,
    pub ip: Option<String>,
    pub dynamic_ip: Option<bool>,
    pub ip_class: Option<i32>,
    pub hostname: Option<String>,
    pub description: Option<String>,
    pub source: Option<String>,
    pub cost: Option<i32>,
    pub tunnel_type: Option<String>,
    pub topology_type: Option<String>,
}

impl From<Tunnel> for TunnelResponse {
    fn from(t: Tunnel) -> TunnelResponse {
        TunnelResponse {
            id: t.id,
            version: t.version,
            router: t.router,
            ip: t.ip,
            dynamic_ip: t.dynamic_ip,
            ip_class: t.ip_class,
            hostname: t.hostname,
            description: t.description,
            source: t.source,
            cost: t.cost,
            tunnel_type: t.tunnel_type,
            topology_type: t.topology_type,
        }
    }
}

impl From<&Tunnel> for TunnelResponse {
    fn from(t: &Tunnel) -> TunnelResponse {
        TunnelResponse {
            id: t.id,
            version: t.version,
            router: t.router,
            ip: t.ip.clone(),
            dynamic_ip: t.dynamic_ip,
            ip_class: t.ip_class,
            hostname: t.hostname.clone(),
            description: t.description.clone(),
            source: t.source.clone(),
            cost: t.cost,
            tunnel_type: t.tunnel_type.clone(),
            topology_type: t.topology_type.clone(),
        }
    }
}

impl Tunnel {
    #[instrument]
    pub async fn all(
        pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<Vec<TunnelResponse>, Status> {
        let conn = &mut pool.get().unwrap();

        match tunnels.load::<Tunnel>(conn) {
            Ok(results) => Ok(results.iter().map(|t| t.into()).collect()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn get(
        pool: &Pool<ConnectionManager<PgConnection>>,
        id_or_router: &IdOrRouter,
    ) -> Result<TunnelResponse, Status> {
        let conn = &mut pool.get().unwrap();

        match id_or_router {
            IdOrRouter::Id(user_id) => match tunnels.find(user_id).first::<Tunnel>(conn) {
                Ok(results) => Ok(results.into()),
                Err(err) => Err(sql_err_to_grpc_error(err)),
            },
            IdOrRouter::Router(router_id) => {
                match tunnels.filter(router.eq(router_id)).first::<Tunnel>(conn) {
                    Ok(results) => Ok(results.into()),
                    Err(err) => Err(sql_err_to_grpc_error(err)),
                }
            }
        }
    }

    #[instrument]
    pub async fn add(
        pool: &Pool<ConnectionManager<PgConnection>>,
        tunnel_data: TunnelAddRequest,
    ) -> Result<TunnelResponse, Status> {
        let tun_type = tunnel_data.tunnel_type.unwrap_or_default();
        let top_type = tunnel_data.topology_type.unwrap_or_default();
        let new_user = NewTunnel {
            version: tunnel_data.version.unwrap_or_default(),
            router: tunnel_data.router,
            ip: tunnel_data.ip.as_str(),
            dynamic_ip: tunnel_data.dynamic_ip.unwrap_or_default(),
            ip_class: tunnel_data.ip_class.unwrap_or_default(),
            description: tunnel_data.description.as_str(),
            source: tunnel_data.source.as_str(),
            cost: tunnel_data.cost.unwrap_or_default(),
            tunnel_type: tun_type.as_str(),
            hostname: tunnel_data.hostname.as_str(),
            topology_type: top_type.as_str(),
        };
        let conn = &mut pool.get().unwrap();

        match diesel::insert_into(tunnels)
            .values(&new_user)
            .get_result::<Tunnel>(conn)
        {
            Ok(results) => Ok(results.into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn update(
        pool: &Pool<ConnectionManager<PgConnection>>,
        tunnel_data: TunnelUpdateRequest,
    ) -> Result<TunnelResponse, Status> {
        let conn = &mut pool.get().unwrap();
        let mut update = UpdateTunnel::default();

        if !tunnel_data.version.is_none() {
            update.version = tunnel_data.version;
        }

        if !tunnel_data.router.is_none() {
            update.router = tunnel_data.router;
        }

        if !tunnel_data.ip.is_none() {
            update.ip = tunnel_data.ip;
        }

        if !tunnel_data.ip_class.is_none()  {
            update.ip_class = tunnel_data.ip_class;
        }

        if !tunnel_data.description.is_none() {
            update.description = tunnel_data.description;
        }

        if !tunnel_data.source.is_none() {
            update.source = tunnel_data.source;
        }

        if !tunnel_data.cost.is_none()  {
            update.cost = tunnel_data.cost;
        }

        if !tunnel_data.tunnel_type.is_none() {
            update.tunnel_type = tunnel_data.tunnel_type;
        }

        if !tunnel_data.hostname.is_none() {
            update.hostname = tunnel_data.hostname;
        }

        if !tunnel_data.topology_type.is_none() {
            update.topology_type = tunnel_data.topology_type;
        }

        match diesel::update(tunnels.find(tunnel_data.id))
            .set(update)
            .get_result::<Tunnel>(conn)
        {
            Ok(results) => Ok(results.into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn delete(
        pool: &Pool<ConnectionManager<PgConnection>>,
        id_or_router: IdOrRouter,
    ) -> Result<usize, Status> {
        let conn = &mut pool.get().unwrap();

        match id_or_router {
            IdOrRouter::Id(permission_id) => {
                match diesel::delete(tunnels.find(permission_id)).execute(conn) {
                    Ok(results) => Ok(results),
                    Err(err) => Err(sql_err_to_grpc_error(err)),
                }
            }
            IdOrRouter::Router(router_id) => {
                match diesel::delete(tunnels.filter(router.eq(router_id))).execute(conn) {
                    Ok(results) => Ok(results),
                    Err(err) => Err(sql_err_to_grpc_error(err)),
                }
            }
        }
    }
}

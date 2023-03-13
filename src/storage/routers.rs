use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tonic::Status;
use tracing::instrument;

use crate::api::router_request::IdOrAgent;
use crate::api::{RouterResponse, RouterAddRequest, RouterUpdateRequest};
use crate::schema::routers;
use crate::schema::routers::dsl::*;
use crate::storage::helpers::sql_err_to_grpc_error;

#[derive(Queryable, AsChangeset, Default, Debug)]
pub struct Router {
    pub id: i32,
    pub agent: i32,
    pub snmp_community: Option<String>,
    pub ssh_username: Option<String>,
    pub ssh_password: Option<String>,
    pub conn_type: Option<String>,
    pub router_type: Option<String>,
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

impl From<Router> for RouterResponse {
    fn from(r: Router) -> RouterResponse {
        RouterResponse {
            id: Some(r.id),
            agent: Some(r.agent),
            snmp_community: r.snmp_community,
            ssh_username: r.ssh_username,
            ssh_password: r.ssh_password,
            conn_type: r.conn_type,
            router_type: r.router_type,
        }
    }
}

impl From<&Router> for RouterResponse {
    fn from(r: &Router) -> RouterResponse {
        RouterResponse {
            id: Some(r.id),
            agent: Some(r.agent),
            snmp_community: r.snmp_community.clone(),
            ssh_username: r.ssh_username.clone(),
            ssh_password: r.ssh_password.clone(),
            conn_type: r.conn_type.clone(),
            router_type: r.router_type.clone(),
        }
    }
}

impl Router {
    #[instrument]
    pub async fn all(
        pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<Vec<RouterResponse>, Status> {
        let conn = &mut pool.get().unwrap();

        match routers.load::<Router>(conn) {
            Ok(results) => Ok(results.iter().map(|t| t.into()).collect()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn get(
        pool: &Pool<ConnectionManager<PgConnection>>,
        id_or_agent: &IdOrAgent,
    ) -> Result<RouterResponse, Status> {
        let conn = &mut pool.get().unwrap();

        match id_or_agent {
            IdOrAgent::Id(user_id) => match routers.find(user_id).first::<Router>(conn) {
                Ok(results) => Ok(results.into()),
                Err(err) => Err(sql_err_to_grpc_error(err)),
            },
            IdOrAgent::Agent(agent_id) => {
                match routers.filter(agent.eq(agent_id)).first::<Router>(conn) {
                    Ok(results) => Ok(results.into()),
                    Err(err) => Err(sql_err_to_grpc_error(err)),
                }
            }
        }
    }

    #[instrument]
    pub async fn add(
        pool: &Pool<ConnectionManager<PgConnection>>,
        router_data: RouterAddRequest,
    ) -> Result<RouterResponse, Status> {
        let new_community = router_data.snmp_community.unwrap_or_default();
        let new_username = router_data.ssh_username.unwrap_or_default();
        let new_password = router_data.ssh_password.unwrap_or_default();
        let new_conn_type = router_data.conn_type.unwrap_or_default();
        let new_router_type = router_data.router_type.unwrap_or_default();
        let new_router = NewRouter {
            agent: router_data.agent,
            snmp_community: new_community.as_str(),
            ssh_username: new_username.as_str(),
            ssh_password: new_password.as_str(),
            conn_type: new_conn_type.as_str(),
            router_type: new_router_type.as_str(),
        };
        let conn = &mut pool.get().unwrap();

        match diesel::insert_into(routers)
            .values(&new_router)
            .get_result::<Router>(conn)
        {
            Ok(results) => Ok(results.into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn update(
        pool: &Pool<ConnectionManager<PgConnection>>,
        router_data: RouterUpdateRequest,
    ) -> Result<RouterResponse, Status> {
        let conn = &mut pool.get().unwrap();
        let mut update = UpdateRouter::default();

        if router_data.id == 0 {
            return Err(Status::invalid_argument("Router id is required"));
        }

        if !router_data.agent.is_none() {
            update.agent = router_data.agent;
        }

        if !router_data.snmp_community.is_none() {
            update.snmp_community = router_data.snmp_community.clone();
        }

        if !router_data.ssh_username.is_none() {
            update.ssh_username = router_data.ssh_username.clone();
        }

        if !router_data.ssh_password.is_none() {
            update.ssh_password = router_data.ssh_password.clone();
        }

        if !router_data.conn_type.is_none() {
            update.conn_type = router_data.conn_type.clone();
        }

        if !router_data.router_type.is_none() {
            update.router_type = router_data.router_type.clone();
        }

        match diesel::update(routers.find(router_data.id))
            .set(update)
            .get_result::<Router>(conn)
        {
            Ok(results) => Ok(results.into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn delete(
        pool: &Pool<ConnectionManager<PgConnection>>,
        id_or_agent: IdOrAgent,
    ) -> Result<usize, Status> {
        let conn = &mut pool.get().unwrap();

        match id_or_agent {
            IdOrAgent::Id(router_id) => {
                match diesel::delete(routers.find(router_id)).execute(conn) {
                    Ok(results) => Ok(results),
                    Err(err) => Err(sql_err_to_grpc_error(err)),
                }
            }
            IdOrAgent::Agent(agent_id) => {
                match diesel::delete(routers.filter(agent.eq(agent_id))).execute(conn) {
                    Ok(results) => Ok(results),
                    Err(err) => Err(sql_err_to_grpc_error(err)),
                }
            }
        }
    }
}

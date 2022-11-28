use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tonic::Status;
use tracing::instrument;

use crate::api::agent_request::IdUuidOrOwner;
use crate::api::AgentData;
use crate::schema::agents;
use crate::schema::agents::dsl::*;
use crate::storage::helpers::sql_err_to_grpc_error;

#[derive(Queryable, Default, Debug)]
pub struct Agent {
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

#[derive(AsChangeset, Default)]
#[diesel(table_name = agents)]
pub struct UpdateAgent {
    pub uuid: Option<String>,
    pub description: Option<String>,
    pub owner: Option<i32>,
}

impl From<Agent> for AgentData {
    fn from(a: Agent) -> AgentData {
        AgentData {
            id: Some(a.id),
            uuid: a.uuid,
            description: Some(a.description),
            owner: a.owner,
        }
    }
}

impl From<&Agent> for AgentData {
    fn from(a: &Agent) -> AgentData {
        AgentData {
            id: Some(a.id),
            uuid: a.uuid.clone(),
            description: Some(a.description.clone()),
            owner: a.owner,
        }
    }
}

impl Agent {
    #[instrument]
    pub async fn all(pool: &Pool<ConnectionManager<PgConnection>>) -> Result<Vec<AgentData>, Status> {
        let conn = &mut pool.get().unwrap();

        match agents.load::<Agent>(conn) {
            Ok(results) => Ok(results.iter().map(|t| t.into()).collect()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn get(pool: &Pool<ConnectionManager<PgConnection>>, id_or_name: &IdUuidOrOwner) -> Result<AgentData, Status> {
        let conn = &mut pool.get().unwrap();

        match id_or_name {
            IdUuidOrOwner::Id(agent_id) => match agents.find(agent_id).first::<Agent>(conn) {
                Ok(results) => Ok(results.into()),
                Err(err) => Err(sql_err_to_grpc_error(err)),
            },
            IdUuidOrOwner::Uuid(agent_uuid) => {
                match agents.filter(uuid.eq(agent_uuid)).first::<Agent>(conn) {
                    Ok(results) => Ok(results.into()),
                    Err(err) => Err(sql_err_to_grpc_error(err)),
                }
            }
            IdUuidOrOwner::Owner(agent_owner) => {
                match agents.filter(owner.eq(agent_owner)).first::<Agent>(conn) {
                    Ok(results) => Ok(results.into()),
                    Err(err) => Err(sql_err_to_grpc_error(err)),
                }
            }
        }
    }

    #[instrument]
    pub async fn add(pool: &Pool<ConnectionManager<PgConnection>>, agent_name: &str, agent_description: &str, agent_owner: i32) -> Result<AgentData, Status> {
        let new_agent = NewAgent { uuid: agent_name, description: agent_description, owner: agent_owner };
        let conn = &mut pool.get().unwrap();

        match diesel::insert_into(agents)
            .values(&new_agent)
            .get_result::<Agent>(conn)
        {
            Ok(results) => Ok(results.into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn update(pool: &Pool<ConnectionManager<PgConnection>>, agent_data: Agent) -> Result<AgentData, Status> {
        let conn = &mut pool.get().unwrap();
        let mut update = UpdateAgent::default();

        if !agent_data.uuid.is_empty() {
            update.uuid = Some(agent_data.uuid);
        }

        if !agent_data.description.is_empty() {
            update.description = Some(agent_data.description)
        }

        if agent_data.owner != 0 {
            update.owner = Some(agent_data.owner)
        }

        match diesel::update(agents.find(agent_data.id))
            .set(update)
            .get_result::<Agent>(conn)
        {
            Ok(results) => Ok(results.into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn delete(
        pool: &Pool<ConnectionManager<PgConnection>>,
        id_uuid_or_owner: IdUuidOrOwner,
    ) -> Result<usize, Status> {
        let conn = &mut pool.get().unwrap();

        match id_uuid_or_owner {
            IdUuidOrOwner::Id(agent_id) => match diesel::delete(agents.find(agent_id)).execute(conn) {
                Ok(results) => Ok(results),
                Err(err) => Err(sql_err_to_grpc_error(err)),
            },
            IdUuidOrOwner::Uuid(agent_uuid) => {
                match diesel::delete(agents.filter(uuid.eq(agent_uuid))).execute(conn) {
                    Ok(results) => Ok(results),
                    Err(err) => Err(sql_err_to_grpc_error(err)),
                }
            }
            IdUuidOrOwner::Owner(owner_id) => {
                match diesel::delete(agents.filter(owner.eq(owner_id))).execute(conn) {
                    Ok(results) => Ok(results),
                    Err(err) => Err(sql_err_to_grpc_error(err)),
                }
            }
        }
    }
}
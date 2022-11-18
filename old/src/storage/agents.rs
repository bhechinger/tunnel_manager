use crate::api::AgentData;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use diesel::PgConnection;

pub struct Agents {
    pub uuid: String,
    pub owner: i32, // this is a foreign key to users
}

impl Default for Agents {
    fn default() -> Agents {
        Agents {
            uuid: "".to_string(),
            owner: 0,
        }
    }
}

impl From<Agents> for AgentData {
    fn from(u: Agents) -> AgentData {
        AgentData {
            uuid: u.uuid,
            owner: u.owner,
        }
    }
}

impl From<&Agents> for AgentData {
    fn from(u: &Agents) -> AgentData {
        AgentData {
            uuid: u.uuid.clone(),
            owner: u.owner.clone(),
        }
    }
}

impl Agents {
    pub async fn all(
        pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<Vec<AgentData>, Error> {
        let agents: Vec<Agents> =
            sqlx::query_as!(Agents, "SELECT uuid, owner FROM agents ORDER by uuid")
                .fetch_all(pool)
                .await?;
        let agents_responses = agents.iter().map(|t| t.into()).collect();

        Ok(agents_responses)
    }

    pub async fn get_by_uuid(
        pool: &Pool<ConnectionManager<PgConnection>>,
        uuid: String,
    ) -> Result<Vec<AgentData>, Error> {
        let agent = sqlx::query_as!(
            Agents,
            "SELECT uuid, owner from agents WHERE uuid = $1",
            uuid
        )
        .fetch_all(pool)
        .await?;
        let agent_response = agent.iter().map(|t| t.into()).collect();

        Ok(agent_response)
    }

    pub async fn get_by_owner(
        pool: &Pool<ConnectionManager<PgConnection>>,
        owner: i32,
    ) -> Result<Vec<AgentData>, Error> {
        let agents = sqlx::query_as!(
            Agents,
            "SELECT uuid, owner from agents WHERE owner = $1",
            owner
        )
        .fetch_all(pool)
        .await?;
        let agents_responses = agents.iter().map(|t| t.into()).collect();

        Ok(agents_responses)
    }

    pub async fn add(
        pool: &Pool<ConnectionManager<PgConnection>>,
        uuid: String,
        owner: i32,
    ) -> Result<AgentData, Error> {
        let agent = sqlx::query_as!(
            Agents,
            "INSERT INTO agents (uuid, owner) VALUES ( $1, $2 ) RETURNING uuid, owner ",
            uuid,
            owner
        )
        .fetch_one(pool)
        .await?;

        Ok(agent.into())
    }

    pub async fn update(
        pool: &Pool<ConnectionManager<PgConnection>>,
        uuid: String,
        owner: i32,
    ) -> Result<AgentData, Error> {
        sqlx::query_as!(
            Agents,
            "UPDATE agents SET owner = $1 WHERE uuid = $2 ",
            owner,
            uuid
        )
        .fetch_one(pool)
        .await?;

        Ok(Agents::default().into())
    }

    pub async fn delete(
        pool: &Pool<ConnectionManager<PgConnection>>,
        uuid: String,
    ) -> Result<AgentData, Error> {
        sqlx::query_as!(Agents, "DELETE FROM agents WHERE uuid = $1", uuid)
            .fetch_one(pool)
            .await?;

        Ok(Agents::default().into())
    }
}

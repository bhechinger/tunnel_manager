use crate::api::AgentData;
use sqlx::postgres::PgPool;
use sqlx::Error;

#[derive(sqlx::FromRow, Debug, Clone)]
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

impl Agents {
    pub async fn all(pool: &PgPool) -> Result<Vec<AgentData>, Error> {
        let agents: Vec<Agents> =
            sqlx::query_as!(Agents, "SELECT uuid, owner FROM agents ORDER by uuid")
                .fetch_all(pool)
                .await?;
        let agents_responses = agents.iter().map(|t| t.into_response()).collect();

        Ok(agents_responses)
    }

    pub async fn get_by_uuid(pool: &PgPool, uuid: String) -> Result<Vec<AgentData>, Error> {
        let agent = sqlx::query_as!(
            Agents,
            "SELECT uuid, owner from agents WHERE uuid = $1",
            uuid
        )
        .fetch_all(pool)
        .await?;
        let agent_response = agent.iter().map(|t| t.into_response()).collect();

        Ok(agent_response)
    }

    pub async fn get_by_owner(pool: &PgPool, owner: i32) -> Result<Vec<AgentData>, Error> {
        let agents = sqlx::query_as!(
            Agents,
            "SELECT uuid, owner from agents WHERE owner = $1",
            owner
        )
        .fetch_all(pool)
        .await?;
        let agents_responses = agents.iter().map(|t| t.into_response()).collect();

        Ok(agents_responses)
    }

    pub async fn add(pool: &PgPool, uuid: String, owner: i32) -> Result<AgentData, Error> {
        let agent = sqlx::query_as!(
            Agents,
            "INSERT INTO agents (uuid, owner) VALUES ( $1, $2 ) RETURNING uuid, owner ",
            uuid,
            owner
        )
        .fetch_one(pool)
        .await?;

        Ok(agent.into_response())
    }

    pub async fn update(pool: &PgPool, uuid: String, owner: i32) -> Result<AgentData, Error> {
        sqlx::query_as!(
            Agents,
            "UPDATE agents SET owner = $1 WHERE uuid = $2 ",
            owner,
            uuid
        )
        .fetch_one(pool)
        .await?;

        Ok(Agents::default().into_response())
    }

    pub async fn delete(pool: &PgPool, uuid: String) -> Result<AgentData, Error> {
        sqlx::query_as!(Agents, "DELETE FROM agents WHERE uuid = $1", uuid)
            .fetch_one(pool)
            .await?;

        Ok(Agents::default().into_response())
    }

    fn into_response(&self) -> AgentData {
        AgentData {
            uuid: self.uuid.clone(),
            owner: self.owner,
        }
    }
}

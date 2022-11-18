use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use tonic::Status;
use tracing::{info, instrument};

use crate::api::PermissionData;
use crate::storage::helpers::sql_err_to_grpc_error;

pub struct Permissions {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl Default for Permissions {
    fn default() -> Permissions {
        Permissions {
            id: 0,
            name: "".to_string(),
            description: "".to_string(),
        }
    }
}

impl From<Permissions> for PermissionData {
    fn from(p: Permissions) -> PermissionData {
        PermissionData {
            id: p.id,
            name: p.name,
            description: p.description,
        }
    }
}

impl From<&Permissions> for PermissionData {
    fn from(p: &Permissions) -> PermissionData {
        PermissionData {
            id: p.id.clone(),
            name: p.name.clone(),
            description: p.description.clone(),
        }
    }
}

impl Permissions {
    #[instrument]
    pub async fn all(
        pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<Vec<PermissionData>, Status> {
        match sqlx::query_as!(
            Permissions,
            "SELECT id, name, description FROM permissions ORDER by id"
        )
        .fetch_all(pool)
        .await
        {
            Ok(r) => Ok(r.iter().map(|t| t.into()).collect()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn get_by_id(
        pool: &Pool<ConnectionManager<PgConnection>>,
        id: i32,
    ) -> Result<PermissionData, Status> {
        match sqlx::query_as!(
            Permissions,
            "SELECT id, name, description from permissions WHERE id = $1",
            id
        )
        .fetch_one(pool)
        .await
        {
            Ok(r) => Ok(r.into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn get_by_name(
        pool: &Pool<ConnectionManager<PgConnection>>,
        name: String,
    ) -> Result<PermissionData, Status> {
        match sqlx::query_as!(
            Permissions,
            "SELECT id, name, description from permissions WHERE name = $1",
            name
        )
        .fetch_one(pool)
        .await
        {
            Ok(r) => Ok(r.into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn add(
        pool: &Pool<ConnectionManager<PgConnection>>,
        name: String,
        description: String,
    ) -> Result<PermissionData, Status> {
        match sqlx::query_as!(
            Permissions,
            "INSERT INTO permissions (name, description) VALUES ( $1, $2 ) RETURNING id, name, description",
            name,
            description
        )
            .fetch_one(pool)
            .await
        {
            Ok(r) => Ok(r.into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn update(
        pool: &Pool<ConnectionManager<PgConnection>>,
        id: i32,
        name: String,
        description: String,
    ) -> Result<PermissionData, Status> {
        let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE permissions SET ");
        let mut missing_args = false;

        match (name.clone(), description.clone()) {
            (a, b) if a != "" && b == "" => {
                builder.push("name = ");
                builder.push_bind(name.clone());
            }
            (a, b) if a == "" && b != "" => {
                builder.push("description = ");
                builder.push_bind(description.clone());
            }
            (a, b) if a != "" && b != "" => {
                builder.push("name = ");
                builder.push_bind(name.clone());
                builder.push(", description = ");
                builder.push_bind(description.clone());
            }
            _ => missing_args = true,
        }

        if missing_args {
            return Err(Status::invalid_argument("name and/or description required"));
        }

        builder.push("WHERE id =");
        builder.push_bind(id);
        let sql = builder.sql();

        info!(sql);

        match sqlx::query_with(sql).fetch_one(pool).await {
            Ok(_) => Ok(Permissions::default().into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn delete_by_id(
        pool: &Pool<ConnectionManager<PgConnection>>,
        id: i32,
    ) -> Result<PermissionData, Status> {
        match sqlx::query_as!(Users, "DELETE FROM permissions WHERE id = $1", id)
            .fetch_one(pool)
            .await
        {
            Ok(_) => Ok(Permissions::default().into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn delete_by_name(
        pool: &Pool<ConnectionManager<PgConnection>>,
        name: String,
    ) -> Result<PermissionData, Status> {
        match sqlx::query_as!(Users, "DELETE FROM permissions WHERE name = $1", name)
            .fetch_one(pool)
            .await
        {
            Ok(_) => Ok(Permissions::default().into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }
}

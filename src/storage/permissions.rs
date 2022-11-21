use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tonic::Status;
use tracing::instrument;

use crate::api::permission_request::IdOrName;
use crate::api::PermissionData;
use crate::schema::permissions;
use crate::schema::permissions::dsl::*;
use crate::storage::helpers::sql_err_to_grpc_error;

#[derive(Queryable, Debug)]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Insertable)]
#[diesel(table_name = permissions)]
pub struct NewPermission<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

#[derive(AsChangeset, Default)]
#[diesel(table_name = permissions)]
pub struct UpdatePermission {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl Default for Permission {
    fn default() -> Permission {
        Permission {
            id: 0,
            name: "".to_string(),
            description: "".to_string(),
        }
    }
}

impl From<Permission> for PermissionData {
    fn from(p: Permission) -> PermissionData {
        PermissionData {
            id: p.id,
            name: p.name,
            description: p.description,
        }
    }
}

impl From<&Permission> for PermissionData {
    fn from(p: &Permission) -> PermissionData {
        PermissionData {
            id: p.id,
            name: p.name.clone(),
            description: p.description.clone(),
        }
    }
}

impl Permission {
    #[instrument]
    pub async fn all(pool: &Pool<ConnectionManager<PgConnection>>) -> Result<Vec<PermissionData>, Status> {
        let conn = &mut pool.get().unwrap();

        match permissions.load::<Permission>(conn) {
            Ok(results) => Ok(results.iter().map(|t| t.into()).collect()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn get(pool: &Pool<ConnectionManager<PgConnection>>, id_or_name: &IdOrName) -> Result<PermissionData, Status> {
        let conn = &mut pool.get().unwrap();

        match id_or_name {
            IdOrName::Id(user_id) => match permissions.find(user_id).first::<Permission>(conn) {
                Ok(results) => Ok(results.into()),
                Err(err) => Err(sql_err_to_grpc_error(err)),
            },
            IdOrName::Name(permission_name) => {
                match permissions.filter(name.eq(permission_name)).first::<Permission>(conn) {
                    Ok(results) => Ok(results.into()),
                    Err(err) => Err(sql_err_to_grpc_error(err)),
                }
            }
        }
    }

    #[instrument]
    pub async fn add(pool: &Pool<ConnectionManager<PgConnection>>, permission_name: &str, permission_description: &str) -> Result<PermissionData, Status> {
        let new_user = NewPermission { name: permission_name, description: permission_description };
        let conn = &mut pool.get().unwrap();

        match diesel::insert_into(permissions)
            .values(&new_user)
            .get_result::<Permission>(conn)
        {
            Ok(results) => Ok(results.into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn update(pool: &Pool<ConnectionManager<PgConnection>>, permission_data: Permission) -> Result<PermissionData, Status> {
        let conn = &mut pool.get().unwrap();

        let mut update = UpdatePermission::default();

        if !permission_data.name.is_empty() {
            update.name = Some(permission_data.name);
        }

        if !permission_data.description.is_empty() {
            update.description = Some(permission_data.description)
        }

        match diesel::update(permissions.find(permission_data.id))
            .set(update)
            .get_result::<Permission>(conn)
        {
            Ok(results) => Ok(results.into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn delete(
        pool: &Pool<ConnectionManager<PgConnection>>,
        id_or_name: IdOrName,
    ) -> Result<usize, Status> {
        let conn = &mut pool.get().unwrap();

        match id_or_name {
            IdOrName::Id(permission_id) => match diesel::delete(permissions.find(permission_id)).execute(conn) {
                Ok(results) => Ok(results),
                Err(err) => Err(sql_err_to_grpc_error(err)),
            },
            IdOrName::Name(permission_name) => {
                match diesel::delete(permissions.filter(name.eq(permission_name))).execute(conn) {
                    Ok(results) => Ok(results),
                    Err(err) => Err(sql_err_to_grpc_error(err)),
                }
            }
        }
    }
}

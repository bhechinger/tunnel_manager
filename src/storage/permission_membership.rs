use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use tonic::Status;
use tracing::instrument;

use crate::api::permission_membership_request::IdPermissionOrUserid;
use crate::api::PermissionMembershipData;
use crate::schema::permission_membership;
use crate::schema::permission_membership::dsl::*;
use crate::storage::helpers::sql_err_to_grpc_error;

#[derive(Queryable, Default, Debug)]
pub struct PermissionMembership {
    pub id: i32,
    pub permission: i32,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = permission_membership)]
pub struct NewPermissionMembership {
    pub permission: i32,
    pub user_id: i32,
}

#[derive(AsChangeset, Default)]
#[diesel(table_name = permission_membership)]
pub struct UpdatePermissionMembership {
    pub permission: Option<i32>,
    pub user_id: Option<i32>,
}

impl From<PermissionMembership> for PermissionMembershipData {
    fn from(p: PermissionMembership) -> PermissionMembershipData {
        PermissionMembershipData {
            id: Some(p.id),
            permission: p.permission,
            user_id: p.user_id,
        }
    }
}

impl From<&PermissionMembership> for PermissionMembershipData {
    fn from(p: &PermissionMembership) -> PermissionMembershipData {
        PermissionMembershipData {
            id: Some(p.id),
            permission: p.permission,
            user_id: p.user_id,
        }
    }
}

impl PermissionMembership {
    #[instrument]
    pub async fn all(
        pool: &Pool<ConnectionManager<PgConnection>>,
    ) -> Result<Vec<PermissionMembershipData>, Status> {
        let conn = &mut pool.get().unwrap();

        match permission_membership.load::<PermissionMembership>(conn) {
            Ok(results) => Ok(results.iter().map(|t| t.into()).collect()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn get(
        pool: &Pool<ConnectionManager<PgConnection>>,
        id_permission_or_userid: &IdPermissionOrUserid,
    ) -> Result<Vec<PermissionMembershipData>, Status> {
        let conn = &mut pool.get().unwrap();

        match id_permission_or_userid {
            IdPermissionOrUserid::Id(pm_id) => match permission_membership
                .find(pm_id)
                .load::<PermissionMembership>(conn)
            {
                Ok(results) => Ok(results.iter().map(|t| t.into()).collect()),
                Err(err) => Err(sql_err_to_grpc_error(err)),
            },
            IdPermissionOrUserid::Permission(pm_permission) => {
                match permission_membership
                    .filter(permission.eq(pm_permission))
                    .load::<PermissionMembership>(conn)
                {
                    Ok(results) => Ok(results.iter().map(|t| t.into()).collect()),
                    Err(err) => Err(sql_err_to_grpc_error(err)),
                }
            }
            IdPermissionOrUserid::UserId(pm_userid) => {
                match permission_membership
                    .filter(user_id.eq(pm_userid))
                    .load::<PermissionMembership>(conn)
                {
                    Ok(results) => Ok(results.iter().map(|t| t.into()).collect()),
                    Err(err) => Err(sql_err_to_grpc_error(err)),
                }
            }
        }
    }

    #[instrument]
    pub async fn add(
        pool: &Pool<ConnectionManager<PgConnection>>,
        pm_data: PermissionMembershipData,
    ) -> Result<PermissionMembershipData, Status> {
        let new_user = NewPermissionMembership {
            permission: pm_data.permission,
            user_id: pm_data.user_id,
        };
        let conn = &mut pool.get().unwrap();

        match diesel::insert_into(permission_membership)
            .values(&new_user)
            .get_result::<PermissionMembership>(conn)
        {
            Ok(results) => Ok(results.into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn update(
        pool: &Pool<ConnectionManager<PgConnection>>,
        pm_data: PermissionMembershipData,
    ) -> Result<PermissionMembershipData, Status> {
        let conn = &mut pool.get().unwrap();
        let mut update = UpdatePermissionMembership::default();

        if pm_data.permission != 0 {
            update.permission = Some(pm_data.permission)
        }

        if pm_data.user_id != 0 {
            update.user_id = Some(pm_data.user_id)
        }

        match diesel::update(permission_membership.find(pm_data.id.unwrap()))
            .set(update)
            .get_result::<PermissionMembership>(conn)
        {
            Ok(results) => Ok(results.into()),
            Err(err) => Err(sql_err_to_grpc_error(err)),
        }
    }

    #[instrument]
    pub async fn delete(
        pool: &Pool<ConnectionManager<PgConnection>>,
        id_permission_or_userid: IdPermissionOrUserid,
    ) -> Result<usize, Status> {
        let conn = &mut pool.get().unwrap();

        match id_permission_or_userid {
            IdPermissionOrUserid::Id(pm_id) => {
                match diesel::delete(permission_membership.find(pm_id)).execute(conn) {
                    Ok(results) => Ok(results),
                    Err(err) => Err(sql_err_to_grpc_error(err)),
                }
            }
            IdPermissionOrUserid::Permission(pm_permission) => {
                match diesel::delete(permission_membership.filter(permission.eq(pm_permission)))
                    .execute(conn)
                {
                    Ok(results) => Ok(results),
                    Err(err) => Err(sql_err_to_grpc_error(err)),
                }
            }
            IdPermissionOrUserid::UserId(pm_userid) => {
                match diesel::delete(permission_membership.filter(user_id.eq(pm_userid)))
                    .execute(conn)
                {
                    Ok(results) => Ok(results),
                    Err(err) => Err(sql_err_to_grpc_error(err)),
                }
            }
        }
    }
}

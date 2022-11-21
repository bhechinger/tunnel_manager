use diesel::prelude::*;

use crate::api::PermissionMembershipData;
use crate::schema::permission_membership;

#[derive(Queryable, Default)]
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
            id: p.id,
            permission: p.permission,
            user_id: p.user_id,
        }
    }
}

impl From<&PermissionMembership> for PermissionMembershipData {
    fn from(p: &PermissionMembership) -> PermissionMembershipData {
        PermissionMembershipData {
            id: p.id,
            permission: p.permission,
            user_id: p.user_id,
        }
    }
}

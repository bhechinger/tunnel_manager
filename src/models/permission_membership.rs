use diesel::prelude::*;

use crate::schema::permission_membership;

#[derive(Queryable)]
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

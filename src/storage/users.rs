use diesel::prelude::*;
// use tracing::{instrument, warn};

use crate::models::users::{User, IdOrEmail, NewUser};
use crate::schema::users::dsl::*;

impl User {
    // #[instrument]
    pub fn all(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
        users.load::<User>(conn)
    }

    // #[instrument]
    pub fn get(conn: &mut PgConnection, id_or_email: &IdOrEmail) -> QueryResult<User> {
        match id_or_email {
            IdOrEmail::Id(user_id) => users.find(user_id).first::<User>(conn),
            IdOrEmail::Email(user_email) => users.filter(email.eq(user_email)).first::<User>(conn),
        }
    }

    // #[instrument]
    pub async fn add(conn: &mut PgConnection, user_email: &str) -> QueryResult<User> {
        let new_user = NewUser { email: user_email };

        diesel::insert_into(users)
            .values(&new_user)
            .get_result(conn)
    }

    // #[instrument]
    pub fn update(conn: &mut PgConnection, user_data: User) -> QueryResult<User> {
        diesel::update(users.find(user_data.id))
            .set(email.eq(user_data.email))
            .get_result::<User>(conn)
    }

    // #[instrument]
    pub fn delete(conn: &mut PgConnection, id_or_email: IdOrEmail) -> QueryResult<usize> {
        match id_or_email {
            IdOrEmail::Id(user_id) => diesel::delete(users.find(user_id)).execute(conn)
            IdOrEmail::Email(user_email) => diesel::delete(users.filter(email.eq(user_email))).execute(conn)
        }
    }
}

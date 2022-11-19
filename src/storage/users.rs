use diesel::prelude::*;

use crate::api::UserData;
use crate::schema::users;
use crate::schema::users::dsl::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: &'a str,
}

pub enum IdOrEmail {
    Id(i32),
    Email(String),
}

impl Default for User {
    fn default() -> User {
        User {
            id: 0,
            email: "".to_string(),
        }
    }
}

impl From<User> for UserData {
    fn from(u: User) -> UserData {
        UserData {
            id: u.id,
            email: u.email,
        }
    }
}

impl From<&User> for UserData {
    fn from(u: &User) -> UserData {
        UserData {
            id: u.id.clone(),
            email: u.email.clone(),
        }
    }
}

impl User {
    // #[instrument]
    pub async fn all(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
        users.load::<User>(conn)
    }

    // #[instrument]
    pub async fn get(conn: &mut PgConnection, id_or_email: &IdOrEmail) -> QueryResult<User> {
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
    pub async fn update(conn: &mut PgConnection, user_data: User) -> QueryResult<User> {
        diesel::update(users.find(user_data.id))
            .set(email.eq(user_data.email))
            .get_result::<User>(conn)
    }

    // #[instrument]
    pub async fn delete(conn: &mut PgConnection, id_or_email: IdOrEmail) -> QueryResult<usize> {
        match id_or_email {
            IdOrEmail::Id(user_id) => diesel::delete(users.find(user_id)).execute(conn)
            IdOrEmail::Email(user_email) => diesel::delete(users.filter(email.eq(user_email))).execute(conn)
        }
    }
}
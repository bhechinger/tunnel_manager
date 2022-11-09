use crate::api::UserData;
use sqlx::postgres::PgPool;
use sqlx::Error;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Users {
    pub id: i32,
    pub email: String, // this is a foreign key to users
}

impl Default for Users {
    fn default() -> Users {
        Users {
            id: 0,
            email: "".to_string(),
        }
    }
}

impl Users {
    pub async fn all(pool: &PgPool) -> Result<Vec<UserData>, Error> {
        let users: Vec<Users> = sqlx::query_as!(Users, "SELECT id, email FROM users ORDER by id")
            .fetch_all(pool)
            .await?;
        let users_responses = users.iter().map(|t| t.into_response()).collect();
        println!("{:?}", users_responses);

        Ok(users_responses)
    }

    pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<UserData, Error> {
        let user = sqlx::query_as!(Users, "SELECT id, email from users WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        Ok(user.into_response())
    }

    pub async fn get_by_email(pool: &PgPool, email: String) -> Result<UserData, Error> {
        let user = sqlx::query_as!(Users, "SELECT id, email from users WHERE email = $1", email)
            .fetch_one(pool)
            .await?;

        Ok(user.into_response())
    }

    pub async fn add(pool: &PgPool, email: String) -> Result<UserData, Error> {
        let user = sqlx::query_as!(
            Users,
            "INSERT INTO users (email) VALUES ( $1 ) RETURNING id, email",
            email,
        )
        .fetch_one(pool)
        .await?;

        Ok(user.into_response())
    }

    pub async fn update(pool: &PgPool, id: i32, email: String) -> Result<UserData, Error> {
        sqlx::query_as!(
            Users,
            "UPDATE users SET email = $1 WHERE id = $2 ",
            email,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(Users::default().into_response())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<UserData, Error> {
        sqlx::query_as!(Users, "DELETE FROM users WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        Ok(Users::default().into_response())
    }

    fn into_response(&self) -> UserData {
        UserData {
            id: self.id,
            email: self.email.clone(),
        }
    }
}

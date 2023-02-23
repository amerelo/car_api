use crate::errors::Error;

use serde::{Deserialize, Serialize};

use sqlx::types::uuid::Uuid;
use sqlx::PgPool;
use tracing::debug;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NewUser {
    pub user_name: String,
    pub email: String,
    pub password: String,
}

pub async fn insert_user_in_table(pg_pool: &PgPool, user: &NewUser) -> Result<Uuid> {
    let user = sqlx::query!(
        r#"
        INSERT INTO users(user_name, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id
    "#,
        &user.user_name,
        &user.email,
        &user.password
    )
    .fetch_one(pg_pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::Database(db_err) if db_err.constraint().is_some() => {
            debug!("{}", db_err.message());
            Error::Conflict("user already exist".into())
        }
        err => err.into(),
    })?;

    Ok(user.id)
}

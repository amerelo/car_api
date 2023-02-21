use super::AppState;

use axum::extract::{Query, State};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use sqlx::types::{time::OffsetDateTime, uuid::Uuid};
use time::format_description::well_known::Rfc3339;

use crate::errors::Error;
use sqlx::postgres::PgPool;
pub type Result<T, E = Error> = ::std::result::Result<T, E>;

// use validator::Validate;

use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AuthenticableUser {
    pub email: String,
    pub password: String,
}

impl AuthenticableUser {
    pub async fn authenticate(&self, pool: &PgPool) -> Option<UserInfo> {
        let user = sqlx::query_as::<_, UserInfo>("SELECT * FROM users WHERE email=$1")
            .bind(&self.email)
            .fetch_optional(pool)
            .await
            .unwrap()?;

        Some(user)
        // match argon2::verify_encoded(&user.password, self.password.as_bytes()).unwrap() {
        //     true => Some(user),
        //     false => None,
        // }
    }
}

#[serde_with::serde_as]
#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct UserInfo {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
    // pub updated_at: PrimitiveDateTime,
}

pub async fn authenticate(
    // cookie_jar: CookieJar,
    State(state): State<Arc<AppState>>,
    Json(user): Json<AuthenticableUser>,
) -> impl IntoResponse {
    match user.authenticate(&state.pg_pool).await {
        Some(_) => {}
        None => {
            return Err((StatusCode::NOT_FOUND, "We couldn't connect you, please ensure that the login and password are correct before trying again"));
        }
    };

    Ok(StatusCode::OK)
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(user): Json<CreateUser>,
) -> Result<StatusCode> {
    // encrypt password

    sqlx::query("INSERT INTO users(username, email, password_hash) VALUES ($1, $2, $3)")
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password)
        .execute(&state.pg_pool)
        .await
        .map_err(|e| match e {
            // if dbe.constraint() == Some("users_email_key")
            sqlx::Error::Database(_dbe) => {
                Error::Conflict("error: create user invalid data".into())
            }
            _ => e.into(),
        })?;

    Ok(StatusCode::CREATED)
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub user_id: Uuid,
    pub username: Option<String>,
    pub password_hash: Option<String>,
    pub email: Option<String>,
}

pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Json(user): Json<UpdateUser>,
) -> impl IntoResponse {
    let response = sqlx::query(
        r#"
        UPDATE users
        SET (username, password_hash, email) = 
            (COALESCE($1, users.username), 
            COALESCE($2, users.password_hash), 
            COALESCE($3, users.email)) 
        WHERE user_id=$4
        "#,
    )
    .bind(&user.username)
    .bind(&user.password_hash)
    .bind(&user.email)
    .bind(&user.user_id)
    .execute(&state.pg_pool)
    .await;

    match response {
        Ok(val) => Ok((
            StatusCode::OK,
            format!("user updated with success {:?}", val),
        )),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("An error happened {:?}", err),
        )),
    }
}

#[derive(Deserialize)]
pub struct SelectedUserById {
    pub user_id: Uuid,
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Json(user): Json<SelectedUserById>,
) -> impl IntoResponse {
    let response = sqlx::query("DELETE FROM users WHERE user_id=$1")
        .bind(user.user_id)
        .execute(&state.pg_pool)
        .await;

    match response {
        Ok(val) => Ok((
            StatusCode::OK,
            format!("user deleted with success {:?}", val),
        )),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("An error happened {:?}", err),
        )),
    }
}

pub async fn get_user_by_id(
    State(state): State<Arc<AppState>>,
    user: Query<SelectedUserById>,
) -> impl IntoResponse {
    let user = sqlx::query_as::<_, UserInfo>("SELECT * FROM users WHERE user_id=$1")
        .bind(user.user_id)
        .fetch_one(&state.pg_pool)
        .await;

    match user {
        Ok(user) => Ok((StatusCode::OK, Json(user))),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("An error happened {:?}", err),
        )),
    }
}

#[derive(Deserialize)]
pub struct SelectedUserByEmail {
    pub email: String,
}

pub async fn get_user_by_email(
    State(state): State<Arc<AppState>>,
    user: Query<SelectedUserByEmail>,
) -> impl IntoResponse {
    let user = sqlx::query_as::<_, UserInfo>("SELECT * FROM users WHERE email=$1")
        .bind(&user.email)
        .fetch_one(&state.pg_pool)
        .await;

    match user {
        Ok(user) => Ok((StatusCode::OK, Json(user))),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("An error happened {:?}", err),
        )),
    }
}

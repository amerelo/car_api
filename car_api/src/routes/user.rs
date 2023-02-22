use super::AppState;
use crate::errors::Error;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use sqlx::types::{time::OffsetDateTime, uuid::Uuid};
use time::format_description::well_known::Rfc3339;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

// use validator::Validate;

use std::sync::Arc;

#[serde_with::serde_as]
#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
    // #[serde_as(as = "Rfc3339")]
    // pub updated_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(user): Json<CreateUser>,
) -> Result<StatusCode> {
    // encrypt password

    sqlx::query("INSERT INTO users(email, password_hash) VALUES ($1, $2)")
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
    pub id: Uuid,
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
        SET (password_hash, email) = 
            (
                COALESCE($1, users.password_hash),
                COALESCE($2, users.email)
            )
        WHERE id=$3
        "#,
    )
    .bind(&user.password_hash)
    .bind(&user.email)
    .bind(&user.id)
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

// getDetails

pub async fn get_user_by_id(
    State(state): State<Arc<AppState>>,
    user: Query<SelectedUserById>,
) -> impl IntoResponse {
    let user = sqlx::query_as::<_, UserInfo>("SELECT * FROM users WHERE id=$1")
        .bind(user.id)
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
        // TODO: find a way to return emty response in axum
        // Err(_) if true => Ok((StatusCode::OK, Json(serde_json::json!({})))),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("An error happened {:?}", err),
        )),
    }
}

#[derive(Deserialize)]
pub struct SelectedUserById {
    pub id: Uuid,
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Json(user): Json<SelectedUserById>,
) -> impl IntoResponse {
    let response = sqlx::query("DELETE FROM users WHERE id=$1")
        .bind(user.id)
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

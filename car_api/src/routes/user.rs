use super::AppState;
use crate::errors::Error;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use sqlx::types::uuid::Uuid;
use sqlx::PgPool;
use tracing::debug;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

use std::sync::Arc;

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

// #[derive(Deserialize)]
// pub struct UpdateUser {
//     pub id: Uuid,
//     pub password_hash: Option<String>,
//     pub email: Option<String>,
// }

// pub async fn update_user(
//     State(state): State<Arc<AppState>>,
//     Json(user): Json<UpdateUser>,
// ) -> impl IntoResponse {
//     let response = sqlx::query(
//         r#"
//         UPDATE users
//         SET (password_hash, email) =
//             (
//                 COALESCE($1, users.password_hash),
//                 COALESCE($2, users.email)
//             )
//         WHERE id=$3
//         "#,
//     )
//     .bind(&user.password_hash)
//     .bind(&user.email)
//     .bind(&user.id)
//     .execute(&state.pg_pool)
//     .await;

//     match response {
//         Ok(val) => Ok((
//             StatusCode::OK,
//             format!("user updated with success {:?}", val),
//         )),
//         Err(err) => Err((
//             StatusCode::INTERNAL_SERVER_ERROR,
//             format!("An error happened {:?}", err),
//         )),
//     }
// }

// #[derive(Deserialize)]
// pub struct SelectedUserByEmail {
//     pub email: String,
// }

// pub async fn get_user_by_email(
//     State(state): State<Arc<AppState>>,
//     user: Query<SelectedUserByEmail>,
// ) -> impl IntoResponse {
//     let user = sqlx::query_as::<_, UserInfo>("SELECT * FROM users WHERE email=$1")
//         .bind(&user.email)
//         .fetch_one(&state.pg_pool)
//         .await;

//     match user {
//         Ok(user) => Ok((StatusCode::OK, Json(user))),
//         // TODO: find a way to return emty response in axum
//         // Err(_) if true => Ok((StatusCode::OK, Json(serde_json::json!({})))),
//         Err(err) => Err((
//             StatusCode::INTERNAL_SERVER_ERROR,
//             format!("An error happened {:?}", err),
//         )),
//     }
// }

// #[derive(Deserialize)]
// pub struct SelectedUserById {
//     pub id: Uuid,
// }

// pub async fn delete_user(
//     State(state): State<Arc<AppState>>,
//     Json(user): Json<SelectedUserById>,
// ) -> impl IntoResponse {
//     let response = sqlx::query("DELETE FROM users WHERE id=$1")
//         .bind(user.id)
//         .execute(&state.pg_pool)
//         .await;

//     match response {
//         Ok(val) => Ok((
//             StatusCode::OK,
//             format!("user deleted with success {:?}", val),
//         )),
//         Err(err) => Err((
//             StatusCode::INTERNAL_SERVER_ERROR,
//             format!("An error happened {:?}", err),
//         )),
//     }
// }

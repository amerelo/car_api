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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateAccount {
    pub name: String,
    pub email: String,
    pub password: String,

    pub car_model: String,
    pub car_plate: String,

    pub iban: String,
    pub bank_country: String,
    pub account_holder: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}

pub async fn create_account(
    State(state): State<Arc<AppState>>,
    Json(user): Json<CreateUser>,
) -> Result<StatusCode> {
    // encrypt password

    Ok(StatusCode::CREATED)
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

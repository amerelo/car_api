use super::{
    authenticate::User,
    user::{insert_user_in_table, NewUser},
    AppState,
};
use crate::errors::Error;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use sqlx::types::{time::OffsetDateTime, uuid::Uuid};
use time::format_description::well_known::Rfc3339;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

// use validator::Validate;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NewCar {
    pub car_model: String,
    pub car_plate: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NewBankDetails {
    pub account_holder: String,
    pub bank_country: String,
    pub iban: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateAccount {
    pub user: NewUser,
    pub car_info: NewCar,
    pub bank_details: NewBankDetails,
}

pub async fn create_account(
    State(state): State<Arc<AppState>>,
    Json(new_account): Json<CreateAccount>,
) -> Result<StatusCode> {
    // encrypt password

    println!("start create account {:#?}", new_account);

    let user_id = insert_user_in_table(&state.pg_pool, &new_account.user).await?;

    println!("user created");

    let bank_details =
        insert_bank_details_in_table(&state.pg_pool, &user_id, &new_account.bank_details);

    let car = insert_car_in_table(&state.pg_pool, &user_id, &new_account.car_info);

    println!("car and bank details ready");

    tokio::try_join!(bank_details, car)?;

    Ok(StatusCode::CREATED)
}

pub async fn insert_car_in_table(pg_pool: &PgPool, user_id: &Uuid, account: &NewCar) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO car(user_id, plate, model)
        VALUES ($1, $2, $3)
    "#,
    )
    .bind(user_id)
    .bind(&account.car_plate)
    .bind(&account.car_model)
    .execute(pg_pool)
    .await?;

    Ok(())
}

pub async fn insert_bank_details_in_table(
    pg_pool: &PgPool,
    user_id: &Uuid,
    account: &NewBankDetails,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO bank_details(user_id, country, iban, account_holder)
        VALUES ($1, $2, $3, $4)
    "#,
    )
    .bind(user_id)
    .bind(&account.bank_country)
    .bind(&account.iban)
    .bind(&account.account_holder)
    .execute(pg_pool)
    .await?;

    Ok(())
}

pub async fn get_account_details(Extension(user): Extension<User>, pg_pool: &PgPool) {
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

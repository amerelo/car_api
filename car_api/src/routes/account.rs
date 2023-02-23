use super::{
    authenticate::User,
    user::{insert_user_in_table, NewUser},
    AppState,
};
use crate::errors::Error;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::types::uuid::Uuid;
use sqlx::PgPool;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

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

    let user_id = insert_user_in_table(&state.pg_pool, &new_account.user).await?;

    let bank_details =
        insert_bank_details_in_table(&state.pg_pool, &user_id, &new_account.bank_details);

    let car = insert_car_in_table(&state.pg_pool, &user_id, &new_account.car_info);

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountDetails {
    user: User,
    cars_info: Vec<CarInfo>,
    bank_details: BankDetailsInfo,
}

pub async fn get_account_details(
    Extension(user): Extension<User>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let cars = get_account_cars_info(&user, &state.pg_pool);
    let bank_details = get_account_bank_details(&user, &state.pg_pool);

    let (cars_info, bank_details) = tokio::try_join!(cars, bank_details).unwrap();

    let account_details = AccountDetails {
        user,
        cars_info,
        bank_details,
    };

    Json(account_details)
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct CarInfo {
    pub id: Uuid,
    pub model: String,
    pub plate: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub async fn get_account_cars_info(user: &User, pg_pool: &PgPool) -> Result<Vec<CarInfo>> {
    let cars_info = sqlx::query_as::<_, CarInfo>(
        r#"
        SELECT * 
        FROM car
        WHERE user_id=$1
    "#,
    )
    .bind(user.id)
    .fetch_all(pg_pool)
    .await?;

    Ok(cars_info)
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct BankDetailsInfo {
    pub id: Uuid,
    pub country: String,
    pub iban: String,
    pub account_holder: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub async fn get_account_bank_details(user: &User, pg_pool: &PgPool) -> Result<BankDetailsInfo> {
    let bank_details = sqlx::query_as::<_, BankDetailsInfo>(
        r#"
    SELECT * 
    FROM bank_details
    WHERE user_id=$1
"#,
    )
    .bind(user.id)
    .fetch_one(pg_pool)
    .await?;

    Ok(bank_details)
}

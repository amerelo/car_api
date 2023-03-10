use super::AppState;

use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_login::{secrecy::SecretVec, AuthUser};
use serde::{Deserialize, Serialize};
use sqlx::types::uuid::Uuid;

type AuthContext = axum_login::extractors::AuthContext<User, axum_login::PostgresStore<User>>;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub user_name: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, sqlx::FromRow)]
pub struct LoginUser {
    pub email: String,
    pub password_hash: String,
}

impl AuthUser for User {
    fn get_id(&self) -> String {
        format!("{}", self.id)
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password_hash.clone().into())
    }
}

pub async fn login_handler(
    mut auth: AuthContext,
    State(state): State<Arc<AppState>>,
    Json(user): Json<LoginUser>,
) -> impl IntoResponse {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email=$1")
        .bind(&user.email)
        .fetch_optional(&state.pg_pool)
        .await
        .unwrap();

    match user {
        Some(user) => match auth.login(&user).await {
            Ok(_) => Ok("User logged in".to_string()),
            Err(_) => Err((StatusCode::UNAUTHORIZED, "Couldn't login user")),
        },
        None => Err((StatusCode::UNAUTHORIZED, "Couldn't login user")),
    }
}

pub async fn logout_handler(mut auth: AuthContext) {
    auth.logout().await;
}

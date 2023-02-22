use super::AppState;

use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use axum_login::{
    axum_sessions::{async_session::MemoryStore as SessionMemoryStore, SessionLayer},
    memory_store::MemoryStore as AuthMemoryStore,
    secrecy::SecretVec,
    AuthLayer, AuthUser, RequireAuthorizationLayer,
};
use serde::{Deserialize, Serialize};
use sqlx::types::uuid::Uuid;

type AuthContext = axum_login::extractors::AuthContext<User, axum_login::PostgresStore<User>>;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
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

// impl LoginUser {
//     pub async fn authenticate(&self, pool: &PgPool) -> Option<User> {
//         let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email=$1")
//             .bind(&self.email)
//             .fetch_optional(pool)
//             .await
//             .unwrap()?;

//         Some(user)
//         // match argon2::verify_encoded(&user.password, self.password.as_bytes()).unwrap() {
//         //     true => Some(user),
//         //     false => None,
//         // }
//     }
// }

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
            Ok(_) => Ok(format!("User logged in")),
            Err(_) => Err((StatusCode::UNAUTHORIZED, "Couldn't login user")),
        },
        None => Err((StatusCode::UNAUTHORIZED, "Couldn't login user")),
    }
}

pub async fn logout_handler(mut auth: AuthContext) {
    auth.logout().await;
}

pub async fn protected_handler(Extension(user): Extension<User>) -> impl IntoResponse {
    format!("Logged in as: {}", user.email)
}

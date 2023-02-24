use crate::routes::{account::*, authenticate::*, health_check::*, AppState};

use std::net::TcpListener;
use std::sync::Arc;

use axum::{
    http::{StatusCode, Uri},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_login::{
    axum_sessions::{async_session::MemoryStore as SessionMemoryStore, SessionLayer},
    AuthLayer, RequireAuthorizationLayer,
};
use rand::Rng;
use sqlx::PgPool;
use tracing::error;

pub async fn run(listener: TcpListener, pg_pool: PgPool) -> std::io::Result<()> {
    let secret = rand::thread_rng().gen::<[u8; 64]>();
    let session_store = SessionMemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret);

    let user_store = axum_login::PostgresStore::<User>::new(pg_pool.clone())
        .with_query("SELECT * FROM users WHERE id::text = $1");
    let auth_layer = AuthLayer::new(user_store, &secret);

    let shared_state = Arc::new(AppState { pg_pool });

    let app = Router::new()
        .route("/api/account", get(get_account_details))
        .route_layer(RequireAuthorizationLayer::<User>::login())
        .route("/api/account", post(create_account))
        .route("/login", post(login_handler))
        .route("/logout", get(logout_handler))
        .route("/health_check", get(health_check))
        .layer(auth_layer)
        .layer(session_layer)
        .fallback(fallback_handler)
        .with_state(shared_state);

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .with_graceful_shutdown(signal_shutdown())
        .await
        .unwrap();

    Ok(())
}

async fn signal_shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
}

#[tracing::instrument]
async fn fallback_handler(uri: Uri) -> impl IntoResponse {
    error!("No route for {}", uri);
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}

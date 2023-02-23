use crate::database::get_pg_pool;
use crate::routes::account::create_account;
use crate::routes::{authenticate::*, health_check::*, user::*, AppState};

use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::sync::Arc;

use axum::{
    http::{StatusCode, Uri},
    response::IntoResponse,
    routing::{get, post, put},
    Router,
};

use tracing::{debug, error};

use axum_login::{
    axum_sessions::{async_session::MemoryStore as SessionMemoryStore, SessionLayer},
    AuthLayer, RequireAuthorizationLayer,
};
use rand::Rng;

pub async fn run() -> std::io::Result<()> {
    let port = match env::var("PORT") {
        Ok(p) => p.parse::<u16>().unwrap(),
        _ => 8080,
    };
    // initialize tracing
    tracing_subscriber::fmt::init();

    let pg_pool = get_pg_pool().await;

    let secret = rand::thread_rng().gen::<[u8; 64]>();

    let session_store = SessionMemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret).with_secure(false);

    let user_store = axum_login::PostgresStore::<User>::new(pg_pool.clone())
        .with_query("SELECT * FROM users WHERE id::text = $1");
    let auth_layer = AuthLayer::new(user_store, &secret);

    let shared_state = Arc::new(AppState { pg_pool });

    let app = Router::new()
        // example
        .route("/protected", get(protected_handler))
        // get details
        .route("/api/get_details", get(protected_handler))
        .route(
            "/api/user",
            put(update_user).get(get_user_by_email).delete(delete_user),
        )
        .route_layer(RequireAuthorizationLayer::<User>::login())
        .route("/api/account", post(create_account))
        // .route("/api/user", post(create_user))
        .route("/api/login", post(login_handler))
        .route("/api/logout", get(logout_handler))
        .route("/health_check", get(health_check))
        //
        .layer(auth_layer)
        .layer(session_layer)
        .with_state(shared_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
    debug!("listening on {}", addr);
    let listener = TcpListener::bind(&addr).unwrap();

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

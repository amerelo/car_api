use axum::{
    http::{StatusCode, Uri},
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use tracing::{debug, error, info};

use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::sync::Arc;

use crate::database::get_pg_pool;
use crate::routes::{health_check::*, user::*, AppState};

pub async fn run() -> std::io::Result<()> {
    let port = match env::var("PORT") {
        Ok(p) => p.parse::<u16>(),
        _ => Ok(8080),
    }
    .unwrap();

    // initialize tracing
    tracing_subscriber::fmt::init();

    let shared_state = Arc::new(AppState {
        pg_pool: get_pg_pool().await,
    });

    // build our application with a route
    let app = Router::new()
        // .route("/", get(root))
        .route("/health_check", get(health_check))
        .route(
            "/api/user",
            post(create_user)
                .put(update_user)
                .get(get_user_by_email)
                .delete(delete_user),
        )
        .route("/api/authenticate", post(authenticate))
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
    println!("signal shutdown");
}

#[tracing::instrument]
async fn fallback_handler(uri: Uri) -> impl IntoResponse {
    error!("No route for {}", uri);
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}

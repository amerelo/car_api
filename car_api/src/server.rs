use crate::{database::get_pg_pool, routes};

use axum::{
    routing::{get, post},
    Router,
};

use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use crate::routes::{note::*, user::*, AppState};

pub async fn run() {
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
        .route("/", get(root))
        // .route(
        //     "/api/note",
        //     post(create_note)
        //         .put(update_note)
        //         .get(get_user_notes)
        //         .delete(delete_note),
        // )
        .route("/api/note/tags/", get(get_user_notes_by_tags))
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

    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// #[axum::async_trait]
// impl<B, T> FromRequest<B> for Qs<T>
// where
//     T: serde::de::DeserializeOwned,
// {
//     type Rejection = Infallible;

//     async fn from_request(req: &mut RequestParts) -> Result<Self, Self::Rejection> {
//         // TODO: error handling
//         let query = req.uri().query().unwrap();
//         Ok(Self(serde_qs::from_str(query).unwrap()))
//     }
// }

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

mod database;
mod startup;

mod encrypt;
mod errors;
mod routes;

use database::get_pg_pool;
use startup::run;

use std::{env, net::TcpListener};
use tracing::debug;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    // initialize tracing
    tracing_subscriber::fmt::init();

    // sqlx::migrate!("./migrations")
    //     .run(&connection_pool)
    //     .await
    //     .expect("Failed to migrate the database");

    let port = match env::var("PORT") {
        Ok(p) => p.parse::<u16>().unwrap(),
        _ => 8080,
    };
    let address = format!("127.0.0.1:{}", port);
    debug!("listening on {}", address);
    let listener = TcpListener::bind(&address).unwrap();

    let pg_pool = get_pg_pool().await;

    run(listener, pg_pool).await
}

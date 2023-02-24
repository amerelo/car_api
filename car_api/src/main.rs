mod database;
mod startup;

mod encrypt;
mod errors;
mod routes;

use database::get_pg_pool;
use startup::run;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::{env, net::TcpListener};
use tracing::debug;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    // initialize tracing
    tracing_subscriber::fmt::init();

    let port = match env::var("PORT") {
        Ok(p) => p.parse::<u16>().unwrap(),
        _ => 8080,
    };

    // create SocketAddr this way in order to be available form docker
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
    debug!("listening on {}", address);
    let listener = TcpListener::bind(&address).unwrap();

    let pg_pool = get_pg_pool().await;

    run(listener, pg_pool).await
}

mod database;
mod server;

mod errors;
mod routes;

use server::run;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    run().await;
}

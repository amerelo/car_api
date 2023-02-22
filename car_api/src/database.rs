use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

/// Returns a postgres pool from the user env.
pub async fn get_pg_pool() -> PgPool {
    let connect_options = PgConnectOptions::new()
        .host("host.docker.internal")
        .database(&std::env::var("POSTGRES_DB").expect("No schema defined in .env"))
        .username(&std::env::var("POSTGRES_USER").expect("No user defined in .env"))
        .password(&std::env::var("POSTGRES_PASSWORD").expect("No password defined in .env"));

    PgPoolOptions::new()
        .max_connections(15)
        .connect_with(connect_options)
        .await
        .expect("failed connection to database ")
}

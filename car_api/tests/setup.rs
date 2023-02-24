use sqlx::postgres::PgConnectOptions;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

use car_api::startup::run;

pub struct TestApp {
    pub database_name: String,
    pub address: String,
    pub pg_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    dotenv::dotenv().ok();

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(database_name.clone()).await;

    let server = run(listener, connection_pool.clone());

    let _ = tokio::spawn(server);

    TestApp {
        database_name,
        address,
        pg_pool: connection_pool,
    }
}

pub async fn configure_database(database_name: String) -> PgPool {
    let mut connection = PgConnection::connect_with(&without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, database_name))
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect_with(with_db(&database_name))
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

pub fn without_db() -> PgConnectOptions {
    PgConnectOptions::new()
        .host("host.docker.internal")
        .username(&std::env::var("POSTGRES_USER").unwrap())
        .password(&std::env::var("POSTGRES_PASSWORD").unwrap())
        .database("postgres")
        .port(5432)
}

pub fn with_db(database_name: &str) -> PgConnectOptions {
    PgConnectOptions::new()
        .host("host.docker.internal")
        .username(&std::env::var("POSTGRES_USER").unwrap())
        .password(&std::env::var("POSTGRES_PASSWORD").unwrap())
        .database(database_name)
        .port(5432)
}

use tokio_postgres::{Client, NoTls};
use std::env;

pub async fn establish_connection() -> Client {
    let url = &env::var("DATABASE_URL").expect("DATABASE_URL` environment variable must be set");

    let (client, connection) = tokio_postgres::connect(url, NoTls)
        .await
        .unwrap();

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    client
}
use dotenv::dotenv;
use std::env;
use sqlx::{postgres::{PgConnectOptions, PgConnection, PgPool, PgSslMode, PgPoolOptions}, pool::PoolOptions, Postgres};
use tokio;
use tracing::info;


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    // println!("{}", db);

    let pg_id = env::var("PG_ID").unwrap();
    let pg_pw = env::var("PG_PW").unwrap();
    let pg_host = env::var("PG_HOST").unwrap();
    let pg_port = env::var("PG_PORT").unwrap();
    let pg_database = env::var("PG_DATABASE").unwrap();

    let str_db = &format!(
        "postgres://{}:{}@{}:{}/{}",
        pg_id, pg_pw, pg_host, pg_port, pg_database
    );

    let pool = PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&str_db)
                    .await?;

    migrates(&pool).await?;


    let row: (i64, ) = sqlx::query_as("SELECT $1")
                        .bind(150_i64)
                        .fetch_one(&pool)
                        .await?;
    
    assert_eq!(row.0, 150);
    Ok(())
}


async fn migrates(pool : &PgPool) -> Result<(), sqlx::Error> {

    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;

    info!("success migrations.");

    Ok(())

}
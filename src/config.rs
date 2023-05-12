use std::{
    env,
    process,
    sync::Arc
};

use spacedust::apis::configuration::Configuration;
use once_cell::sync::Lazy;
use reqwest_middleware::{Middleware, ClientWithMiddleware};
use tokio::sync::OnceCell;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

/// [`Configuration`] object for use in all API calls.
/// Sets API key and manages rate limit.
pub static CONFIGURATION: Lazy<Configuration> = Lazy::new(|| {
    let Ok(token) = env::var("TOKEN") else {
        eprintln!("TOKEN environment variable expected");
        process::exit(1);
    };

    let mut configuration = Configuration::new();
    configuration.bearer_access_token = Some(token);
    let middleware: Box<[Arc<dyn Middleware>]> = Box::new([Arc::new(crate::rate_limit::Middleware)]);
    configuration.client = ClientWithMiddleware::new(reqwest::Client::new(), middleware);
    configuration
});

static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();
/// Returns the global database pool to be used for all database operations
pub async fn get_global_db_pool() -> &'static Pool<Postgres> {
    DB_POOL.get_or_init(|| async {
        let Ok(database_url) = env::var("DATABASE_URL") else {
            eprintln!("DATABASE_URL environment variable expected");
            process::exit(1);
        };
        let Ok(pool) = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await 
        else {
            eprintln!("Database connection failed");
            process::exit(1);
        };
        pool
    }).await
}

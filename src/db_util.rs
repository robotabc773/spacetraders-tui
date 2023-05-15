#![allow(clippy::expect_used)]

use crate::config::get_global_db_pool;

pub async fn setup_database() {
    sqlx::migrate!()
        .run(get_global_db_pool().await)
        .await
        .expect("database migration");
}

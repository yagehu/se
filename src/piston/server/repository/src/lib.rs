mod namespace;

use std::time::Duration;

pub use namespace::{
    new_namespace_repository,
    InsertNamespaceParams,
    NamespaceRepository,
};

use config::Database;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};

pub async fn db_pool_from_config(
    config: &Database,
) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(1))
        .connect_with(
            PgConnectOptions::new()
                .host(&config.host)
                .port(config.port)
                .database(&config.name)
                .username(&config.username)
                .password(&config.password),
        )
        .await
}

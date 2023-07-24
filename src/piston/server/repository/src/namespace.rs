use async_trait::async_trait;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use model::Namespace;

pub fn new_namespace_repository(db_pool: PgPool) -> impl NamespaceRepository {
    Impl { db_pool }
}

#[async_trait]
pub trait NamespaceRepository {
    async fn insert(
        &self,
        p: InsertNamespaceParams<'async_trait>,
    ) -> Result<Namespace, sqlx::Error>;
}

#[derive(Clone, Debug)]
pub struct InsertNamespaceParams<'a> {
    pub id:         Uuid,
    pub created_at: OffsetDateTime,

    pub name: &'a str,
}

#[derive(Clone, Debug)]
struct Impl {
    db_pool: PgPool,
}

#[async_trait]
impl NamespaceRepository for Impl {
    async fn insert(
        &self,
        p: InsertNamespaceParams<'async_trait>,
    ) -> Result<Namespace, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO namespace
              ( id
              , created_at
              )
            VALUES ($1, $2)
            RETURNING *;
            "#,
        )
        .bind(p.id)
        .bind(p.created_at)
        .fetch_one(&self.db_pool)
        .await
    }
}

use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use uuid::Uuid;

use config::Database;
use model::Namespace;
use repository::{InsertNamespaceParams, NamespaceRepository};

#[tokio::test]
async fn insert() {
    let database: Database = cfg::new().unwrap();
    let db_pool = repository::db_pool_from_config(&database).await.unwrap();
    let namespace_repository = repository::new_namespace_repository(db_pool);
    let id = Uuid::new_v4();
    let name = "test";
    let created_at =
        OffsetDateTime::parse("1985-04-12T23:20:50.52Z", &Rfc3339).unwrap();

    let result = namespace_repository
        .insert(InsertNamespaceParams {
            id,
            created_at,
            name,
        })
        .await
        .unwrap();

    assert_eq!(
        result,
        Namespace {
            id,
            created_at,
            name: name.to_owned(),
        }
    );
}

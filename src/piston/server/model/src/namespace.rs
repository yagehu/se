use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Namespace {
    pub id:         Uuid,
    pub created_at: OffsetDateTime,

    pub name: String,
}

impl PartialEq for Namespace {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

impl Eq for Namespace {
}

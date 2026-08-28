use chrono::Utc;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tokens")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub token_id: Uuid,
    pub token: String,
    pub user_id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Model {
    pub fn new(token: String, user_id: Uuid) -> Self {
        let now = Utc::now().naive_utc();
        let token_id = Uuid::now_v7();

        Self {
            token_id,
            token,
            user_id,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

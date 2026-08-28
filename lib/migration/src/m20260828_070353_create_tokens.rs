use sea_orm_migration::{prelude::*, schema::*};

use crate::entities::Tokens;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Tokens::Table)
                    .col(uuid(Tokens::TokenId).primary_key())
                    .col(uuid(Tokens::UserId))
                    .col(text(Tokens::Token))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("tokens_token_user_id_udx")
                    .table(Tokens::Table)
                    .col(Tokens::Token)
                    .col(Tokens::UserId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tokens::Table).to_owned())
            .await
    }
}

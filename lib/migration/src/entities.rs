use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Tokens {
    Table,
    TokenId,
    Token,
    UserId,
}

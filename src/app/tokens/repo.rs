use sea_orm::{
    ColumnTrait as _, Condition, ConnectionTrait, EntityTrait as _, IntoActiveModel as _,
    QueryFilter as _,
};

use crate::app::error::AppError;

mod token;

pub type TokenModel = token::Model;

pub async fn create_stream<T: ConnectionTrait>(
    db: &T,
    model: TokenModel,
) -> Result<TokenModel, AppError> {
    token::Entity::insert(model.clone().into_active_model())
        .on_conflict_do_nothing_on([token::Column::Token, token::Column::UserId])
        .exec(db)
        .await?;

    let token = token::Entity::find()
        .filter(
            Condition::all()
                .add(token::Column::Token.eq(model.token))
                .add(token::Column::UserId.eq(model.user_id)),
        )
        .one(db)
        .await?
        .ok_or(AppError::Unreachable)?;

    Ok(token)
}

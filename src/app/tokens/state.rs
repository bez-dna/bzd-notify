use crate::app::db::DbState;

#[derive(Clone)]
pub struct TokensState {
    pub db: DbState,
}

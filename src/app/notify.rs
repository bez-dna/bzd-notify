use bzd_lib::error::Error;
use tokio::try_join;

use crate::app::state::AppState;

mod messaging;
mod service;
pub mod settings;
pub mod state;

pub async fn messaging(state: &AppState) -> Result<(), Error> {
    try_join!(messaging::messages(state.notify.clone()),)?;

    Ok(())
}

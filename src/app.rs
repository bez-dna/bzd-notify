use axum::Router;
use bzd_lib::{
    error::Error,
    settings::{HttpSettings, Settings as _},
};
use tokio::try_join;
use tonic::service::Routes;
use tracing::info;

use crate::app::{settings::AppSettings, state::AppState};

mod db;
mod error;
mod mess;
mod notify;
mod settings;
mod state;
mod tokens;

pub async fn run() -> Result<(), Error> {
    let settings = AppSettings::new()?;
    let state = AppState::new(&settings).await?;

    try_join!(http_and_grpc(&state, &settings.http), messaging(&state))?;

    Ok(())
}

async fn http_and_grpc(state: &AppState, settings: &HttpSettings) -> Result<(), Error> {
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(tonic_health::pb::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(bzd_notify_api::tokens::FILE_DESCRIPTOR_SET)
        .build_v1alpha()?;

    let (_, health_service) = tonic_health::server::health_reporter();

    let router = Router::new().with_state(());
    let routes = Routes::from(router);
    let router = routes
        .add_service(reflection_service)
        .add_service(health_service)
        .add_service(tokens::service(state))
        .into_axum_router();

    let listener = tokio::net::TcpListener::bind(&settings.endpoint).await?;

    info!("app: started on {}", listener.local_addr()?);
    axum::serve(listener, router).await?;

    Ok(())
}

async fn messaging(state: &AppState) -> Result<(), Error> {
    info!("messaging: started");
    notify::messaging(state).await?;

    Ok(())
}

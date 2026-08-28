use bzd_notify_api::tokens::tokens_service_server::TokensServiceServer;

use crate::app::{state::AppState, tokens::grpc::GrpcTokensService};

mod grpc;
mod repo;
mod service;
pub mod state;

pub fn service(state: &AppState) -> TokensServiceServer<GrpcTokensService> {
    TokensServiceServer::new(GrpcTokensService::new(state.tokens.clone()))
}

//! Sample application for testing + validating GRPC functionality on the FL0 platform.

use std::{error::Error, net::SocketAddr};

use tokio::signal::unix::SignalKind;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
use crate::api::canary::canary_server::CanaryServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "grpc_canary=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
        .init();

    let port = std::env::var("PORT")
        .unwrap_or_default()
        .parse()
        .unwrap_or(8080);

    let channel_capacity = std::env::var("CHANNEL_CAPACITY")
        .unwrap_or_default()
        .parse()
        .unwrap_or(10);

    tracing::info!(?port, "Starting up...");

    let server = api::GrpcServer::new(channel_capacity);
    let grpc_canary = CanaryServer::new(server);

    let cancel_token = CancellationToken::new();
    let server_token = cancel_token.child_token();

    let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate()).map_err(|err| {
        tracing::error!(error=?err, "Couldn't instantiate signal handler!");
        err
    })?;

    let _termination_handler = tokio::spawn(async move {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => { tracing::info!("SIGINT/CTRL-C signal received! Cancelling tasks"); cancel_token.cancel();},
            _ =  sigterm.recv() => { tracing::info!("SIGTERM signal received! Cancelling tasks"); cancel_token.cancel();}

        }
    });
    let addr = if std::env::var("ENVIRONMENT").is_ok_and(|e| e == "PROD") {
        SocketAddr::from(([0, 0, 0, 0], port))
    } else {
        SocketAddr::from(([127, 0, 0, 1], port))
    };

    tracing::info!(?addr, "Starting server!");
    tonic::transport::Server::builder()
        .add_service(grpc_canary)
        .serve_with_shutdown(addr, server_token.cancelled())
        .await?;

    Ok(())
}

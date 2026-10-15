// SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use any2nix_website::app;
use std::env;
use tokio::net::TcpListener;
#[cfg(feature = "trace")]
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let host = env::var("ANY2NIX_WEBSITE_HOST")
        .or_else(|_| env::var("HOST"))
        .unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = env::var("ANY2NIX_WEBSITE_PORT")
        .or_else(|_| env::var("PORT"))
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let listener = TcpListener::bind((host.as_str(), port)).await.unwrap();
    let app = app();

    #[cfg(feature = "trace")]
    {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                    format!(
                        "{}=debug,tower_http=debug,axum::rejection=trace",
                        env!("CARGO_CRATE_NAME")
                    )
                    .into()
                }),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
        tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    }

    axum::serve(listener, app).await.unwrap();
}

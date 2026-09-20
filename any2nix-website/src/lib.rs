// SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
//
// SPDX-License-Identifier: GPL-3.0-or-later

#![doc(html_favicon_url = "https://zipline.gs-101.dev/u/DF6up7.ico")]
#![doc(html_logo_url = "https://zipline.gs-101.dev/u/DZWGB0.svg")]
//! This crate provides a web application for translating different formats
//! to [Nix](https://nixos.org).
//!
//! # Supported Formats
//!
//! See [any2nix::Format] for an enumeration of supported formats.
//!
//! Each format is gated behind its own [feature](https://doc.rust-lang.org/cargo/reference/features.html).
//!
//! The default feature enables all formats.
//!
//! # Crate Features
//!
//! Besides the features for each format, this crate also exposes the following
//! features:
//!
//! - `api`: Enables an "/api" endpoint for interaction without a browser through [any2nix-api].
//! - `trace`: Enables tracing of incoming HTTP requests through [tower-http] and [tracing].
//! - `utoipa`: Enables OpenAPI schema generation through [utoipa].
//! - `utoipa-swagger-ui`: Enables the [Swagger UI](https://swagger.io/tools/swagger-ui/) for the API through [utoipa-swagger-ui].

pub(crate) mod embed;
pub(crate) mod templates;

use crate::embed::Dist;
use crate::templates::{ErrorSnackbar, IndexPage, Nix};
use any2nix::Format;
use any2nix_i18n::I18n;
use askama::Template;
use axum::{
    Form, Router,
    extract::Path,
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use rust_embed::Embed;
use serde::Deserialize;
#[cfg(feature = "trace")]
use tower_http::trace::TraceLayer;

#[derive(Deserialize)]
pub(crate) struct ConvertForm {
    input: String,
}

pub(crate) struct HtmlTemplate<T>(pub(crate) T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        let html = self.0.render().expect("template rendering is infallible");

        Html(html).into_response()
    }
}

#[derive(Debug)]
pub(crate) struct Error(any2nix::Error);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let template = ErrorSnackbar {
            error: self.0.to_string(),
        };

        (StatusCode::BAD_REQUEST, HtmlTemplate(template)).into_response()
    }
}

pub(crate) async fn get_root(i18n: I18n) -> HtmlTemplate<IndexPage> {
    HtmlTemplate(IndexPage {
        i18n,
        formats: Format::VARIANTS,
    })
}

// Helper for accessing embedded files from paths.
pub(crate) fn get_path<E: Embed>(_embed: E, path: String) -> impl IntoResponse {
    let path = path.trim_start_matches('/');

    match E::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub(crate) async fn get_assets(Path(path): Path<String>) -> impl IntoResponse {
    get_path(Dist, path)
}

pub(crate) async fn post_convert(
    i18n: I18n,
    Path(format): Path<Format>,
    Form(form): Form<ConvertForm>,
) -> Result<HtmlTemplate<Nix>, Error> {
    let nix = format.to_nix(&form.input).map_err(Error)?;
    let template = Nix { i18n, nix };

    Ok(HtmlTemplate(template))
}

pub fn app() -> Router {
    let router = Router::new()
        .route("/", get(get_root))
        .route("/assets/{*path}", get(get_assets))
        .route("/convert/{format}", post(post_convert));

    #[cfg(feature = "api")]
    let router = router.nest("/api", any2nix_api::app());

    #[cfg(feature = "trace")]
    let router = router.layer(TraceLayer::new_for_http());

    router
}

#[cfg(test)]
mod tests {
    use super::*;
    use any2nix_i18n::EN_US;

    #[tokio::test]
    async fn returns_index_page() {
        let response_status = get_root(EN_US).await.into_response().status();
        let expected = StatusCode::OK;

        assert_eq!(response_status, expected)
    }

    #[tokio::test]
    async fn errors_on_nonexistent_file() {
        let response_status = get_assets(Path("bogus".to_string()))
            .await
            .into_response()
            .status();
        let expected = StatusCode::NOT_FOUND;

        assert_eq!(response_status, expected)
    }

    #[tokio::test]
    async fn returns_index_css() {
        let response_status = get_assets(Path("css/index.css".to_string()))
            .await
            .into_response()
            .status();
        let expected = StatusCode::OK;

        assert_eq!(response_status, expected)
    }

    #[tokio::test]
    async fn returns_index_js() {
        let response_status = get_assets(Path("js/index.js".to_string()))
            .await
            .into_response()
            .status();
        let expected = StatusCode::OK;

        assert_eq!(response_status, expected)
    }

    #[cfg(feature = "ini")]
    #[tokio::test]
    async fn errors_on_invalid_format() {
        let request = ConvertForm {
            input: "[broken\nnonsense = \"".to_string(),
        };
        let response_status = post_convert(EN_US, Path(Format::Ini), Form(request))
            .await
            .into_response()
            .status();
        let expected = StatusCode::BAD_REQUEST;

        assert_eq!(response_status, expected)
    }

    #[cfg(feature = "ini")]
    #[tokio::test]
    async fn converts_valid_format() {
        let request = ConvertForm {
            input: "enable-mouse = no\n[dmenu]\nmode = index".to_string(),
        };
        let response_status = post_convert(EN_US, Path(Format::Ini), Form(request))
            .await
            .into_response()
            .status();
        let expected = StatusCode::OK;

        assert_eq!(response_status, expected)
    }

    #[tokio::test]
    async fn returns_functional_router() {
        let app = app();

        assert!(app.has_routes())
    }
}

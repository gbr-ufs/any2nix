// SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
//
// SPDX-License-Identifier: GPL-3.0-or-later

// TODO: Remove manual descriptions once utoipa supports docstring links.

#![doc(html_favicon_url = "https://zipline.gs-101.dev/u/DF6up7.ico")]
#![doc(html_logo_url = "https://zipline.gs-101.dev/u/DZWGB0.svg")]
//! This crate provides an [HTTP](https://httpwg.org/specs/) server to be used as
//! an [OpenAPI](https://www.openapis.org/)-compatible API for translating different
//! formats to [Nix](https://nixos.org).
//!
//! It exposes its own router for integration with other [axum]-based programs.
//!
//! # Usage
//!
//! This crate is on [crates.io](https://crates.io/crates/any2nix) and can
//! be added as a dependency of your project:
//!
//! ```bash
//! cargo add any2nix-api
//! ```
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
//! - `trace`: Enables tracing of incoming HTTP requests through [tower-http] and [tracing].
//! - `utoipa`: Enables OpenAPI schema generation through [utoipa].
//! - `utoipa-swagger-ui`: Enables the [Swagger UI](https://swagger.io/tools/swagger-ui/) for the API through [utoipa-swagger-ui].
//!
//! # Examples: Adding it as an API to another program
//!
//! 1. Add it as dependency of your program:
//!
//! ```bash
//! cargo add any2nix-api
//! ```
//!
//! 2. Add the router to your [axum] program through [nesting][axum::Router::nest]:
//!
//! ```rust,no_run
//! # #[tokio::main]
//! # async fn main() {
//! use axum::{routing::get, Router};
//! use tokio::net::TcpListener;
//!
//! let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
//! let api_router = any2nix_api::app();
//! let app = Router::new().route("/", get(|| async { "Hello, World!"})).nest("/api", api_router);
//!
//! axum::serve(listener, app).await.unwrap();
//! # }
//! ```
//!
//! # Examples: Sending TOML to the API
//!
//! ```bash
//! curl \
//! -d '{"input": "[package]\nname = \"any2nix\""}' \
//! -X POST \
//! -H "Content-Type: application/json" \
//! http://localhost:3000/v1/toml
//! # Output:
//! #
//! # {"nix":"{\n  package = {\n    name = \"any2nix\";\n  };\n}"}
//! ```

pub mod requests;
pub mod responses;

use crate::requests::ConvertRequest;
use crate::responses::{ConvertResponse, ErrorResponse, RootResponse};
use any2nix::Format;
use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
#[cfg(feature = "trace")]
use tower_http::trace::TraceLayer;
#[cfg(feature = "utoipa")]
use utoipa::OpenApi;

/// Error type of the API.
#[derive(Debug)]
pub struct Error(any2nix::Error);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = Json(ErrorResponse {
            error: self.0.to_string(),
        });
        (StatusCode::BAD_REQUEST, body).into_response()
    }
}

/// Root endpoint of the API. Returns crate information.
#[cfg_attr(
    feature = "utoipa",
    utoipa::path(
        get,
        path = "/",
        responses(
            (body = RootResponse, description = "Root endpoint of the API", status = 200)
        ),
        summary = "Root endpoint of the API",
    )
)]
pub async fn get_root() -> Json<RootResponse> {
    Json(RootResponse {
        description: env!("CARGO_PKG_DESCRIPTION").to_string(),
        #[cfg(feature = "utoipa-swagger-ui")]
        docs: "/docs".to_string(),
        name: env!("CARGO_PKG_NAME").to_string(),
        #[cfg(feature = "utoipa")]
        openapi: "/api-docs/openapi.json".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Converts the [supported formats][crate#supported-formats] to [Nix](https://nixos.org).
///
/// See [ConvertRequest] and [ConvertResponse] for the request and response
/// schema, respectively.
///
/// # Errors
///
/// Returns [Error::Conversion] in case something went wrong during the
/// conversion processs.
///
/// [axum::extract::Path].
#[cfg_attr(
    feature = "utoipa",
    utoipa::path(
        description = "Converts input data from the supported formats to Nix",
        path = "/v1/{format}",
                params(
            ("format" = Format, Path, description = "Source format to convert from")
        ),
        post,
        request_body = ConvertRequest,
        responses(
            (body = ConvertResponse, description = "Conversion successful", status = 200),
            (body = ErrorResponse, description = "Conversion error", status = 400),
            (description = "Unsupported format", status = 404)
        ),
        summary = "Converts the supported formats to Nix",
    )
)]
pub async fn post_format(
    Path(format): Path<Format>,
    Json(payload): Json<ConvertRequest>,
) -> Result<Json<ConvertResponse>, Error> {
    let nix = format.to_nix(&payload.input).map_err(Error)?;

    Ok(Json(ConvertResponse { nix }))
}

/// Generator of the program's router. Exposed publicly for integration with other
/// [axum]-based programs.
///
/// Exposes the "/v1/{format}" route, such that {format} is one of the
/// [supported formats][crate#supported-formats].
///
/// # Crate Features
///
/// With `utoipa-swagger-ui` enabled, it also exposes the "/docs" route for the
/// [Swagger UI](https://swagger.io/tools/swagger-ui/) of the API.
///
/// # Examples: Adding it as an API to another program through [nesting][axum::Router::nest]
///
///
/// ```rust,no_run
/// # #[tokio::main]
/// # async fn main() {
/// use axum::{routing::get, Router};
/// use tokio::net::TcpListener;
///
/// let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
/// let api_router = any2nix_api::app();
/// let app = Router::new().route("/", get(|| async { "Hello, World!"})).nest("/api", api_router);
///
/// axum::serve(listener, app).await.unwrap();
/// # }
/// ```
pub fn app() -> Router {
    let router = Router::new()
        .route("/", get(get_root))
        .route("/v1/{format}", post(post_format));
    #[cfg(feature = "utoipa-swagger-ui")]
    let router = {
        let config = utoipa_swagger_ui::Config::new(["/api-docs/openapi.json"]).use_base_layout();

        router.merge(
            utoipa_swagger_ui::SwaggerUi::new("/docs")
                .config(config)
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
    };

    #[cfg(feature = "trace")]
    let router = router.layer(TraceLayer::new_for_http());

    router
}

#[cfg(feature = "utoipa")]
#[derive(OpenApi)]
#[openapi(
    components(schemas(ConvertRequest, ConvertResponse, ErrorResponse, Format, RootResponse)),
    info(contact(
        name = "GitHub Issues",
        url = "https://github.com/gbr-ufs/any2nix/issues"
    )),
    paths(get_root, post_format)
)]
struct ApiDoc;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn returns_crate_information() {
        let response = get_root().await;

        assert_eq!(response.0.description, env!("CARGO_PKG_DESCRIPTION"));
        assert_eq!(response.0.name, env!("CARGO_PKG_NAME"));
        assert_eq!(response.0.version, env!("CARGO_PKG_VERSION"));
    }

    #[cfg(feature = "ini")]
    #[tokio::test]
    async fn errors_on_invalid_format() {
        let request = ConvertRequest {
            input: "[broken\nnonsense = \"".to_string(),
        };
        let response = post_format(Path(Format::Ini), Json(request))
            .await
            .into_response();
        let expected_status = StatusCode::BAD_REQUEST;
        let expected_body = Json(ErrorResponse {
            error: "INI parsing error: expected a key, found an unexpected character at line 2 column 1".to_string(),
        });
        let expected = (expected_status, expected_body).into_response();

        assert_eq!(response.status(), expected.status());
    }

    #[cfg(feature = "ini")]
    #[tokio::test]
    async fn converts_valid_format() {
        let request = ConvertRequest {
            input: "enable-mouse = no\n[dmenu]\nmode = index".to_string(),
        };
        let response = post_format(Path(Format::Ini), Json(request)).await.unwrap();
        let expected = r#"{
  dmenu = {
    mode = "index";
  };
  enable-mouse = "no";
}"#;

        assert_eq!(response.nix, expected);
    }

    #[tokio::test]
    async fn returns_functional_router() {
        let app = app();

        assert!(app.has_routes())
    }
}

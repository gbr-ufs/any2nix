// SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::convert::Infallible;

use axum::{extract::FromRequestParts, http::request::Parts};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct I18n {
    // Used in the button that appears under the conversion result.
    pub(crate) another_one: &'static str,
    // Message in the button used to attach a file for conversion.
    pub(crate) attach_file: &'static str,
    // Message that appears in the "copy to clipboard" button after it's
    // clicked.
    pub(crate) copied: &'static str,
    // Message in the "copy to clipboard" button.
    pub(crate) copy: &'static str,
    // Message in the button used to download the converted Nix expression.
    pub(crate) download_nix: &'static str,
    // Language code of the preferred language reported by the browser to
    // be used in the HTML.
    pub(crate) lang: &'static str,
    // Message that appears when JavaScript is unavailable.
    pub(crate) noscript: &'static str,
    // Text of the "skip to content" button.
    pub(crate) skip_to_content: &'static str,
    // Message of the button used to submit text for conversion.
    pub(crate) submit: &'static str,
}

pub(crate) const EN_US: I18n = I18n {
    another_one: "Another one?",
    attach_file: "Attach file",
    copied: "Copied!",
    copy: "Copy to clipboard",
    download_nix: "Download as .nix",
    lang: "en-US",
    noscript: "This website requires JavaScript",
    skip_to_content: "Skip to content",
    submit: "Submit",
};

pub(crate) const PT_BR: I18n = I18n {
    another_one: "Mais um?",
    attach_file: "Anexar arquivo",
    copied: "Copiado!",
    copy: "Copiar para a área de transferência",
    download_nix: "Baixar como .nix",
    lang: "pt-BR",
    noscript: "Este site requer JavaScript",
    skip_to_content: "Ir para o conteúdo principal",
    submit: "Enviar",
};

// Used to get the user's preferred language from the browser.
impl<S> FromRequestParts<S> for I18n
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Some(header) = parts
            .headers
            .get("Accept-Language")
            .and_then(|h| h.to_str().ok())
        else {
            return Ok(EN_US);
        };

        for entry in header.to_ascii_lowercase().split(',') {
            let language = entry.split(';').next().unwrap_or("").trim();

            match language {
                "pt-br" | "pt" => return Ok(PT_BR),
                "en-us" | "en" => return Ok(EN_US),
                _ => {}
            }
        }

        Ok(EN_US)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;

    async fn extract_i18n(header_value: Option<&str>) -> I18n {
        let mut builder = Request::builder();

        if let Some(val) = header_value {
            builder = builder.header("Accept-Language", val);
        }

        let (mut parts, _) = builder.body(()).unwrap().into_parts();

        I18n::from_request_parts(&mut parts, &()).await.unwrap()
    }

    #[tokio::test]
    async fn defaults_to_en_us_when_header_missing() {
        assert_eq!(extract_i18n(None).await, EN_US);
    }

    #[tokio::test]
    async fn extracts_pt_br() {
        assert_eq!(extract_i18n(Some("pt-BR")).await, PT_BR);
        assert_eq!(extract_i18n(Some("pt")).await, PT_BR);
    }

    #[tokio::test]
    async fn extracts_en_us() {
        assert_eq!(extract_i18n(Some("en-US")).await, EN_US);
        assert_eq!(extract_i18n(Some("en")).await, EN_US);
    }

    #[tokio::test]
    async fn handles_q_factors_and_lists() {
        assert_eq!(
            extract_i18n(Some("es-ES,pt-BR;q=0.9,en-US;q=0.8")).await,
            PT_BR
        );
    }

    #[tokio::test]
    async fn falls_back_to_en_us() {
        assert_eq!(extract_i18n(Some("fr-FR,de;q=0.9,ja")).await, EN_US);
    }
}

use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub enum Environment {
    Production,
    Staging,
}

pub struct BillplzClient {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: String,
    pub(crate) api_key: String,
    pub(crate) environment: Option<Environment>,
}

impl BillplzClient {
    pub fn new(environment: Environment, api_key: impl Into<String>) -> Self {
        let base_url = match &environment {
            Environment::Production => "https://www.billplz.com".to_string(),
            Environment::Staging => "https://www.billplz-sandbox.com".to_string(),
        };

        Self {
            http: reqwest::Client::new(),
            base_url,
            api_key: api_key.into(),
            environment: Some(environment),
        }
    }

    /// Create a client with a custom base URL (useful for testing with wiremock).
    pub fn with_base_url(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: base_url.into(),
            api_key: api_key.into(),
            environment: None,
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub(crate) async fn parse_response<T: DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<T, crate::error::BillplzError> {
        let status = resp.status();
        let body = resp.text().await?;

        if !status.is_success() {
            #[derive(serde::Deserialize)]
            struct ApiErrorWrapper {
                error: ApiErrorDetail,
            }
            #[derive(serde::Deserialize)]
            struct ApiErrorDetail {
                r#type: String,
                message: String,
            }

            if let Ok(api_err) = serde_json::from_str::<ApiErrorWrapper>(&body) {
                return Err(crate::error::BillplzError::Api {
                    error_type: api_err.error.r#type,
                    message: api_err.error.message,
                });
            }
        }

        let parsed: T = serde_json::from_str(&body)?;
        Ok(parsed)
    }
}

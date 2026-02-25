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
}

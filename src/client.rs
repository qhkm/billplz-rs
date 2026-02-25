#[derive(Debug, Clone)]
pub enum Environment {
    Production,
    Staging,
}

pub struct BillplzClient {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: String,
    pub(crate) api_key: String,
}

impl BillplzClient {
    pub fn new(environment: Environment, api_key: impl Into<String>) -> Self {
        let base_url = match environment {
            Environment::Production => "https://www.billplz.com".to_string(),
            Environment::Staging => "https://www.billplz-sandbox.com".to_string(),
        };

        Self {
            http: reqwest::Client::new(),
            base_url,
            api_key: api_key.into(),
        }
    }
}

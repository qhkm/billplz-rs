#[derive(Debug, thiserror::Error)]
pub enum BillplzError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error ({error_type}): {message}")]
    Api {
        error_type: String,
        message: String,
    },

    #[error("JSON parse error: {0}")]
    Parse(#[from] serde_json::Error),
}

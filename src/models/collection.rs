use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitPayment {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_cut: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_cut: Option<String>,
    pub stack_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_header: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_payments: Option<Vec<SplitPayment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logo {
    #[serde(default)]
    pub thumb_url: Option<String>,
    #[serde(default)]
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionResponse {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub split_header: Option<bool>,
    #[serde(default)]
    pub split_payments: Option<Vec<SplitPayment>>,
    #[serde(default)]
    pub logo: Option<Logo>,
    pub status: String,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bill {
    pub collection_id: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    pub name: String,
    pub amount: i64,
    pub callback_url: String,
    pub description: String,
    pub due_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_1_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_2_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_2: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BillResponse {
    pub id: String,
    pub collection_id: String,
    pub email: String,
    #[serde(default)]
    pub mobile: Option<String>,
    pub name: String,
    pub amount: i64,
    pub callback_url: String,
    pub description: String,
    pub due_at: String,
    #[serde(default)]
    pub redirect_url: Option<String>,
    #[serde(default)]
    pub deliver: Option<bool>,
    #[serde(default)]
    pub reference_1_label: Option<String>,
    #[serde(default)]
    pub reference_1: Option<String>,
    #[serde(default)]
    pub reference_2_label: Option<String>,
    #[serde(default)]
    pub reference_2: Option<String>,
    pub paid: bool,
    pub state: String,
    #[serde(default)]
    pub paid_amount: i64,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub paid_at: Option<String>,
}

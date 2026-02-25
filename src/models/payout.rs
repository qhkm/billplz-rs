use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payout {
    pub mass_payment_instruction_collection_id: String,
    pub bank_code: String,
    pub bank_account_number: String,
    pub identity_number: String,
    pub name: String,
    pub description: String,
    pub total: i64,
}

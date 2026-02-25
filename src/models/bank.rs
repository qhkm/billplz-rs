use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FpxBank {
    pub bank_code: String,
    pub bank_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bank {
    pub name: String,
    pub id_no: String,
    pub acc_no: String,
    pub code: String,
    pub organization: bool,
}

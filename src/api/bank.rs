use crate::client::{BillplzClient, Environment};
use crate::error::BillplzError;
use crate::models::bank::{Bank, FpxBank};

/// Returns the hardcoded list of Malaysian FPX banks.
/// If the client environment is Staging, test banks are appended.
fn production_banks() -> Vec<FpxBank> {
    vec![
        FpxBank { bank_code: "ABMB0212".into(), bank_name: "Alliance Bank".into() },
        FpxBank { bank_code: "ABB0233".into(),  bank_name: "Affin Bank".into() },
        FpxBank { bank_code: "AMBB0209".into(), bank_name: "AmBank".into() },
        FpxBank { bank_code: "BCBB0235".into(), bank_name: "CIMB Clicks".into() },
        FpxBank { bank_code: "BIMB0340".into(), bank_name: "Bank Islam".into() },
        FpxBank { bank_code: "BKRM0602".into(), bank_name: "Bank Rakyat".into() },
        FpxBank { bank_code: "BMMB0341".into(), bank_name: "Bank Muamalat".into() },
        FpxBank { bank_code: "BSN0601".into(),  bank_name: "BSN".into() },
        FpxBank { bank_code: "CIT0217".into(),  bank_name: "Citibank Berhad".into() },
        FpxBank { bank_code: "HLB0224".into(),  bank_name: "Hong Leong Bank".into() },
        FpxBank { bank_code: "HSBC0223".into(), bank_name: "HSBC Bank".into() },
        FpxBank { bank_code: "KFH0346".into(),  bank_name: "Kuwait Finance House".into() },
        FpxBank { bank_code: "MB2U0227".into(), bank_name: "Maybank2u".into() },
        FpxBank { bank_code: "MBB0227".into(),  bank_name: "Maybank2E".into() },
        FpxBank { bank_code: "MBB0228".into(),  bank_name: "Maybank2E".into() },
        FpxBank { bank_code: "OCBC0229".into(), bank_name: "OCBC Bank".into() },
        FpxBank { bank_code: "PBB0233".into(),  bank_name: "Public Bank".into() },
        FpxBank { bank_code: "RHB0218".into(),  bank_name: "RHB Now".into() },
        FpxBank { bank_code: "SCB0216".into(),  bank_name: "Standard Chartered".into() },
        FpxBank { bank_code: "UOB0226".into(),  bank_name: "UOB Bank".into() },
    ]
}

fn staging_test_banks() -> Vec<FpxBank> {
    vec![
        FpxBank { bank_code: "TEST0001".into(), bank_name: "Test 0001".into() },
        FpxBank { bank_code: "TEST0002".into(), bank_name: "Test 0002".into() },
        FpxBank { bank_code: "TEST0003".into(), bank_name: "Test 0003".into() },
        FpxBank { bank_code: "TEST0004".into(), bank_name: "Test 0004".into() },
        FpxBank { bank_code: "TEST0021".into(), bank_name: "Test 0021".into() },
        FpxBank { bank_code: "TEST0022".into(), bank_name: "Test 0022".into() },
        FpxBank { bank_code: "TEST0023".into(), bank_name: "Test 0023".into() },
    ]
}

pub struct CreateBankVerificationBuilder<'a> {
    client: &'a BillplzClient,
    name: String,
    id_no: String,
    acc_no: String,
    code: String,
    organization: bool,
}

impl<'a> CreateBankVerificationBuilder<'a> {
    pub(crate) fn new(
        client: &'a BillplzClient,
        name: impl Into<String>,
        id_no: impl Into<String>,
        acc_no: impl Into<String>,
        code: impl Into<String>,
    ) -> Self {
        Self {
            client,
            name: name.into(),
            id_no: id_no.into(),
            acc_no: acc_no.into(),
            code: code.into(),
            organization: false,
        }
    }

    pub fn organization(mut self, organization: bool) -> Self {
        self.organization = organization;
        self
    }

    pub async fn send(self) -> Result<String, BillplzError> {
        let url = format!("{}/api/v3/bank_verification_services", self.client.base_url);

        let body = Bank {
            name: self.name,
            id_no: self.id_no,
            acc_no: self.acc_no,
            code: self.code,
            organization: self.organization,
        };

        let resp = self
            .client
            .http
            .post(&url)
            .basic_auth(&self.client.api_key, Option::<&str>::None)
            .json(&body)
            .send()
            .await?;

        let text = resp.text().await?;
        Ok(text)
    }
}

impl BillplzClient {
    /// Returns the hardcoded list of Malaysian FPX banks.
    /// Staging environment also includes test banks.
    pub fn get_fpx_banks(&self) -> Vec<FpxBank> {
        let mut banks = production_banks();
        if matches!(self.environment, Some(Environment::Staging)) {
            banks.extend(staging_test_banks());
        }
        banks
    }

    /// GET /api/v3/bank_verification_services/{bank_account_number}
    /// Returns the raw response body as a String.
    pub async fn get_bank_verification(
        &self,
        bank_account_number: &str,
    ) -> Result<String, BillplzError> {
        let url = format!(
            "{}/api/v3/bank_verification_services/{}",
            self.base_url, bank_account_number
        );

        let resp = self
            .http
            .get(&url)
            .basic_auth(&self.api_key, Option::<&str>::None)
            .send()
            .await?;

        let text = resp.text().await?;
        Ok(text)
    }

    /// POST /api/v3/bank_verification_services
    /// Returns a builder for creating a bank verification request.
    pub fn create_bank_verification(
        &self,
        name: impl Into<String>,
        id_no: impl Into<String>,
        acc_no: impl Into<String>,
        code: impl Into<String>,
    ) -> CreateBankVerificationBuilder<'_> {
        CreateBankVerificationBuilder::new(self, name, id_no, acc_no, code)
    }
}

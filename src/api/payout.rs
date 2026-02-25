use crate::client::BillplzClient;
use crate::error::BillplzError;
use crate::models::payout::Payout;

pub struct CreatePayoutBuilder<'a> {
    client: &'a BillplzClient,
    mass_payment_instruction_collection_id: String,
    bank_code: String,
    bank_account_number: String,
    identity_number: String,
    name: String,
    description: String,
    total: i64,
}

impl<'a> CreatePayoutBuilder<'a> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        client: &'a BillplzClient,
        mass_payment_instruction_collection_id: impl Into<String>,
        bank_code: impl Into<String>,
        bank_account_number: impl Into<String>,
        identity_number: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        total: i64,
    ) -> Self {
        Self {
            client,
            mass_payment_instruction_collection_id: mass_payment_instruction_collection_id.into(),
            bank_code: bank_code.into(),
            bank_account_number: bank_account_number.into(),
            identity_number: identity_number.into(),
            name: name.into(),
            description: description.into(),
            total,
        }
    }

    pub async fn send(self) -> Result<String, BillplzError> {
        let url = format!("{}/api/v4/mass_payment_instructions", self.client.base_url);

        let body = Payout {
            mass_payment_instruction_collection_id: self.mass_payment_instruction_collection_id,
            bank_code: self.bank_code,
            bank_account_number: self.bank_account_number,
            identity_number: self.identity_number,
            name: self.name,
            description: self.description,
            total: self.total,
        };

        let resp = self
            .client
            .http
            .post(&url)
            .basic_auth(&self.client.api_key, Option::<&str>::None)
            .json(&body)
            .send()
            .await?;

        Ok(resp.text().await?)
    }
}

impl BillplzClient {
    pub async fn get_payout(&self, payout_id: impl Into<String>) -> Result<String, BillplzError> {
        let url = format!(
            "{}/api/v4/mass_payment_instructions/{}",
            self.base_url,
            payout_id.into()
        );

        let resp = self
            .http
            .get(&url)
            .basic_auth(&self.api_key, Option::<&str>::None)
            .send()
            .await?;

        Ok(resp.text().await?)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_payout(
        &self,
        mass_payment_instruction_collection_id: impl Into<String>,
        bank_code: impl Into<String>,
        bank_account_number: impl Into<String>,
        identity_number: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        total: i64,
    ) -> CreatePayoutBuilder<'_> {
        CreatePayoutBuilder::new(
            self,
            mass_payment_instruction_collection_id,
            bank_code,
            bank_account_number,
            identity_number,
            name,
            description,
            total,
        )
    }
}

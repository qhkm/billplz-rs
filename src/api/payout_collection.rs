use crate::client::BillplzClient;
use crate::error::BillplzError;
use crate::models::payout_collection::PayoutCollection;

pub struct CreatePayoutCollectionBuilder<'a> {
    client: &'a BillplzClient,
    title: String,
}

impl<'a> CreatePayoutCollectionBuilder<'a> {
    pub(crate) fn new(client: &'a BillplzClient, title: impl Into<String>) -> Self {
        Self {
            client,
            title: title.into(),
        }
    }

    pub async fn send(self) -> Result<String, BillplzError> {
        let url = format!(
            "{}/api/v4/mass_payment_instruction_collections",
            self.client.base_url
        );

        let body = PayoutCollection {
            title: self.title,
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
    pub async fn get_payout_collection(
        &self,
        payout_collection_id: impl Into<String>,
    ) -> Result<String, BillplzError> {
        let url = format!(
            "{}/api/v4/mass_payment_instruction_collections/{}",
            self.base_url,
            payout_collection_id.into()
        );

        let resp = self
            .http
            .get(&url)
            .basic_auth(&self.api_key, Option::<&str>::None)
            .send()
            .await?;

        Ok(resp.text().await?)
    }

    pub fn create_payout_collection(
        &self,
        title: impl Into<String>,
    ) -> CreatePayoutCollectionBuilder<'_> {
        CreatePayoutCollectionBuilder::new(self, title)
    }
}

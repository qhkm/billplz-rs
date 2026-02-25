use crate::client::BillplzClient;
use crate::error::BillplzError;
use crate::models::collection::{Collection, CollectionResponse, SplitPayment};

pub struct CreateCollectionBuilder<'a> {
    client: &'a BillplzClient,
    title: String,
    split_header: Option<bool>,
    split_payments: Vec<SplitPayment>,
}

impl<'a> CreateCollectionBuilder<'a> {
    pub(crate) fn new(client: &'a BillplzClient, title: impl Into<String>) -> Self {
        Self {
            client,
            title: title.into(),
            split_header: None,
            split_payments: Vec::new(),
        }
    }

    pub fn split_header(mut self, split_header: bool) -> Self {
        self.split_header = Some(split_header);
        self
    }

    pub fn split_payment(mut self, email: impl Into<String>, stack_order: i32) -> Self {
        self.split_payments.push(SplitPayment {
            email: email.into(),
            fixed_cut: None,
            variable_cut: None,
            stack_order,
        });
        self
    }

    pub fn split_payment_with_fixed_cut(
        mut self,
        email: impl Into<String>,
        fixed_cut: i64,
        stack_order: i32,
    ) -> Self {
        self.split_payments.push(SplitPayment {
            email: email.into(),
            fixed_cut: Some(fixed_cut),
            variable_cut: None,
            stack_order,
        });
        self
    }

    pub fn split_payment_with_variable_cut(
        mut self,
        email: impl Into<String>,
        variable_cut: impl Into<String>,
        stack_order: i32,
    ) -> Self {
        self.split_payments.push(SplitPayment {
            email: email.into(),
            fixed_cut: None,
            variable_cut: Some(variable_cut.into()),
            stack_order,
        });
        self
    }

    pub async fn send(self) -> Result<CollectionResponse, BillplzError> {
        let url = format!("{}/api/v4/collections", self.client.base_url);

        let body = Collection {
            title: self.title,
            split_header: self.split_header,
            split_payments: if self.split_payments.is_empty() {
                None
            } else {
                Some(self.split_payments)
            },
        };

        let resp = self
            .client
            .http
            .post(&url)
            .basic_auth(&self.client.api_key, Option::<&str>::None)
            .json(&body)
            .send()
            .await?;

        self.client.parse_response(resp).await
    }
}

impl BillplzClient {
    pub async fn get_collection(
        &self,
        collection_id: &str,
    ) -> Result<CollectionResponse, BillplzError> {
        let url = format!("{}/api/v4/collections/{}", self.base_url, collection_id);

        let resp = self
            .http
            .get(&url)
            .basic_auth(&self.api_key, Option::<&str>::None)
            .send()
            .await?;

        self.parse_response(resp).await
    }

    pub fn create_collection(&self, title: impl Into<String>) -> CreateCollectionBuilder<'_> {
        CreateCollectionBuilder::new(self, title)
    }
}

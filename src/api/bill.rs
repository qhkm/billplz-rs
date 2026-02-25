use crate::client::BillplzClient;
use crate::error::BillplzError;
use crate::models::bill::{Bill, BillResponse};

pub struct CreateBillBuilder<'a> {
    client: &'a BillplzClient,
    collection_id: String,
    email: String,
    name: String,
    amount: i64,
    callback_url: String,
    description: String,
    due_at: String,
    mobile: Option<String>,
    redirect_url: Option<String>,
    deliver: Option<bool>,
    reference_1_label: Option<String>,
    reference_1: Option<String>,
    reference_2_label: Option<String>,
    reference_2: Option<String>,
}

impl<'a> CreateBillBuilder<'a> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        client: &'a BillplzClient,
        collection_id: impl Into<String>,
        email: impl Into<String>,
        name: impl Into<String>,
        amount: i64,
        callback_url: impl Into<String>,
        description: impl Into<String>,
        due_at: impl Into<String>,
    ) -> Self {
        Self {
            client,
            collection_id: collection_id.into(),
            email: email.into(),
            name: name.into(),
            amount,
            callback_url: callback_url.into(),
            description: description.into(),
            due_at: due_at.into(),
            mobile: None,
            redirect_url: None,
            deliver: None,
            reference_1_label: None,
            reference_1: None,
            reference_2_label: None,
            reference_2: None,
        }
    }

    pub fn mobile(mut self, mobile: impl Into<String>) -> Self {
        self.mobile = Some(mobile.into());
        self
    }

    pub fn redirect_url(mut self, redirect_url: impl Into<String>) -> Self {
        self.redirect_url = Some(redirect_url.into());
        self
    }

    pub fn deliver(mut self, deliver: bool) -> Self {
        self.deliver = Some(deliver);
        self
    }

    pub fn reference_1_label(mut self, label: impl Into<String>) -> Self {
        self.reference_1_label = Some(label.into());
        self
    }

    pub fn reference_1(mut self, reference: impl Into<String>) -> Self {
        self.reference_1 = Some(reference.into());
        self
    }

    pub fn reference_2_label(mut self, label: impl Into<String>) -> Self {
        self.reference_2_label = Some(label.into());
        self
    }

    pub fn reference_2(mut self, reference: impl Into<String>) -> Self {
        self.reference_2 = Some(reference.into());
        self
    }

    pub async fn send(self) -> Result<BillResponse, BillplzError> {
        let url = format!("{}/api/v3/bills", self.client.base_url);

        let body = Bill {
            collection_id: self.collection_id,
            email: self.email,
            mobile: self.mobile,
            name: self.name,
            amount: self.amount,
            callback_url: self.callback_url,
            description: self.description,
            due_at: self.due_at,
            redirect_url: self.redirect_url,
            deliver: self.deliver,
            reference_1_label: self.reference_1_label,
            reference_1: self.reference_1,
            reference_2_label: self.reference_2_label,
            reference_2: self.reference_2,
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
    pub async fn get_bill(&self, bill_id: impl Into<String>) -> Result<BillResponse, BillplzError> {
        let url = format!("{}/api/v3/bills/{}", self.base_url, bill_id.into());

        let resp = self
            .http
            .get(&url)
            .basic_auth(&self.api_key, Option::<&str>::None)
            .send()
            .await?;

        self.parse_response(resp).await
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_bill(
        &self,
        collection_id: impl Into<String>,
        email: impl Into<String>,
        name: impl Into<String>,
        amount: i64,
        callback_url: impl Into<String>,
        description: impl Into<String>,
        due_at: impl Into<String>,
    ) -> CreateBillBuilder<'_> {
        CreateBillBuilder::new(
            self,
            collection_id,
            email,
            name,
            amount,
            callback_url,
            description,
            due_at,
        )
    }
}

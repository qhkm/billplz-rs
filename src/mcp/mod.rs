use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::ServerInfo,
    tool, tool_handler, tool_router,
};
use serde::Deserialize;
use crate::BillplzClient;

pub struct BillplzMcp {
    client: BillplzClient,
    tool_router: ToolRouter<Self>,
}

impl BillplzMcp {
    pub fn new(client: BillplzClient) -> Self {
        Self {
            client,
            tool_router: Self::tool_router(),
        }
    }
}

// --- Input schemas ---

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetByIdInput {
    /// The resource ID
    pub id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateCollectionInput {
    /// Collection title
    pub title: String,
    /// Enable split header
    #[serde(default)]
    pub split_header: Option<bool>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateBillInput {
    /// Collection ID to create the bill under
    pub collection_id: String,
    /// Customer email
    pub email: String,
    /// Customer name
    pub name: String,
    /// Amount in cents (e.g. 10000 = RM 100.00)
    pub amount: i64,
    /// Callback URL for payment notifications
    pub callback_url: String,
    /// Bill description
    pub description: String,
    /// Due date (YYYY-MM-DD)
    pub due_at: String,
    /// Customer mobile number
    #[serde(default)]
    pub mobile: Option<String>,
    /// Redirect URL after payment
    #[serde(default)]
    pub redirect_url: Option<String>,
    /// Reference 1 label
    #[serde(default)]
    pub reference_1_label: Option<String>,
    /// Reference 1 value
    #[serde(default)]
    pub reference_1: Option<String>,
    /// Reference 2 label
    #[serde(default)]
    pub reference_2_label: Option<String>,
    /// Reference 2 value
    #[serde(default)]
    pub reference_2: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetBankVerificationInput {
    /// Bank account number
    pub account_number: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateBankVerificationInput {
    /// Account holder name
    pub name: String,
    /// Identity number (IC/passport)
    pub id_no: String,
    /// Bank account number
    pub acc_no: String,
    /// Bank SWIFT code
    pub code: String,
    /// Is this an organization account
    #[serde(default)]
    pub organization: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreatePayoutInput {
    /// Payout collection ID
    pub collection_id: String,
    /// Bank SWIFT code
    pub bank_code: String,
    /// Recipient bank account number
    pub acc_no: String,
    /// Recipient identity number
    pub id_no: String,
    /// Recipient name
    pub name: String,
    /// Payout description
    pub description: String,
    /// Total amount in cents
    pub total: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreatePayoutCollectionInput {
    /// Payout collection title
    pub title: String,
}

// --- Tool implementations ---

#[tool_router]
impl BillplzMcp {
    #[tool(description = "Get a Billplz collection by ID")]
    async fn get_collection(
        &self,
        Parameters(input): Parameters<GetByIdInput>,
    ) -> String {
        match self.client.get_collection(&input.id).await {
            Ok(r) => serde_json::to_string_pretty(&r).unwrap_or_else(|e| e.to_string()),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Create a new Billplz collection")]
    async fn create_collection(
        &self,
        Parameters(input): Parameters<CreateCollectionInput>,
    ) -> String {
        let mut builder = self.client.create_collection(&input.title);
        if let Some(true) = input.split_header {
            builder = builder.split_header(true);
        }
        match builder.send().await {
            Ok(r) => serde_json::to_string_pretty(&r).unwrap_or_else(|e| e.to_string()),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Get a Billplz bill by ID")]
    async fn get_bill(
        &self,
        Parameters(input): Parameters<GetByIdInput>,
    ) -> String {
        match self.client.get_bill(&input.id).await {
            Ok(r) => serde_json::to_string_pretty(&r).unwrap_or_else(|e| e.to_string()),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Create a new Billplz bill for payment collection. Amount is in cents (e.g. 10000 = RM 100.00)")]
    async fn create_bill(
        &self,
        Parameters(input): Parameters<CreateBillInput>,
    ) -> String {
        let mut builder = self.client.create_bill(
            &input.collection_id,
            &input.email,
            &input.name,
            input.amount,
            &input.callback_url,
            &input.description,
            &input.due_at,
        );
        if let Some(m) = &input.mobile {
            builder = builder.mobile(m);
        }
        if let Some(r) = &input.redirect_url {
            builder = builder.redirect_url(r);
        }
        if let Some(l) = &input.reference_1_label {
            builder = builder.reference_1_label(l);
        }
        if let Some(v) = &input.reference_1 {
            builder = builder.reference_1(v);
        }
        if let Some(l) = &input.reference_2_label {
            builder = builder.reference_2_label(l);
        }
        if let Some(v) = &input.reference_2 {
            builder = builder.reference_2(v);
        }
        match builder.send().await {
            Ok(r) => serde_json::to_string_pretty(&r).unwrap_or_else(|e| e.to_string()),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "List all Malaysian FPX banks available for online payment")]
    async fn get_fpx_banks(&self) -> String {
        let banks = self.client.get_fpx_banks();
        serde_json::to_string_pretty(&banks).unwrap_or_else(|e| e.to_string())
    }

    #[tool(description = "Get bank account verification status by account number")]
    async fn get_bank_verification(
        &self,
        Parameters(input): Parameters<GetBankVerificationInput>,
    ) -> String {
        match self.client.get_bank_verification(&input.account_number).await {
            Ok(r) => r,
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Create a bank account verification for payout eligibility")]
    async fn create_bank_verification(
        &self,
        Parameters(input): Parameters<CreateBankVerificationInput>,
    ) -> String {
        match self.client
            .create_bank_verification(&input.name, &input.id_no, &input.acc_no, &input.code)
            .organization(input.organization)
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Get a mass payment instruction (payout) by ID")]
    async fn get_payout(
        &self,
        Parameters(input): Parameters<GetByIdInput>,
    ) -> String {
        match self.client.get_payout(&input.id).await {
            Ok(r) => r,
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Create a mass payment instruction (payout). Total is in cents.")]
    async fn create_payout(
        &self,
        Parameters(input): Parameters<CreatePayoutInput>,
    ) -> String {
        match self.client
            .create_payout(
                &input.collection_id,
                &input.bank_code,
                &input.acc_no,
                &input.id_no,
                &input.name,
                &input.description,
                input.total,
            )
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Get a payout collection by ID")]
    async fn get_payout_collection(
        &self,
        Parameters(input): Parameters<GetByIdInput>,
    ) -> String {
        match self.client.get_payout_collection(&input.id).await {
            Ok(r) => r,
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Create a new payout collection")]
    async fn create_payout_collection(
        &self,
        Parameters(input): Parameters<CreatePayoutCollectionInput>,
    ) -> String {
        match self.client.create_payout_collection(&input.title).send().await {
            Ok(r) => r,
            Err(e) => format!("Error: {}", e),
        }
    }
}

#[tool_handler]
impl ServerHandler for BillplzMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Billplz payment gateway MCP server. Manage collections, bills, payouts, and bank verifications.".into()),
            ..Default::default()
        }
    }
}

pub async fn start_mcp_server(client: BillplzClient) -> Result<(), Box<dyn std::error::Error>> {
    use rmcp::{ServiceExt, transport::stdio};
    let service = BillplzMcp::new(client);
    let server = service.serve(stdio()).await?;
    server.waiting().await?;
    Ok(())
}

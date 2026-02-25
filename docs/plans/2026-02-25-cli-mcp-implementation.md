# CLI + MCP Server Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a CLI binary and MCP server to billplz-rs so humans and AI agents can interact with the Billplz API from the terminal.

**Architecture:** A `billplz` binary built with clap (derive) for CLI subcommands, plus a `billplz mcp` subcommand that starts an rmcp stdio MCP server. Both share the same config loading (env vars > config file) and the existing library client.

**Tech Stack:** clap (CLI), rmcp (MCP server), toml (config), dirs (home dir), schemars (JSON Schema for MCP tool inputs), serde_json (output)

---

### Task 1: Update Cargo.toml + Add Binary Entry Point

**Files:**
- Modify: `Cargo.toml`
- Create: `src/main.rs`

**Step 1: Update Cargo.toml**

Add `[[bin]]` section and new dependencies:

```toml
[package]
name = "billplz"
version = "0.2.0"
edition = "2021"
description = "Rust SDK and CLI for the Billplz payment gateway API"
license = "MIT"
repository = "https://github.com/qhkm/billplz-rs"
keywords = ["billplz", "payment", "fpx", "malaysia"]
categories = ["api-bindings", "command-line-utilities"]
readme = "README.md"

[[bin]]
name = "billplz"
path = "src/main.rs"

[dependencies]
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
clap = { version = "4", features = ["derive"] }
rmcp = { version = "0.16", features = ["server", "transport-io"] }
schemars = "1"
toml = "0.8"
dirs = "6"

[dev-dependencies]
wiremock = "0.6"
tokio = { version = "1", features = ["full", "test-util"] }
```

**Step 2: Create minimal src/main.rs**

```rust
mod cli;

#[tokio::main]
async fn main() {
    if let Err(e) = cli::run().await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
```

**Step 3: Create src/cli/mod.rs stub**

```rust
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("billplz CLI - coming soon");
    Ok(())
}
```

**Step 4: Verify it compiles**

Run: `cd ~/rust/billplz-rs && cargo build`
Expected: Compiles. Both lib and bin targets built.

**Step 5: Commit**

```bash
cd ~/rust/billplz-rs
git add Cargo.toml src/main.rs src/cli/mod.rs
git commit -m "chore: add binary target and CLI module scaffold"
```

---

### Task 2: Config Loading (env vars + config file)

**Files:**
- Create: `src/cli/config.rs`
- Modify: `src/cli/mod.rs`
- Create: `tests/config_test.rs`

**Step 1: Write failing tests**

Create `tests/config_test.rs`:

```rust
use std::io::Write;

// We test the config module through the library's public interface
// Config loading: env vars take priority over config file

#[test]
fn test_config_from_env_vars() {
    // Temporarily set env vars
    std::env::set_var("BILLPLZ_API_KEY", "env-test-key");
    std::env::set_var("BILLPLZ_ENVIRONMENT", "staging");

    let config = billplz::cli::config::Config::load(None).unwrap();
    assert_eq!(config.api_key, "env-test-key");
    assert_eq!(config.environment, "staging");

    std::env::remove_var("BILLPLZ_API_KEY");
    std::env::remove_var("BILLPLZ_ENVIRONMENT");
}

#[test]
fn test_config_from_file() {
    // Clear env vars to test file fallback
    std::env::remove_var("BILLPLZ_API_KEY");
    std::env::remove_var("BILLPLZ_ENVIRONMENT");

    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("config.toml");
    let mut f = std::fs::File::create(&config_path).unwrap();
    writeln!(f, r#"api_key = "file-test-key""#).unwrap();
    writeln!(f, r#"environment = "production""#).unwrap();

    let config = billplz::cli::config::Config::load(Some(&config_path)).unwrap();
    assert_eq!(config.api_key, "file-test-key");
    assert_eq!(config.environment, "production");
}

#[test]
fn test_env_vars_override_config_file() {
    std::env::set_var("BILLPLZ_API_KEY", "env-key");
    std::env::set_var("BILLPLZ_ENVIRONMENT", "staging");

    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("config.toml");
    let mut f = std::fs::File::create(&config_path).unwrap();
    writeln!(f, r#"api_key = "file-key""#).unwrap();
    writeln!(f, r#"environment = "production""#).unwrap();

    let config = billplz::cli::config::Config::load(Some(&config_path)).unwrap();
    assert_eq!(config.api_key, "env-key");
    assert_eq!(config.environment, "staging");

    std::env::remove_var("BILLPLZ_API_KEY");
    std::env::remove_var("BILLPLZ_ENVIRONMENT");
}

#[test]
fn test_config_missing_api_key_errors() {
    std::env::remove_var("BILLPLZ_API_KEY");
    std::env::remove_var("BILLPLZ_ENVIRONMENT");

    let result = billplz::cli::config::Config::load(Some(std::path::Path::new("/nonexistent")));
    assert!(result.is_err());
}
```

Note: add `tempfile = "3"` to `[dev-dependencies]` in Cargo.toml.

**Step 2: Run tests to verify they fail**

Run: `cd ~/rust/billplz-rs && cargo test --test config_test`
Expected: FAIL — module doesn't exist yet.

**Step 3: Implement config module**

Make the `cli` module public in `src/lib.rs` by adding:
```rust
pub mod cli;
```

Create `src/cli/config.rs`:

```rust
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Default)]
struct FileConfig {
    api_key: Option<String>,
    environment: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub api_key: String,
    pub environment: String,
}

impl Config {
    pub fn load(config_path: Option<&Path>) -> Result<Self, String> {
        // Try loading from file first
        let file_config = config_path
            .or_else(|| {
                dirs::home_dir().map(|h| h.join(".billplz").join("config.toml"))
            }.as_deref())
            .and_then(|p| std::fs::read_to_string(p).ok())
            .and_then(|s| toml::from_str::<FileConfig>(&s).ok())
            .unwrap_or_default();

        // Env vars override file config
        let api_key = std::env::var("BILLPLZ_API_KEY")
            .ok()
            .or(file_config.api_key)
            .ok_or("API key not found. Set BILLPLZ_API_KEY or add api_key to ~/.billplz/config.toml")?;

        let environment = std::env::var("BILLPLZ_ENVIRONMENT")
            .ok()
            .or(file_config.environment)
            .unwrap_or_else(|| "staging".to_string());

        Ok(Config { api_key, environment })
    }

    pub fn into_client(self) -> crate::BillplzClient {
        let env = match self.environment.as_str() {
            "production" => crate::Environment::Production,
            _ => crate::Environment::Staging,
        };
        crate::BillplzClient::new(env, self.api_key)
    }
}
```

Update `src/cli/mod.rs`:

```rust
pub mod config;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("billplz CLI - coming soon");
    Ok(())
}
```

**Step 4: Run tests to verify they pass**

Run: `cd ~/rust/billplz-rs && cargo test --test config_test`
Expected: PASS (run tests serially with `-- --test-threads=1` since they mutate env vars).

**Step 5: Commit**

```bash
cd ~/rust/billplz-rs
git add Cargo.toml src/lib.rs src/cli/ tests/config_test.rs
git commit -m "feat: add config loading with env var and config file support"
```

---

### Task 3: CLI Argument Parsing with Clap

**Files:**
- Modify: `src/cli/mod.rs`
- Modify: `src/main.rs`

**Step 1: Implement clap CLI structure**

Update `src/cli/mod.rs` with full clap definitions:

```rust
pub mod config;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "billplz", version, about = "CLI and MCP server for the Billplz payment API")]
pub struct Cli {
    /// Output formatted JSON
    #[arg(long, global = true)]
    pub pretty: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage collections
    Collection {
        #[command(subcommand)]
        action: CollectionAction,
    },
    /// Manage bills
    Bill {
        #[command(subcommand)]
        action: BillAction,
    },
    /// Bank operations
    Bank {
        #[command(subcommand)]
        action: BankAction,
    },
    /// Manage payouts
    Payout {
        #[command(subcommand)]
        action: PayoutAction,
    },
    /// Manage payout collections
    PayoutCollection {
        #[command(subcommand)]
        action: PayoutCollectionAction,
    },
    /// Start MCP server (stdio)
    Mcp,
}

#[derive(Subcommand)]
pub enum CollectionAction {
    /// Get a collection by ID
    Get { id: String },
    /// Create a new collection
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        split_header: bool,
    },
}

#[derive(Subcommand)]
pub enum BillAction {
    /// Get a bill by ID
    Get { id: String },
    /// Create a new bill
    Create {
        #[arg(long)]
        collection_id: String,
        #[arg(long)]
        email: String,
        #[arg(long)]
        name: String,
        #[arg(long)]
        amount: i64,
        #[arg(long)]
        callback_url: String,
        #[arg(long)]
        description: String,
        #[arg(long)]
        due_at: String,
        #[arg(long)]
        mobile: Option<String>,
        #[arg(long)]
        redirect_url: Option<String>,
        #[arg(long)]
        reference_1_label: Option<String>,
        #[arg(long)]
        reference_1: Option<String>,
        #[arg(long)]
        reference_2_label: Option<String>,
        #[arg(long)]
        reference_2: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum BankAction {
    /// List FPX banks
    FpxList,
    /// Get bank account verification status
    Verify { account_number: String },
    /// Create a bank account verification
    CreateVerification {
        #[arg(long)]
        name: String,
        #[arg(long)]
        id_no: String,
        #[arg(long)]
        acc_no: String,
        #[arg(long)]
        code: String,
        #[arg(long)]
        organization: bool,
    },
}

#[derive(Subcommand)]
pub enum PayoutAction {
    /// Get a payout by ID
    Get { id: String },
    /// Create a payout
    Create {
        #[arg(long)]
        collection_id: String,
        #[arg(long)]
        bank_code: String,
        #[arg(long)]
        acc_no: String,
        #[arg(long)]
        id_no: String,
        #[arg(long)]
        name: String,
        #[arg(long)]
        description: String,
        #[arg(long)]
        total: i64,
    },
}

#[derive(Subcommand)]
pub enum PayoutCollectionAction {
    /// Get a payout collection by ID
    Get { id: String },
    /// Create a payout collection
    Create {
        #[arg(long)]
        title: String,
    },
}

fn output_json(value: &impl serde::Serialize, pretty: bool) {
    if pretty {
        println!("{}", serde_json::to_string_pretty(value).unwrap());
    } else {
        println!("{}", serde_json::to_string(value).unwrap());
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Mcp => {
            // Task 5: MCP server
            eprintln!("MCP server not yet implemented");
            Ok(())
        }
        _ => {
            let config = config::Config::load(None)?;
            let client = config.into_client();
            execute_command(&cli, &client).await
        }
    }
}

async fn execute_command(cli: &Cli, client: &crate::BillplzClient) -> Result<(), Box<dyn std::error::Error>> {
    match &cli.command {
        Commands::Collection { action } => match action {
            CollectionAction::Get { id } => {
                let result = client.get_collection(id).await?;
                output_json(&result, cli.pretty);
            }
            CollectionAction::Create { title, split_header } => {
                let mut builder = client.create_collection(title);
                if *split_header {
                    builder = builder.split_header(true);
                }
                let result = builder.send().await?;
                output_json(&result, cli.pretty);
            }
        },
        Commands::Bill { action } => match action {
            BillAction::Get { id } => {
                let result = client.get_bill(id).await?;
                output_json(&result, cli.pretty);
            }
            BillAction::Create {
                collection_id, email, name, amount, callback_url,
                description, due_at, mobile, redirect_url,
                reference_1_label, reference_1, reference_2_label, reference_2,
            } => {
                let mut builder = client.create_bill(
                    collection_id, email, name, *amount, callback_url, description, due_at,
                );
                if let Some(m) = mobile { builder = builder.mobile(m); }
                if let Some(r) = redirect_url { builder = builder.redirect_url(r); }
                if let Some(l) = reference_1_label { builder = builder.reference_1_label(l); }
                if let Some(v) = reference_1 { builder = builder.reference_1(v); }
                if let Some(l) = reference_2_label { builder = builder.reference_2_label(l); }
                if let Some(v) = reference_2 { builder = builder.reference_2(v); }
                let result = builder.send().await?;
                output_json(&result, cli.pretty);
            }
        },
        Commands::Bank { action } => match action {
            BankAction::FpxList => {
                let banks = client.get_fpx_banks();
                output_json(&banks, cli.pretty);
            }
            BankAction::Verify { account_number } => {
                let result = client.get_bank_verification(account_number).await?;
                // Raw string from API, parse as JSON value for consistent output
                let value: serde_json::Value = serde_json::from_str(&result)
                    .unwrap_or(serde_json::Value::String(result));
                output_json(&value, cli.pretty);
            }
            BankAction::CreateVerification { name, id_no, acc_no, code, organization } => {
                let result = client.create_bank_verification(name, id_no, acc_no, code)
                    .organization(*organization)
                    .send()
                    .await?;
                let value: serde_json::Value = serde_json::from_str(&result)
                    .unwrap_or(serde_json::Value::String(result));
                output_json(&value, cli.pretty);
            }
        },
        Commands::Payout { action } => match action {
            PayoutAction::Get { id } => {
                let result = client.get_payout(id).await?;
                let value: serde_json::Value = serde_json::from_str(&result)
                    .unwrap_or(serde_json::Value::String(result));
                output_json(&value, cli.pretty);
            }
            PayoutAction::Create {
                collection_id, bank_code, acc_no, id_no, name, description, total,
            } => {
                let result = client.create_payout(
                    collection_id, bank_code, acc_no, id_no, name, description, *total,
                ).send().await?;
                let value: serde_json::Value = serde_json::from_str(&result)
                    .unwrap_or(serde_json::Value::String(result));
                output_json(&value, cli.pretty);
            }
        },
        Commands::PayoutCollection { action } => match action {
            PayoutCollectionAction::Get { id } => {
                let result = client.get_payout_collection(id).await?;
                let value: serde_json::Value = serde_json::from_str(&result)
                    .unwrap_or(serde_json::Value::String(result));
                output_json(&value, cli.pretty);
            }
            PayoutCollectionAction::Create { title } => {
                let result = client.create_payout_collection(title).send().await?;
                let value: serde_json::Value = serde_json::from_str(&result)
                    .unwrap_or(serde_json::Value::String(result));
                output_json(&value, cli.pretty);
            }
        },
        Commands::Mcp => unreachable!(),
    }
    Ok(())
}
```

Note: `BillResponse` and `CollectionResponse` need `Serialize` derived so `output_json` can serialize them. Update `src/models/bill.rs` to add `Serialize` to `BillResponse`, and `src/models/collection.rs` to add `Serialize` to `CollectionResponse` and `Logo`.

**Step 2: Verify it compiles and help text works**

Run: `cd ~/rust/billplz-rs && cargo run -- --help`
Expected: Shows help with all subcommands.

Run: `cd ~/rust/billplz-rs && cargo run -- bill create --help`
Expected: Shows bill create options.

**Step 3: Commit**

```bash
cd ~/rust/billplz-rs
git add src/cli/mod.rs src/main.rs src/models/bill.rs src/models/collection.rs
git commit -m "feat: add CLI with all subcommands for collections, bills, banks, payouts"
```

---

### Task 4: Add Serialize to Response Models

**Files:**
- Modify: `src/models/bill.rs` — add `Serialize` to `BillResponse`
- Modify: `src/models/collection.rs` — add `Serialize` to `CollectionResponse`, `Logo`

**Step 1: Add Serialize derive**

In `src/models/bill.rs`, change:
```rust
#[derive(Debug, Clone, Deserialize)]
pub struct BillResponse {
```
to:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillResponse {
```

In `src/models/collection.rs`, change:
```rust
#[derive(Debug, Clone, Deserialize)]
pub struct Logo {
```
to:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logo {
```

And:
```rust
#[derive(Debug, Clone, Deserialize)]
pub struct CollectionResponse {
```
to:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionResponse {
```

Add `use serde::Serialize;` if not already imported (should be via `use serde::{Deserialize, Serialize};`).

**Step 2: Verify existing tests still pass**

Run: `cd ~/rust/billplz-rs && cargo test`
Expected: All 29 existing tests pass.

**Step 3: Commit**

```bash
cd ~/rust/billplz-rs
git add src/models/bill.rs src/models/collection.rs
git commit -m "feat: add Serialize to response models for JSON CLI output"
```

---

### Task 5: MCP Server

**Files:**
- Create: `src/mcp/mod.rs`
- Modify: `src/main.rs` (add `mod mcp`)
- Modify: `src/lib.rs` (add `pub mod mcp`)
- Modify: `src/cli/mod.rs` (wire up `Commands::Mcp`)

**Step 1: Create MCP server module**

Create `src/mcp/mod.rs`:

```rust
use rmcp::{
    ServerHandler,
    tool,
    handler::server::tool::Parameters,
    model::{ServerInfo, CallToolResult, Content},
    Error as McpError,
};
use serde::Deserialize;
use crate::BillplzClient;

pub struct BillplzMcp {
    client: BillplzClient,
}

impl BillplzMcp {
    pub fn new(client: BillplzClient) -> Self {
        Self { client }
    }
}

// Input schemas for each tool

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetCollectionInput {
    /// The collection ID
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
pub struct GetBillInput {
    /// The bill ID
    pub id: String,
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
    /// Bank account number to verify
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
pub struct GetPayoutInput {
    /// Payout ID
    pub id: String,
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
pub struct GetPayoutCollectionInput {
    /// Payout collection ID
    pub id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreatePayoutCollectionInput {
    /// Payout collection title
    pub title: String,
}

#[rmcp::tool_router]
impl BillplzMcp {
    #[tool(description = "Get a Billplz collection by ID")]
    async fn get_collection(
        &self,
        Parameters(input): Parameters<GetCollectionInput>,
    ) -> Result<CallToolResult, McpError> {
        match self.client.get_collection(&input.id).await {
            Ok(r) => Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&r).unwrap(),
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Create a new Billplz collection")]
    async fn create_collection(
        &self,
        Parameters(input): Parameters<CreateCollectionInput>,
    ) -> Result<CallToolResult, McpError> {
        let mut builder = self.client.create_collection(&input.title);
        if let Some(true) = input.split_header {
            builder = builder.split_header(true);
        }
        match builder.send().await {
            Ok(r) => Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&r).unwrap(),
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Get a Billplz bill by ID")]
    async fn get_bill(
        &self,
        Parameters(input): Parameters<GetBillInput>,
    ) -> Result<CallToolResult, McpError> {
        match self.client.get_bill(&input.id).await {
            Ok(r) => Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&r).unwrap(),
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Create a new Billplz bill for payment collection")]
    async fn create_bill(
        &self,
        Parameters(input): Parameters<CreateBillInput>,
    ) -> Result<CallToolResult, McpError> {
        let mut builder = self.client.create_bill(
            &input.collection_id,
            &input.email,
            &input.name,
            input.amount,
            &input.callback_url,
            &input.description,
            &input.due_at,
        );
        if let Some(m) = &input.mobile { builder = builder.mobile(m); }
        if let Some(r) = &input.redirect_url { builder = builder.redirect_url(r); }
        if let Some(l) = &input.reference_1_label { builder = builder.reference_1_label(l); }
        if let Some(v) = &input.reference_1 { builder = builder.reference_1(v); }
        if let Some(l) = &input.reference_2_label { builder = builder.reference_2_label(l); }
        if let Some(v) = &input.reference_2 { builder = builder.reference_2(v); }
        match builder.send().await {
            Ok(r) => Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&r).unwrap(),
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "List all FPX banks available for payment")]
    async fn get_fpx_banks(&self) -> Result<CallToolResult, McpError> {
        let banks = self.client.get_fpx_banks();
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&banks).unwrap(),
        )]))
    }

    #[tool(description = "Get bank account verification status")]
    async fn get_bank_verification(
        &self,
        Parameters(input): Parameters<GetBankVerificationInput>,
    ) -> Result<CallToolResult, McpError> {
        match self.client.get_bank_verification(&input.account_number).await {
            Ok(r) => Ok(CallToolResult::success(vec![Content::text(r)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Create a bank account verification for payout eligibility")]
    async fn create_bank_verification(
        &self,
        Parameters(input): Parameters<CreateBankVerificationInput>,
    ) -> Result<CallToolResult, McpError> {
        match self.client
            .create_bank_verification(&input.name, &input.id_no, &input.acc_no, &input.code)
            .organization(input.organization)
            .send()
            .await
        {
            Ok(r) => Ok(CallToolResult::success(vec![Content::text(r)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Get a payout (mass payment instruction) by ID")]
    async fn get_payout(
        &self,
        Parameters(input): Parameters<GetPayoutInput>,
    ) -> Result<CallToolResult, McpError> {
        match self.client.get_payout(&input.id).await {
            Ok(r) => Ok(CallToolResult::success(vec![Content::text(r)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Create a payout (mass payment instruction)")]
    async fn create_payout(
        &self,
        Parameters(input): Parameters<CreatePayoutInput>,
    ) -> Result<CallToolResult, McpError> {
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
            Ok(r) => Ok(CallToolResult::success(vec![Content::text(r)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Get a payout collection by ID")]
    async fn get_payout_collection(
        &self,
        Parameters(input): Parameters<GetPayoutCollectionInput>,
    ) -> Result<CallToolResult, McpError> {
        match self.client.get_payout_collection(&input.id).await {
            Ok(r) => Ok(CallToolResult::success(vec![Content::text(r)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(description = "Create a new payout collection")]
    async fn create_payout_collection(
        &self,
        Parameters(input): Parameters<CreatePayoutCollectionInput>,
    ) -> Result<CallToolResult, McpError> {
        match self.client.create_payout_collection(&input.title).send().await {
            Ok(r) => Ok(CallToolResult::success(vec![Content::text(r)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }
}

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
```

**Step 2: Wire up MCP in CLI**

Update `Commands::Mcp` handler in `src/cli/mod.rs`:

```rust
Commands::Mcp => {
    let config = config::Config::load(None)?;
    let client = config.into_client();
    crate::mcp::start_mcp_server(client).await
}
```

Add `pub mod mcp;` to `src/lib.rs`.
Add `mod mcp;` is not needed in main.rs since it's in lib.rs.

**Step 3: Verify it compiles**

Run: `cd ~/rust/billplz-rs && cargo build`
Expected: Compiles.

**Step 4: Verify help text**

Run: `cd ~/rust/billplz-rs && cargo run -- mcp --help`

**Step 5: Commit**

```bash
cd ~/rust/billplz-rs
git add src/mcp/ src/lib.rs src/cli/mod.rs
git commit -m "feat: add MCP server with all Billplz tools"
```

---

### Task 6: Update README + Final Verification

**Files:**
- Modify: `README.md`

**Step 1: Run full test suite + clippy**

Run: `cd ~/rust/billplz-rs && cargo test && cargo clippy -- -D warnings`

**Step 2: Update README with CLI and MCP sections**

Add CLI usage and MCP configuration sections to `README.md` after the existing library usage section.

**Step 3: Bump version to 0.2.0**

Already done in Cargo.toml from Task 1.

**Step 4: Commit and push**

```bash
cd ~/rust/billplz-rs
git add -A
git commit -m "docs: update README with CLI and MCP server usage"
git push
```

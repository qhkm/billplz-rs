# billplz-rs Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Port the billplz-go SDK to an idiomatic Rust crate with builder pattern, async/await, and proper error handling.

**Architecture:** A `BillplzClient` struct owns a `reqwest::Client`, base URL, and API key. GET endpoints are direct async methods. POST/create endpoints use per-resource builder structs with required fields in constructors and optional fields as chainable methods. All methods return `Result<T, BillplzError>`.

**Tech Stack:** Rust, reqwest, tokio, serde, thiserror, wiremock (test mock server)

**Reference:** Design doc at `docs/plans/2026-02-25-billplz-rs-design.md`. Source Go code at `~/ios/billplz-go/`.

---

### Task 1: Project Scaffolding

**Files:**
- Create: `Cargo.toml`
- Create: `src/lib.rs`

**Step 1: Initialize the Rust project**

Run:
```bash
cd ~/rust/billplz-rs
cargo init --lib
```
Expected: `Cargo.toml` and `src/lib.rs` created.

**Step 2: Replace Cargo.toml with dependencies**

Replace `Cargo.toml` contents:

```toml
[package]
name = "billplz"
version = "0.1.0"
edition = "2021"
description = "Rust SDK for the Billplz payment gateway API"
license = "MIT"

[dependencies]
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"

[dev-dependencies]
wiremock = "0.6"
tokio = { version = "1", features = ["full", "test-util"] }
```

**Step 3: Replace src/lib.rs with module skeleton**

```rust
pub mod error;
pub mod models;
pub mod api;
mod client;

pub use client::{BillplzClient, Environment};
pub use error::BillplzError;
```

**Step 4: Create empty module files**

Create these files with minimal content so the project compiles:

`src/error.rs`:
```rust
#[derive(Debug, thiserror::Error)]
pub enum BillplzError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error ({error_type}): {message}")]
    Api {
        error_type: String,
        message: String,
    },

    #[error("JSON parse error: {0}")]
    Parse(#[from] serde_json::Error),
}
```

`src/models/mod.rs`:
```rust
pub mod bank;
pub mod bill;
pub mod collection;
pub mod payout;
pub mod payout_collection;
```

`src/models/bank.rs`:
```rust
// Bank models - implemented in Task 4
```

`src/models/bill.rs`:
```rust
// Bill models - implemented in Task 4
```

`src/models/collection.rs`:
```rust
// Collection models - implemented in Task 4
```

`src/models/payout.rs`:
```rust
// Payout models - implemented in Task 4
```

`src/models/payout_collection.rs`:
```rust
// Payout collection models - implemented in Task 4
```

`src/api/mod.rs`:
```rust
pub mod bank;
pub mod bill;
pub mod collection;
pub mod payout;
pub mod payout_collection;
```

`src/api/bank.rs`:
```rust
// Bank API - implemented in Task 6
```

`src/api/bill.rs`:
```rust
// Bill API - implemented in Task 7
```

`src/api/collection.rs`:
```rust
// Collection API - implemented in Task 5
```

`src/api/payout.rs`:
```rust
// Payout API - implemented in Task 8
```

`src/api/payout_collection.rs`:
```rust
// Payout collection API - implemented in Task 9
```

`src/client.rs`:
```rust
use crate::error::BillplzError;

#[derive(Debug, Clone)]
pub enum Environment {
    Production,
    Staging,
}

pub struct BillplzClient {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: String,
    pub(crate) api_key: String,
}

impl BillplzClient {
    pub fn new(environment: Environment, api_key: impl Into<String>) -> Self {
        let base_url = match environment {
            Environment::Production => "https://www.billplz.com".to_string(),
            Environment::Staging => "https://www.billplz-sandbox.com".to_string(),
        };

        Self {
            http: reqwest::Client::new(),
            base_url,
            api_key: api_key.into(),
        }
    }
}
```

**Step 5: Verify it compiles**

Run: `cd ~/rust/billplz-rs && cargo check`
Expected: Compiles with no errors.

**Step 6: Initialize git and commit**

```bash
cd ~/rust/billplz-rs
git init
echo "target/" > .gitignore
git add -A
git commit -m "chore: scaffold billplz-rs project with module structure"
```

---

### Task 2: Error Handling + Tests

**Files:**
- Modify: `src/error.rs`
- Create: `tests/error_test.rs`

**Step 1: Write the failing test**

Create `tests/error_test.rs`:

```rust
use billplz::BillplzError;

#[test]
fn test_api_error_display() {
    let err = BillplzError::Api {
        error_type: "unauthorized".to_string(),
        message: "Invalid API key".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "API error (unauthorized): Invalid API key"
    );
}

#[test]
fn test_api_error_debug() {
    let err = BillplzError::Api {
        error_type: "not_found".to_string(),
        message: "Bill not found".to_string(),
    };
    let debug = format!("{:?}", err);
    assert!(debug.contains("not_found"));
    assert!(debug.contains("Bill not found"));
}
```

**Step 2: Run test to verify it passes**

Run: `cd ~/rust/billplz-rs && cargo test --test error_test`
Expected: PASS (error types already exist from scaffolding).

**Step 3: Commit**

```bash
cd ~/rust/billplz-rs
git add tests/error_test.rs
git commit -m "test: add error type display and debug tests"
```

---

### Task 3: Client Construction + Tests

**Files:**
- Create: `tests/client_test.rs`
- Modify: `src/client.rs` (if needed)

**Step 1: Write the failing test**

Create `tests/client_test.rs`:

```rust
use billplz::{BillplzClient, Environment};

#[test]
fn test_client_production_url() {
    let client = BillplzClient::new(Environment::Production, "test-api-key");
    assert_eq!(client.base_url(), "https://www.billplz.com");
}

#[test]
fn test_client_staging_url() {
    let client = BillplzClient::new(Environment::Staging, "test-api-key");
    assert_eq!(client.base_url(), "https://www.billplz-sandbox.com");
}

#[test]
fn test_client_custom_url() {
    let client = BillplzClient::with_base_url("http://localhost:8080", "test-key");
    assert_eq!(client.base_url(), "http://localhost:8080");
}
```

**Step 2: Run test to verify it fails**

Run: `cd ~/rust/billplz-rs && cargo test --test client_test`
Expected: FAIL — `base_url()` and `with_base_url()` methods don't exist yet.

**Step 3: Implement the methods**

Update `src/client.rs`:

```rust
use crate::error::BillplzError;

#[derive(Debug, Clone)]
pub enum Environment {
    Production,
    Staging,
}

pub struct BillplzClient {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: String,
    pub(crate) api_key: String,
}

impl BillplzClient {
    pub fn new(environment: Environment, api_key: impl Into<String>) -> Self {
        let base_url = match environment {
            Environment::Production => "https://www.billplz.com".to_string(),
            Environment::Staging => "https://www.billplz-sandbox.com".to_string(),
        };

        Self {
            http: reqwest::Client::new(),
            base_url,
            api_key: api_key.into(),
        }
    }

    /// Create a client with a custom base URL (useful for testing).
    pub fn with_base_url(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: base_url.into(),
            api_key: api_key.into(),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cd ~/rust/billplz-rs && cargo test --test client_test`
Expected: PASS

**Step 5: Commit**

```bash
cd ~/rust/billplz-rs
git add src/client.rs tests/client_test.rs
git commit -m "feat: add BillplzClient with base_url accessor and custom URL constructor"
```

---

### Task 4: Models (All Resources)

**Files:**
- Modify: `src/models/collection.rs`
- Modify: `src/models/bill.rs`
- Modify: `src/models/bank.rs`
- Modify: `src/models/payout.rs`
- Modify: `src/models/payout_collection.rs`
- Create: `tests/models_test.rs`

**Step 1: Write failing tests for model serialization/deserialization**

Create `tests/models_test.rs`:

```rust
use billplz::models::{
    bill::{Bill, BillResponse},
    collection::{Collection, CollectionResponse, SplitPayment},
    bank::{Bank, FpxBank},
    payout::Payout,
    payout_collection::PayoutCollection,
};

#[test]
fn test_bill_serialize() {
    let bill = Bill {
        collection_id: "abc123".to_string(),
        email: "test@test.com".to_string(),
        mobile: Some("601234567890".to_string()),
        name: "Test User".to_string(),
        amount: 10000,
        callback_url: "https://example.com/callback".to_string(),
        description: "Test bill".to_string(),
        due_at: "2024-07-12".to_string(),
        redirect_url: None,
        deliver: None,
        reference_1_label: None,
        reference_1: None,
        reference_2_label: None,
        reference_2: None,
    };
    let json = serde_json::to_value(&bill).unwrap();
    assert_eq!(json["collection_id"], "abc123");
    assert_eq!(json["amount"], 10000);
    assert_eq!(json["email"], "test@test.com");
}

#[test]
fn test_bill_response_deserialize() {
    let json = r#"{
        "id": "bill123",
        "collection_id": "abc123",
        "email": "test@test.com",
        "mobile": "601234567890",
        "name": "Test User",
        "amount": 10000,
        "callback_url": "https://example.com/callback",
        "description": "Test bill",
        "due_at": "2024-07-12",
        "paid": false,
        "state": "due",
        "paid_amount": 0,
        "url": "https://www.billplz.com/bills/bill123"
    }"#;
    let resp: BillResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.id, "bill123");
    assert!(!resp.paid);
    assert_eq!(resp.state, "due");
    assert_eq!(resp.amount, 10000);
}

#[test]
fn test_collection_serialize() {
    let collection = Collection {
        title: "My Collection".to_string(),
        split_header: None,
        split_payments: None,
    };
    let json = serde_json::to_value(&collection).unwrap();
    assert_eq!(json["title"], "My Collection");
}

#[test]
fn test_collection_with_split_payments_serialize() {
    let collection = Collection {
        title: "Split Collection".to_string(),
        split_header: Some(true),
        split_payments: Some(vec![
            SplitPayment {
                email: "a@test.com".to_string(),
                fixed_cut: Some(100),
                variable_cut: None,
                stack_order: 0,
            },
        ]),
    };
    let json = serde_json::to_value(&collection).unwrap();
    assert_eq!(json["split_payments"][0]["email"], "a@test.com");
    assert_eq!(json["split_payments"][0]["fixed_cut"], 100);
}

#[test]
fn test_collection_response_deserialize() {
    let json = r#"{
        "id": "col123",
        "title": "My Collection",
        "logo": {
            "thumb_url": "https://example.com/thumb.png",
            "avatar_url": "https://example.com/avatar.png"
        },
        "status": "active"
    }"#;
    let resp: CollectionResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.id, "col123");
    assert_eq!(resp.title, "My Collection");
    assert_eq!(resp.status, "active");
}

#[test]
fn test_bank_serialize() {
    let bank = Bank {
        name: "Test User".to_string(),
        id_no: "91234567890".to_string(),
        acc_no: "999988887777".to_string(),
        code: "MBBEMYKL".to_string(),
        organization: true,
    };
    let json = serde_json::to_value(&bank).unwrap();
    assert_eq!(json["name"], "Test User");
    assert_eq!(json["code"], "MBBEMYKL");
    assert!(json["organization"].as_bool().unwrap());
}

#[test]
fn test_fpx_bank_deserialize() {
    let json = r#"{"bank_code": "MB2U0227", "bank_name": "Maybank2u"}"#;
    let bank: FpxBank = serde_json::from_str(json).unwrap();
    assert_eq!(bank.bank_code, "MB2U0227");
    assert_eq!(bank.bank_name, "Maybank2u");
}

#[test]
fn test_payout_serialize() {
    let payout = Payout {
        mass_payment_instruction_collection_id: "col123".to_string(),
        bank_code: "MBBEMYKL".to_string(),
        bank_account_number: "999988887777".to_string(),
        identity_number: "91234567890".to_string(),
        name: "Test User".to_string(),
        description: "Payout test".to_string(),
        total: 50000,
    };
    let json = serde_json::to_value(&payout).unwrap();
    assert_eq!(json["total"], 50000);
    assert_eq!(json["bank_code"], "MBBEMYKL");
}

#[test]
fn test_payout_collection_serialize() {
    let pc = PayoutCollection {
        title: "My Payout Collection".to_string(),
    };
    let json = serde_json::to_value(&pc).unwrap();
    assert_eq!(json["title"], "My Payout Collection");
}
```

**Step 2: Run test to verify it fails**

Run: `cd ~/rust/billplz-rs && cargo test --test models_test`
Expected: FAIL — model structs are empty placeholders.

**Step 3: Implement all models**

`src/models/bill.rs`:
```rust
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
```

`src/models/collection.rs`:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitPayment {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_cut: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_cut: Option<String>,
    pub stack_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_header: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_payments: Option<Vec<SplitPayment>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Logo {
    #[serde(default)]
    pub thumb_url: Option<String>,
    #[serde(default)]
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CollectionResponse {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub split_header: Option<bool>,
    #[serde(default)]
    pub split_payments: Option<Vec<SplitPayment>>,
    #[serde(default)]
    pub logo: Option<Logo>,
    pub status: String,
}
```

`src/models/bank.rs`:
```rust
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
```

`src/models/payout.rs`:
```rust
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
```

`src/models/payout_collection.rs`:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutCollection {
    pub title: String,
}
```

**Step 4: Run test to verify it passes**

Run: `cd ~/rust/billplz-rs && cargo test --test models_test`
Expected: PASS

**Step 5: Commit**

```bash
cd ~/rust/billplz-rs
git add src/models/ tests/models_test.rs
git commit -m "feat: add all model structs with serde serialization"
```

---

### Task 5: Collection API (GET + Create Builder)

**Files:**
- Modify: `src/api/collection.rs`
- Create: `tests/collection_api_test.rs`

**Step 1: Write failing tests**

Create `tests/collection_api_test.rs`:

```rust
use billplz::{BillplzClient, BillplzError};
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, header};

#[tokio::test]
async fn test_get_collection_success() {
    let mock_server = MockServer::start().await;
    let client = BillplzClient::with_base_url(&mock_server.uri(), "test-api-key");

    Mock::given(method("GET"))
        .and(path("/api/v4/collections/col123"))
        .and(header("authorization", "Basic dGVzdC1hcGkta2V5Og=="))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "col123",
            "title": "My Collection",
            "logo": {
                "thumb_url": "https://example.com/thumb.png",
                "avatar_url": "https://example.com/avatar.png"
            },
            "status": "active"
        })))
        .mount(&mock_server)
        .await;

    let resp = client.get_collection("col123").await.unwrap();
    assert_eq!(resp.id, "col123");
    assert_eq!(resp.title, "My Collection");
    assert_eq!(resp.status, "active");
}

#[tokio::test]
async fn test_get_collection_api_error() {
    let mock_server = MockServer::start().await;
    let client = BillplzClient::with_base_url(&mock_server.uri(), "bad-key");

    Mock::given(method("GET"))
        .and(path("/api/v4/collections/col123"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "error": {
                "type": "unauthorized",
                "message": "Invalid API key"
            }
        })))
        .mount(&mock_server)
        .await;

    let err = client.get_collection("col123").await.unwrap_err();
    match err {
        BillplzError::Api { error_type, message } => {
            assert_eq!(error_type, "unauthorized");
            assert_eq!(message, "Invalid API key");
        }
        _ => panic!("Expected Api error, got {:?}", err),
    }
}

#[tokio::test]
async fn test_create_collection_success() {
    let mock_server = MockServer::start().await;
    let client = BillplzClient::with_base_url(&mock_server.uri(), "test-api-key");

    Mock::given(method("POST"))
        .and(path("/api/v4/collections"))
        .and(header("content-type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "new_col",
            "title": "New Collection",
            "status": "active"
        })))
        .mount(&mock_server)
        .await;

    let resp = client
        .create_collection("New Collection")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.id, "new_col");
    assert_eq!(resp.title, "New Collection");
}

#[tokio::test]
async fn test_create_collection_with_split_payments() {
    let mock_server = MockServer::start().await;
    let client = BillplzClient::with_base_url(&mock_server.uri(), "test-api-key");

    Mock::given(method("POST"))
        .and(path("/api/v4/collections"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "split_col",
            "title": "Split Collection",
            "split_header": true,
            "status": "active"
        })))
        .mount(&mock_server)
        .await;

    let resp = client
        .create_collection("Split Collection")
        .split_header(true)
        .split_payment("a@test.com", 0)
        .split_payment_with_fixed_cut("b@test.com", 100, 1)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.id, "split_col");
}
```

**Step 2: Run test to verify it fails**

Run: `cd ~/rust/billplz-rs && cargo test --test collection_api_test`
Expected: FAIL — `get_collection`, `create_collection` methods don't exist.

**Step 3: Implement collection API + helper for API error parsing**

`src/api/collection.rs`:
```rust
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
```

Also add the shared `parse_response` helper to `src/client.rs`. Append after the existing `impl BillplzClient` block:

```rust
// Add these imports at the top of client.rs:
use serde::de::DeserializeOwned;

// Add this inside the existing impl BillplzClient block:
    pub(crate) async fn parse_response<T: DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<T, BillplzError> {
        let status = resp.status();
        let body = resp.text().await?;

        // Try to parse as API error first if non-success status
        if !status.is_success() {
            #[derive(serde::Deserialize)]
            struct ApiErrorWrapper {
                error: ApiErrorDetail,
            }
            #[derive(serde::Deserialize)]
            struct ApiErrorDetail {
                r#type: String,
                message: String,
            }

            if let Ok(api_err) = serde_json::from_str::<ApiErrorWrapper>(&body) {
                return Err(BillplzError::Api {
                    error_type: api_err.error.r#type,
                    message: api_err.error.message,
                });
            }
        }

        let parsed: T = serde_json::from_str(&body)?;
        Ok(parsed)
    }
```

**Step 4: Run test to verify it passes**

Run: `cd ~/rust/billplz-rs && cargo test --test collection_api_test`
Expected: PASS

**Step 5: Commit**

```bash
cd ~/rust/billplz-rs
git add src/api/collection.rs src/client.rs tests/collection_api_test.rs
git commit -m "feat: add collection API with get and create builder"
```

---

### Task 6: Bill API (GET + Create Builder)

**Files:**
- Modify: `src/api/bill.rs`
- Create: `tests/bill_api_test.rs`

**Step 1: Write failing tests**

Create `tests/bill_api_test.rs`:

```rust
use billplz::BillplzClient;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_get_bill_success() {
    let mock_server = MockServer::start().await;
    let client = BillplzClient::with_base_url(&mock_server.uri(), "test-key");

    Mock::given(method("GET"))
        .and(path("/api/v3/bills/bill123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "bill123",
            "collection_id": "col1",
            "email": "test@test.com",
            "name": "Test",
            "amount": 5000,
            "callback_url": "https://cb.url",
            "description": "Test bill",
            "due_at": "2024-07-12",
            "paid": true,
            "state": "paid",
            "paid_amount": 5000,
            "url": "https://www.billplz.com/bills/bill123"
        })))
        .mount(&mock_server)
        .await;

    let resp = client.get_bill("bill123").await.unwrap();
    assert_eq!(resp.id, "bill123");
    assert!(resp.paid);
    assert_eq!(resp.paid_amount, 5000);
}

#[tokio::test]
async fn test_create_bill_required_fields_only() {
    let mock_server = MockServer::start().await;
    let client = BillplzClient::with_base_url(&mock_server.uri(), "test-key");

    Mock::given(method("POST"))
        .and(path("/api/v3/bills"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "new_bill",
            "collection_id": "col1",
            "email": "test@test.com",
            "name": "Test User",
            "amount": 10000,
            "callback_url": "https://cb.url",
            "description": "Test",
            "due_at": "2024-07-12",
            "paid": false,
            "state": "due",
            "paid_amount": 0,
            "url": "https://www.billplz.com/bills/new_bill"
        })))
        .mount(&mock_server)
        .await;

    let resp = client
        .create_bill("col1", "test@test.com", "Test User", 10000, "https://cb.url", "Test", "2024-07-12")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.id, "new_bill");
    assert!(!resp.paid);
}

#[tokio::test]
async fn test_create_bill_with_optional_fields() {
    let mock_server = MockServer::start().await;
    let client = BillplzClient::with_base_url(&mock_server.uri(), "test-key");

    Mock::given(method("POST"))
        .and(path("/api/v3/bills"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "opt_bill",
            "collection_id": "col1",
            "email": "test@test.com",
            "name": "Test User",
            "amount": 10000,
            "callback_url": "https://cb.url",
            "description": "Test",
            "due_at": "2024-07-12",
            "paid": false,
            "state": "due",
            "paid_amount": 0,
            "mobile": "601234567890",
            "redirect_url": "https://redirect.url",
            "reference_1_label": "Ref 1",
            "reference_1": "abc"
        })))
        .mount(&mock_server)
        .await;

    let resp = client
        .create_bill("col1", "test@test.com", "Test User", 10000, "https://cb.url", "Test", "2024-07-12")
        .mobile("601234567890")
        .redirect_url("https://redirect.url")
        .reference_1_label("Ref 1")
        .reference_1("abc")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.id, "opt_bill");
}
```

**Step 2: Run test to verify it fails**

Run: `cd ~/rust/billplz-rs && cargo test --test bill_api_test`
Expected: FAIL — `get_bill`, `create_bill` methods don't exist.

**Step 3: Implement bill API**

`src/api/bill.rs`:
```rust
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

    pub fn reference_1(mut self, value: impl Into<String>) -> Self {
        self.reference_1 = Some(value.into());
        self
    }

    pub fn reference_2_label(mut self, label: impl Into<String>) -> Self {
        self.reference_2_label = Some(label.into());
        self
    }

    pub fn reference_2(mut self, value: impl Into<String>) -> Self {
        self.reference_2 = Some(value.into());
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
    pub async fn get_bill(&self, bill_id: &str) -> Result<BillResponse, BillplzError> {
        let url = format!("{}/api/v3/bills/{}", self.base_url, bill_id);

        let resp = self
            .http
            .get(&url)
            .basic_auth(&self.api_key, Option::<&str>::None)
            .send()
            .await?;

        self.parse_response(resp).await
    }

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
```

**Step 4: Run test to verify it passes**

Run: `cd ~/rust/billplz-rs && cargo test --test bill_api_test`
Expected: PASS

**Step 5: Commit**

```bash
cd ~/rust/billplz-rs
git add src/api/bill.rs tests/bill_api_test.rs
git commit -m "feat: add bill API with get and create builder"
```

---

### Task 7: Bank API (FPX Banks, Verification GET + Create)

**Files:**
- Modify: `src/api/bank.rs`
- Create: `tests/bank_api_test.rs`

**Step 1: Write failing tests**

Create `tests/bank_api_test.rs`:

```rust
use billplz::{BillplzClient, Environment};
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[test]
fn test_get_fpx_banks_production() {
    let client = BillplzClient::new(Environment::Production, "key");
    let banks = client.get_fpx_banks();
    assert!(!banks.is_empty());
    assert!(banks.iter().any(|b| b.bank_code == "MB2U0227"));
    // No test banks in production
    assert!(!banks.iter().any(|b| b.bank_code.starts_with("TEST")));
}

#[test]
fn test_get_fpx_banks_staging_includes_test_banks() {
    let client = BillplzClient::new(Environment::Staging, "key");
    let banks = client.get_fpx_banks();
    assert!(banks.iter().any(|b| b.bank_code == "MB2U0227"));
    assert!(banks.iter().any(|b| b.bank_code.starts_with("TEST")));
}

#[tokio::test]
async fn test_get_bank_verification_success() {
    let mock_server = MockServer::start().await;
    let client = BillplzClient::with_base_url(&mock_server.uri(), "test-key");

    Mock::given(method("GET"))
        .and(path("/api/v3/bank_verification_services/999988887777"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{"name":"Test User","id_no":"91234567890","acc_no":"999988887777","code":"MBBEMYKL","organization":false,"authorization_date":"2024-01-01","status":"verified"}"#,
        ))
        .mount(&mock_server)
        .await;

    let resp = client
        .get_bank_verification("999988887777")
        .await
        .unwrap();
    assert!(resp.contains("verified"));
}

#[tokio::test]
async fn test_create_bank_verification_success() {
    let mock_server = MockServer::start().await;
    let client = BillplzClient::with_base_url(&mock_server.uri(), "test-key");

    Mock::given(method("POST"))
        .and(path("/api/v3/bank_verification_services"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{"name":"Test User","id_no":"91234567890","acc_no":"999988887777","code":"MBBEMYKL","organization":true,"status":"pending"}"#,
        ))
        .mount(&mock_server)
        .await;

    let resp = client
        .create_bank_verification("Test User", "91234567890", "999988887777", "MBBEMYKL")
        .organization(true)
        .send()
        .await
        .unwrap();
    assert!(resp.contains("pending"));
}
```

**Step 2: Run test to verify it fails**

Run: `cd ~/rust/billplz-rs && cargo test --test bank_api_test`
Expected: FAIL

**Step 3: Implement bank API**

Note: The Go SDK's bank verification endpoints return raw strings (not typed responses), so we mirror that. `get_fpx_banks` is a local hardcoded list (same as Go).

Add to `src/client.rs` a field to track the environment:

Update `BillplzClient` struct and constructors to store `environment`:

```rust
pub struct BillplzClient {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: String,
    pub(crate) api_key: String,
    pub(crate) environment: Option<Environment>,
}
```

Update `new` to store `Some(environment)` and `with_base_url` to store `None`.

`src/api/bank.rs`:
```rust
use crate::client::{BillplzClient, Environment};
use crate::error::BillplzError;
use crate::models::bank::{Bank, FpxBank};

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

        Ok(resp.text().await?)
    }
}

impl BillplzClient {
    pub fn get_fpx_banks(&self) -> Vec<FpxBank> {
        let mut banks = vec![
            FpxBank { bank_code: "ABMB0212".into(), bank_name: "Alliance Bank".into() },
            FpxBank { bank_code: "ABB0233".into(), bank_name: "Affin Bank".into() },
            FpxBank { bank_code: "AMBB0209".into(), bank_name: "AmBank".into() },
            FpxBank { bank_code: "BCBB0235".into(), bank_name: "CIMB Clicks".into() },
            FpxBank { bank_code: "BIMB0340".into(), bank_name: "Bank Islam".into() },
            FpxBank { bank_code: "BKRM0602".into(), bank_name: "Bank Rakyat".into() },
            FpxBank { bank_code: "BMMB0341".into(), bank_name: "Bank Muamalat".into() },
            FpxBank { bank_code: "BSN0601".into(), bank_name: "BSN".into() },
            FpxBank { bank_code: "CIT0217".into(), bank_name: "Citibank Berhad".into() },
            FpxBank { bank_code: "HLB0224".into(), bank_name: "Hong Leong Bank".into() },
            FpxBank { bank_code: "HSBC0223".into(), bank_name: "HSBC Bank".into() },
            FpxBank { bank_code: "KFH0346".into(), bank_name: "Kuwait Finance House".into() },
            FpxBank { bank_code: "MB2U0227".into(), bank_name: "Maybank2u".into() },
            FpxBank { bank_code: "MBB0227".into(), bank_name: "Maybank2E".into() },
            FpxBank { bank_code: "MBB0228".into(), bank_name: "Maybank2E".into() },
            FpxBank { bank_code: "OCBC0229".into(), bank_name: "OCBC Bank".into() },
            FpxBank { bank_code: "PBB0233".into(), bank_name: "Public Bank".into() },
            FpxBank { bank_code: "RHB0218".into(), bank_name: "RHB Now".into() },
            FpxBank { bank_code: "SCB0216".into(), bank_name: "Standard Chartered".into() },
            FpxBank { bank_code: "UOB0226".into(), bank_name: "UOB Bank".into() },
        ];

        if matches!(self.environment, Some(Environment::Staging)) {
            banks.extend(vec![
                FpxBank { bank_code: "TEST0001*".into(), bank_name: "Test 0001".into() },
                FpxBank { bank_code: "TEST0002*".into(), bank_name: "Test 0002".into() },
                FpxBank { bank_code: "TEST0003*".into(), bank_name: "Test 0003".into() },
                FpxBank { bank_code: "TEST0004*".into(), bank_name: "Test 0004".into() },
                FpxBank { bank_code: "TEST0021*".into(), bank_name: "Test 0021".into() },
                FpxBank { bank_code: "TEST0022*".into(), bank_name: "Test 0022".into() },
                FpxBank { bank_code: "TEST0023*".into(), bank_name: "Test 0023".into() },
            ]);
        }

        banks
    }

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

        Ok(resp.text().await?)
    }

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
```

**Step 4: Run test to verify it passes**

Run: `cd ~/rust/billplz-rs && cargo test --test bank_api_test`
Expected: PASS

**Step 5: Commit**

```bash
cd ~/rust/billplz-rs
git add src/api/bank.rs src/client.rs tests/bank_api_test.rs
git commit -m "feat: add bank API with FPX banks list and verification endpoints"
```

---

### Task 8: Payout API (GET + Create Builder)

**Files:**
- Modify: `src/api/payout.rs`
- Create: `tests/payout_api_test.rs`

**Step 1: Write failing tests**

Create `tests/payout_api_test.rs`:

```rust
use billplz::BillplzClient;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_get_payout_success() {
    let mock_server = MockServer::start().await;
    let client = BillplzClient::with_base_url(&mock_server.uri(), "test-key");

    Mock::given(method("GET"))
        .and(path("/api/v4/mass_payment_instructions/payout123"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{"id":"payout123","mass_payment_instruction_collection_id":"col1","bank_code":"MBBEMYKL","bank_account_number":"999988887777","identity_number":"91234567890","name":"Test","description":"Payout","total":50000,"status":"processing"}"#,
        ))
        .mount(&mock_server)
        .await;

    let resp = client.get_payout("payout123").await.unwrap();
    assert!(resp.contains("payout123"));
}

#[tokio::test]
async fn test_create_payout_success() {
    let mock_server = MockServer::start().await;
    let client = BillplzClient::with_base_url(&mock_server.uri(), "test-key");

    Mock::given(method("POST"))
        .and(path("/api/v4/mass_payment_instructions"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{"id":"new_payout","status":"enqueued"}"#,
        ))
        .mount(&mock_server)
        .await;

    let resp = client
        .create_payout("col1", "MBBEMYKL", "999988887777", "91234567890", "Test User", "Test payout", 50000)
        .send()
        .await
        .unwrap();
    assert!(resp.contains("new_payout"));
}
```

**Step 2: Run test to verify it fails**

Run: `cd ~/rust/billplz-rs && cargo test --test payout_api_test`
Expected: FAIL

**Step 3: Implement payout API**

`src/api/payout.rs`:
```rust
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
    pub(crate) fn new(
        client: &'a BillplzClient,
        collection_id: impl Into<String>,
        bank_code: impl Into<String>,
        bank_account_number: impl Into<String>,
        identity_number: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        total: i64,
    ) -> Self {
        Self {
            client,
            mass_payment_instruction_collection_id: collection_id.into(),
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
    pub async fn get_payout(&self, payout_id: &str) -> Result<String, BillplzError> {
        let url = format!(
            "{}/api/v4/mass_payment_instructions/{}",
            self.base_url, payout_id
        );

        let resp = self
            .http
            .get(&url)
            .basic_auth(&self.api_key, Option::<&str>::None)
            .send()
            .await?;

        Ok(resp.text().await?)
    }

    pub fn create_payout(
        &self,
        collection_id: impl Into<String>,
        bank_code: impl Into<String>,
        bank_account_number: impl Into<String>,
        identity_number: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        total: i64,
    ) -> CreatePayoutBuilder<'_> {
        CreatePayoutBuilder::new(
            self,
            collection_id,
            bank_code,
            bank_account_number,
            identity_number,
            name,
            description,
            total,
        )
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cd ~/rust/billplz-rs && cargo test --test payout_api_test`
Expected: PASS

**Step 5: Commit**

```bash
cd ~/rust/billplz-rs
git add src/api/payout.rs tests/payout_api_test.rs
git commit -m "feat: add payout API with get and create builder"
```

---

### Task 9: Payout Collection API (GET + Create Builder)

**Files:**
- Modify: `src/api/payout_collection.rs`
- Create: `tests/payout_collection_api_test.rs`

**Step 1: Write failing tests**

Create `tests/payout_collection_api_test.rs`:

```rust
use billplz::BillplzClient;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_get_payout_collection_success() {
    let mock_server = MockServer::start().await;
    let client = BillplzClient::with_base_url(&mock_server.uri(), "test-key");

    Mock::given(method("GET"))
        .and(path("/api/v4/mass_payment_instruction_collections/pc123"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{"id":"pc123","title":"My Payout Collection","status":"active"}"#,
        ))
        .mount(&mock_server)
        .await;

    let resp = client.get_payout_collection("pc123").await.unwrap();
    assert!(resp.contains("pc123"));
}

#[tokio::test]
async fn test_create_payout_collection_success() {
    let mock_server = MockServer::start().await;
    let client = BillplzClient::with_base_url(&mock_server.uri(), "test-key");

    Mock::given(method("POST"))
        .and(path("/api/v4/mass_payment_instruction_collections"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{"id":"new_pc","title":"New Payout Collection","status":"active"}"#,
        ))
        .mount(&mock_server)
        .await;

    let resp = client
        .create_payout_collection("New Payout Collection")
        .send()
        .await
        .unwrap();
    assert!(resp.contains("new_pc"));
}
```

**Step 2: Run test to verify it fails**

Run: `cd ~/rust/billplz-rs && cargo test --test payout_collection_api_test`
Expected: FAIL

**Step 3: Implement payout collection API**

`src/api/payout_collection.rs`:
```rust
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
        payout_collection_id: &str,
    ) -> Result<String, BillplzError> {
        let url = format!(
            "{}/api/v4/mass_payment_instruction_collections/{}",
            self.base_url, payout_collection_id
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
```

**Step 4: Run test to verify it passes**

Run: `cd ~/rust/billplz-rs && cargo test --test payout_collection_api_test`
Expected: PASS

**Step 5: Commit**

```bash
cd ~/rust/billplz-rs
git add src/api/payout_collection.rs tests/payout_collection_api_test.rs
git commit -m "feat: add payout collection API with get and create builder"
```

---

### Task 10: Full Integration — Verify All Tests Pass

**Step 1: Run entire test suite**

Run: `cd ~/rust/billplz-rs && cargo test`
Expected: All tests pass.

**Step 2: Run clippy for linting**

Run: `cd ~/rust/billplz-rs && cargo clippy -- -D warnings`
Expected: No warnings.

**Step 3: Fix any clippy warnings if they exist**

Address all warnings (unused imports, etc.) and re-run.

**Step 4: Final commit**

```bash
cd ~/rust/billplz-rs
git add -A
git commit -m "chore: fix clippy warnings, all tests passing"
```

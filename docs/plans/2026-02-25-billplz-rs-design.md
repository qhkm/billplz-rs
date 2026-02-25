# billplz-rs Design

Rust port of the official [billplz-go](https://github.com/Billplz/billplz-go) SDK for the [Billplz API](https://billplz.com/api).

## Decisions

- **Async runtime**: tokio + reqwest
- **Pattern**: Builder pattern for create/POST endpoints, direct methods for GET endpoints
- **Error handling**: Idiomatic Rust `Result<T, BillplzError>` with custom error enum
- **No global state**: Each `BillplzClient` instance is self-contained (unlike Go version's package-level vars)

## Project Structure

```
~/rust/billplz-rs/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Re-exports, BillplzClient
│   ├── error.rs            # BillplzError enum
│   ├── models/
│   │   ├── mod.rs
│   │   ├── collection.rs   # Collection, CollectionResponse, SplitPayment, Logo
│   │   ├── bill.rs         # Bill, BillResponse
│   │   ├── bank.rs         # Bank, FpxBank
│   │   ├── payout.rs       # Payout
│   │   └── payout_collection.rs  # PayoutCollection
│   ├── api/
│   │   ├── mod.rs
│   │   ├── collection.rs   # CreateCollectionBuilder, get_collection
│   │   ├── bill.rs         # CreateBillBuilder, get_bill
│   │   ├── bank.rs         # CreateBankVerificationBuilder, get_bank_verification, get_fpx_banks
│   │   ├── payout.rs       # CreatePayoutBuilder, get_payout
│   │   └── payout_collection.rs  # CreatePayoutCollectionBuilder, get_payout_collection
```

## Core Client

```rust
pub enum Environment {
    Production,
    Staging,
}

pub struct BillplzClient {
    http: reqwest::Client,
    base_url: String,
    api_key: String,
    environment: Environment,
}

impl BillplzClient {
    pub fn new(environment: Environment, api_key: impl Into<String>) -> Self;
}
```

## Builder Pattern

Create/POST endpoints use builders. Required fields go in the constructor, optional fields are chained:

```rust
let bill = client
    .create_bill("collection_id", "email", "name", 10000, "https://cb.url", "desc", "2024-07-12")
    .mobile("60193102400")
    .redirect_url("https://redirect.url")
    .reference_1_label("Label")
    .reference_1("Value")
    .send()
    .await?;
```

GET endpoints are simple methods:

```rust
let bill = client.get_bill("bill_id").await?;
```

## Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum BillplzError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error ({error_type}): {message}")]
    Api { error_type: String, message: String },

    #[error("JSON parse error: {0}")]
    Parse(#[from] serde_json::Error),
}
```

All public methods return `Result<T, BillplzError>`.

## API Coverage

Ported 1:1 from the Go SDK:

| Resource            | Methods                              | API Version |
|---------------------|--------------------------------------|-------------|
| Collection          | get_collection, create_collection    | v4          |
| Bill                | get_bill, create_bill                | v3          |
| Bank                | get_fpx_banks, get_bank_verification, create_bank_verification | v3 |
| Payout              | get_payout, create_payout            | v4          |
| Payout Collection   | get_payout_collection, create_payout_collection | v4 |

## Dependencies

- `reqwest` (with `json` feature) — HTTP client
- `tokio` — async runtime
- `serde` / `serde_json` — serialization
- `thiserror` — error derive macro

## Base URLs

- Production: `https://www.billplz.com`
- Staging: `https://www.billplz-sandbox.com`

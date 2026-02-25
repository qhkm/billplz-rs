# billplz-rs

Rust SDK for the [Billplz API](https://billplz.com/api).

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
billplz = { git = "https://github.com/qhkm/billplz-rs" }
tokio = { version = "1", features = ["full"] }
```

## Setup

```rust
use billplz::{BillplzClient, Environment};

// Production
let client = BillplzClient::new(Environment::Production, "your-api-key");

// Staging / Sandbox
let client = BillplzClient::new(Environment::Staging, "your-api-key");
```

## Usage

### Collections

#### Get Collection

```rust
let collection = client.get_collection("ei3a6mdl").await?;
println!("Title: {}", collection.title);
println!("Status: {}", collection.status);
```

#### Create Collection

```rust
let collection = client
    .create_collection("My Collection")
    .send()
    .await?;
println!("ID: {}", collection.id);
```

#### Create Collection with Split Payments

```rust
let collection = client
    .create_collection("Split Collection")
    .split_header(true)
    .split_payment_with_fixed_cut("partner@example.com", 100, 0)
    .split_payment_with_variable_cut("partner2@example.com", "20", 1)
    .send()
    .await?;
```

### Bills

#### Create Bill

```rust
let bill = client
    .create_bill(
        "collection_id",
        "customer@example.com",
        "John Doe",
        10000, // amount in cents (RM 100.00)
        "https://example.com/callback",
        "Invoice #123",
        "2024-12-31",
    )
    .mobile("60191234567")
    .redirect_url("https://example.com/redirect")
    .reference_1_label("Reference")
    .reference_1("INV-123")
    .send()
    .await?;
println!("Bill URL: {:?}", bill.url);
```

#### Get Bill

```rust
let bill = client.get_bill("bill_id").await?;
println!("Paid: {}", bill.paid);
println!("State: {}", bill.state);
```

### Banks

#### Get FPX Banks

Returns the list of Malaysian FPX banks. Staging environment includes test banks.

```rust
let banks = client.get_fpx_banks();
for bank in &banks {
    println!("{}: {}", bank.bank_code, bank.bank_name);
}
```

#### Verify Bank Account

```rust
let result = client.get_bank_verification("999988887777").await?;
println!("{}", result);
```

#### Create Bank Verification

Required for mass payment (payout) recipients.

```rust
let result = client
    .create_bank_verification("John Doe", "91234567890", "999988887777", "MBBEMYKL")
    .organization(true)
    .send()
    .await?;
```

### Payout Collections

#### Create Payout Collection

```rust
let result = client
    .create_payout_collection("Salary Payments")
    .send()
    .await?;
```

#### Get Payout Collection

```rust
let result = client.get_payout_collection("payout_collection_id").await?;
```

### Payouts (Mass Payment Instructions)

#### Create Payout

```rust
let result = client
    .create_payout(
        "payout_collection_id",
        "MBBEMYKL",
        "999988887777",
        "91234567890",
        "John Doe",
        "Salary for January",
        50000, // RM 500.00
    )
    .send()
    .await?;
```

#### Get Payout

```rust
let result = client.get_payout("payout_id").await?;
```

## Error Handling

All methods return `Result<T, BillplzError>`. The error enum covers three cases:

```rust
use billplz::BillplzError;

match client.get_bill("invalid").await {
    Ok(bill) => println!("Got bill: {}", bill.id),
    Err(BillplzError::Api { error_type, message }) => {
        // Billplz API returned an error (e.g. unauthorized, not found)
        eprintln!("API error ({}): {}", error_type, message);
    }
    Err(BillplzError::Http(e)) => {
        // Network / HTTP error
        eprintln!("HTTP error: {}", e);
    }
    Err(BillplzError::Parse(e)) => {
        // JSON parsing error
        eprintln!("Parse error: {}", e);
    }
}
```

## CLI

The `billplz` binary wraps every SDK method as a subcommand with JSON output.

### Installation

```bash
cargo install billplz
```

### Configuration

Auth is loaded from environment variables or a config file (`~/.billplz/config.toml`). Env vars take priority.

**Environment variables:**

```bash
export BILLPLZ_API_KEY="your-api-key"
export BILLPLZ_ENVIRONMENT="staging"  # or "production" (default)
```

**Config file** (`~/.billplz/config.toml`):

```toml
api_key = "your-api-key"
environment = "staging"
```

### Commands

```bash
# Collections
billplz collection get <id>
billplz collection create --title "My Collection" --split-header

# Bills
billplz bill get <id>
billplz bill create --collection-id <id> --email user@example.com \
  --name "John Doe" --amount 10000 --callback-url https://example.com/callback \
  --description "Invoice #123" --due-at "2024-12-31"

# Banks
billplz bank fpx-list
billplz bank verify <account-number>
billplz bank create-verification --name "John Doe" --id-no 91234567890 \
  --acc-no 999988887777 --code MBBEMYKL

# Payouts
billplz payout get <id>
billplz payout create --collection-id <id> --bank-code MBBEMYKL \
  --acc-no 999988887777 --id-no 91234567890 --name "John Doe" \
  --description "Salary" --total 50000

# Payout Collections
billplz payout-collection get <id>
billplz payout-collection create --title "Salary Payments"
```

Use `--pretty` for formatted JSON output:

```bash
billplz --pretty collection get ei3a6mdl
```

## MCP Server

The `billplz mcp` subcommand starts an MCP (Model Context Protocol) server over stdio, so AI agents can interact with the Billplz API.

### Setup

Add to your MCP client config (e.g. Claude Desktop `claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "billplz": {
      "command": "billplz",
      "args": ["mcp"],
      "env": {
        "BILLPLZ_API_KEY": "your-api-key",
        "BILLPLZ_ENVIRONMENT": "staging"
      }
    }
  }
}
```

### Available Tools

| Tool | Description |
|------|-------------|
| `get_collection` | Get a collection by ID |
| `create_collection` | Create a new collection |
| `get_bill` | Get a bill by ID |
| `create_bill` | Create a new bill |
| `get_fpx_banks` | List FPX banks |
| `get_bank_verification` | Get bank verification status |
| `create_bank_verification` | Create a bank verification |
| `get_payout` | Get a payout by ID |
| `create_payout` | Create a payout |
| `get_payout_collection` | Get a payout collection by ID |
| `create_payout_collection` | Create a payout collection |

## Reference

- [Billplz API Documentation](https://billplz.com/api)
- [Billplz Dev Jam (Facebook)](https://www.facebook.com/groups/billplzdevjam/)

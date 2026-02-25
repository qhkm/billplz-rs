# billplz CLI + MCP Server Design

## Goal

Add a CLI binary and MCP server to billplz-rs so both humans and AI agents can interact with the Billplz API from the command line.

## CLI (`billplz`)

Wraps every SDK method as a subcommand. JSON output by default, `--pretty` for formatted output.

### Auth

1. Env vars: `BILLPLZ_API_KEY`, `BILLPLZ_ENVIRONMENT`
2. Fallback: `~/.billplz/config.toml`

```toml
api_key = "your-key"
environment = "staging"
```

Env vars take priority over config file.

### Commands

```
billplz collection get <id>
billplz collection create --title <title> [--split-header] [--split-payment <email:fixed_cut:stack_order>...]
billplz bill get <id>
billplz bill create --collection-id <id> --email <email> --name <name> --amount <cents> --callback-url <url> --description <desc> --due-at <date> [--mobile <mobile>] [--redirect-url <url>] [--reference-1-label <l>] [--reference-1 <v>] [--reference-2-label <l>] [--reference-2 <v>]
billplz bank fpx-list
billplz bank verify <account-number>
billplz bank create-verification --name <n> --id-no <id> --acc-no <acc> --code <code> [--organization]
billplz payout get <id>
billplz payout create --collection-id <id> --bank-code <code> --acc-no <acc> --id-no <id> --name <n> --description <d> --total <cents>
billplz payout-collection get <id>
billplz payout-collection create --title <title>
```

### Output

- Default: JSON (for AI agents)
- `--pretty`: formatted JSON with indentation

## MCP Server (`billplz mcp`)

Runs as stdio MCP server. AI agents connect via MCP config.

### Tools

- `get_collection`, `create_collection`
- `get_bill`, `create_bill`
- `get_fpx_banks`, `get_bank_verification`, `create_bank_verification`
- `get_payout`, `create_payout`
- `get_payout_collection`, `create_payout_collection`

Each tool has typed input schema matching the CLI flags.

## Dependencies

- `clap` (derive) — CLI parsing
- `rmcp` — Rust MCP SDK
- `toml` + `serde` — config file
- `dirs` — home directory path

## Structure

```
src/
  lib.rs
  main.rs
  cli/
    mod.rs
    config.rs
    collection.rs
    bill.rs
    bank.rs
    payout.rs
    payout_collection.rs
  mcp/
    mod.rs
```

`Cargo.toml` adds `[[bin]]` target alongside `[lib]`.

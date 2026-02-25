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
    /// Start MCP server (stdio transport)
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
#[allow(clippy::large_enum_variant)]
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
            let config = config::Config::load(None)?;
            let client = config.into_client();
            crate::mcp::start_mcp_server(client).await
        }
        _ => {
            let config = config::Config::load(None)?;
            let client = config.into_client();
            execute_command(&cli, &client).await
        }
    }
}

async fn execute_command(
    cli: &Cli,
    client: &crate::BillplzClient,
) -> Result<(), Box<dyn std::error::Error>> {
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
                collection_id,
                email,
                name,
                amount,
                callback_url,
                description,
                due_at,
                mobile,
                redirect_url,
                reference_1_label,
                reference_1,
                reference_2_label,
                reference_2,
            } => {
                let mut builder = client.create_bill(
                    collection_id,
                    email,
                    name,
                    *amount,
                    callback_url,
                    description,
                    due_at,
                );
                if let Some(m) = mobile {
                    builder = builder.mobile(m);
                }
                if let Some(r) = redirect_url {
                    builder = builder.redirect_url(r);
                }
                if let Some(l) = reference_1_label {
                    builder = builder.reference_1_label(l);
                }
                if let Some(v) = reference_1 {
                    builder = builder.reference_1(v);
                }
                if let Some(l) = reference_2_label {
                    builder = builder.reference_2_label(l);
                }
                if let Some(v) = reference_2 {
                    builder = builder.reference_2(v);
                }
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
                let value: serde_json::Value = serde_json::from_str(&result)
                    .unwrap_or(serde_json::Value::String(result));
                output_json(&value, cli.pretty);
            }
            BankAction::CreateVerification {
                name,
                id_no,
                acc_no,
                code,
                organization,
            } => {
                let result = client
                    .create_bank_verification(name, id_no, acc_no, code)
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
                collection_id,
                bank_code,
                acc_no,
                id_no,
                name,
                description,
                total,
            } => {
                let result = client
                    .create_payout(
                        collection_id,
                        bank_code,
                        acc_no,
                        id_no,
                        name,
                        description,
                        *total,
                    )
                    .send()
                    .await?;
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

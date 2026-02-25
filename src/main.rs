#[tokio::main]
async fn main() {
    if let Err(e) = billplz::cli::run().await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

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

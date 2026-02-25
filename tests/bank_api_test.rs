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

    let resp = client.get_bank_verification("999988887777").await.unwrap();
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

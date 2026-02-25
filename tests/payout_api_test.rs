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

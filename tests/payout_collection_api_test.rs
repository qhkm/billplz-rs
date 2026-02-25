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

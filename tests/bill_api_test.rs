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

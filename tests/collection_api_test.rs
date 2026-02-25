use billplz::{BillplzClient, BillplzError};
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, header};

#[tokio::test]
async fn test_get_collection_success() {
    let mock_server = MockServer::start().await;
    let client = BillplzClient::with_base_url(&mock_server.uri(), "test-api-key");

    Mock::given(method("GET"))
        .and(path("/api/v4/collections/col123"))
        .and(header("authorization", "Basic dGVzdC1hcGkta2V5Og=="))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "col123",
            "title": "My Collection",
            "logo": {
                "thumb_url": "https://example.com/thumb.png",
                "avatar_url": "https://example.com/avatar.png"
            },
            "status": "active"
        })))
        .mount(&mock_server)
        .await;

    let resp = client.get_collection("col123").await.unwrap();
    assert_eq!(resp.id, "col123");
    assert_eq!(resp.title, "My Collection");
    assert_eq!(resp.status, "active");
}

#[tokio::test]
async fn test_get_collection_api_error() {
    let mock_server = MockServer::start().await;
    let client = BillplzClient::with_base_url(&mock_server.uri(), "bad-key");

    Mock::given(method("GET"))
        .and(path("/api/v4/collections/col123"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "error": {
                "type": "unauthorized",
                "message": "Invalid API key"
            }
        })))
        .mount(&mock_server)
        .await;

    let err = client.get_collection("col123").await.unwrap_err();
    match err {
        BillplzError::Api { error_type, message } => {
            assert_eq!(error_type, "unauthorized");
            assert_eq!(message, "Invalid API key");
        }
        _ => panic!("Expected Api error, got {:?}", err),
    }
}

#[tokio::test]
async fn test_create_collection_success() {
    let mock_server = MockServer::start().await;
    let client = BillplzClient::with_base_url(&mock_server.uri(), "test-api-key");

    Mock::given(method("POST"))
        .and(path("/api/v4/collections"))
        .and(header("content-type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "new_col",
            "title": "New Collection",
            "status": "active"
        })))
        .mount(&mock_server)
        .await;

    let resp = client
        .create_collection("New Collection")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.id, "new_col");
    assert_eq!(resp.title, "New Collection");
}

#[tokio::test]
async fn test_create_collection_with_split_payments() {
    let mock_server = MockServer::start().await;
    let client = BillplzClient::with_base_url(&mock_server.uri(), "test-api-key");

    Mock::given(method("POST"))
        .and(path("/api/v4/collections"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "split_col",
            "title": "Split Collection",
            "split_header": true,
            "status": "active"
        })))
        .mount(&mock_server)
        .await;

    let resp = client
        .create_collection("Split Collection")
        .split_header(true)
        .split_payment("a@test.com", 0)
        .split_payment_with_fixed_cut("b@test.com", 100, 1)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.id, "split_col");
}

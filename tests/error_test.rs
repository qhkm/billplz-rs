use billplz::BillplzError;

#[test]
fn test_api_error_display() {
    let err = BillplzError::Api {
        error_type: "unauthorized".to_string(),
        message: "Invalid API key".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "API error (unauthorized): Invalid API key"
    );
}

#[test]
fn test_api_error_debug() {
    let err = BillplzError::Api {
        error_type: "not_found".to_string(),
        message: "Bill not found".to_string(),
    };
    let debug = format!("{:?}", err);
    assert!(debug.contains("not_found"));
    assert!(debug.contains("Bill not found"));
}

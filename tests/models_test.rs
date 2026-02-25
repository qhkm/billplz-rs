use billplz::models::{
    bill::{Bill, BillResponse},
    collection::{Collection, CollectionResponse, SplitPayment},
    bank::{Bank, FpxBank},
    payout::Payout,
    payout_collection::PayoutCollection,
};

#[test]
fn test_bill_serialize() {
    let bill = Bill {
        collection_id: "abc123".to_string(),
        email: "test@test.com".to_string(),
        mobile: Some("601234567890".to_string()),
        name: "Test User".to_string(),
        amount: 10000,
        callback_url: "https://example.com/callback".to_string(),
        description: "Test bill".to_string(),
        due_at: "2024-07-12".to_string(),
        redirect_url: None,
        deliver: None,
        reference_1_label: None,
        reference_1: None,
        reference_2_label: None,
        reference_2: None,
    };
    let json = serde_json::to_value(&bill).unwrap();
    assert_eq!(json["collection_id"], "abc123");
    assert_eq!(json["amount"], 10000);
    assert_eq!(json["email"], "test@test.com");
}

#[test]
fn test_bill_response_deserialize() {
    let json = r#"{
        "id": "bill123",
        "collection_id": "abc123",
        "email": "test@test.com",
        "mobile": "601234567890",
        "name": "Test User",
        "amount": 10000,
        "callback_url": "https://example.com/callback",
        "description": "Test bill",
        "due_at": "2024-07-12",
        "paid": false,
        "state": "due",
        "paid_amount": 0,
        "url": "https://www.billplz.com/bills/bill123"
    }"#;
    let resp: BillResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.id, "bill123");
    assert!(!resp.paid);
    assert_eq!(resp.state, "due");
    assert_eq!(resp.amount, 10000);
}

#[test]
fn test_collection_serialize() {
    let collection = Collection {
        title: "My Collection".to_string(),
        split_header: None,
        split_payments: None,
    };
    let json = serde_json::to_value(&collection).unwrap();
    assert_eq!(json["title"], "My Collection");
}

#[test]
fn test_collection_with_split_payments_serialize() {
    let collection = Collection {
        title: "Split Collection".to_string(),
        split_header: Some(true),
        split_payments: Some(vec![
            SplitPayment {
                email: "a@test.com".to_string(),
                fixed_cut: Some(100),
                variable_cut: None,
                stack_order: 0,
            },
        ]),
    };
    let json = serde_json::to_value(&collection).unwrap();
    assert_eq!(json["split_payments"][0]["email"], "a@test.com");
    assert_eq!(json["split_payments"][0]["fixed_cut"], 100);
}

#[test]
fn test_collection_response_deserialize() {
    let json = r#"{
        "id": "col123",
        "title": "My Collection",
        "logo": {
            "thumb_url": "https://example.com/thumb.png",
            "avatar_url": "https://example.com/avatar.png"
        },
        "status": "active"
    }"#;
    let resp: CollectionResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.id, "col123");
    assert_eq!(resp.title, "My Collection");
    assert_eq!(resp.status, "active");
}

#[test]
fn test_bank_serialize() {
    let bank = Bank {
        name: "Test User".to_string(),
        id_no: "91234567890".to_string(),
        acc_no: "999988887777".to_string(),
        code: "MBBEMYKL".to_string(),
        organization: true,
    };
    let json = serde_json::to_value(&bank).unwrap();
    assert_eq!(json["name"], "Test User");
    assert_eq!(json["code"], "MBBEMYKL");
    assert!(json["organization"].as_bool().unwrap());
}

#[test]
fn test_fpx_bank_deserialize() {
    let json = r#"{"bank_code": "MB2U0227", "bank_name": "Maybank2u"}"#;
    let bank: FpxBank = serde_json::from_str(json).unwrap();
    assert_eq!(bank.bank_code, "MB2U0227");
    assert_eq!(bank.bank_name, "Maybank2u");
}

#[test]
fn test_payout_serialize() {
    let payout = Payout {
        mass_payment_instruction_collection_id: "col123".to_string(),
        bank_code: "MBBEMYKL".to_string(),
        bank_account_number: "999988887777".to_string(),
        identity_number: "91234567890".to_string(),
        name: "Test User".to_string(),
        description: "Payout test".to_string(),
        total: 50000,
    };
    let json = serde_json::to_value(&payout).unwrap();
    assert_eq!(json["total"], 50000);
    assert_eq!(json["bank_code"], "MBBEMYKL");
}

#[test]
fn test_payout_collection_serialize() {
    let pc = PayoutCollection {
        title: "My Payout Collection".to_string(),
    };
    let json = serde_json::to_value(&pc).unwrap();
    assert_eq!(json["title"], "My Payout Collection");
}

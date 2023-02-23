mod setup;

use crate::setup::*;

#[tokio::test]
async fn get_details_returns_201_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = serde_json::json!(
        {
            "user": {
                "email": "toto@email.com",
              "password": "my super pasword",
                "user_name": "toto"
            },
            "car_info": {
                "car_model": "tesla",
                "car_plate": "42"
            },
            "bank_details": {
                "account_holder": "toto",
                "bank_country": "france",
                "iban": "12345"
            }
        }
    );

    // Act
    let response = client
        // Use the returned application address
        .post(&format!("{}/api/account", &app.address))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

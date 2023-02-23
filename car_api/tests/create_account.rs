mod setup;

use crate::setup::*;

#[tokio::test]
async fn create_returns_201_for_valid_form_data() {
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

#[tokio::test]
async fn create_returns_422_for_missing_data() {
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
    assert_eq!(422, response.status().as_u16(),);
}

// curl --request POST \
//   --url http://localhost:8080/api/account \
//   --header 'Content-Type: application/json' \
//   --cookie axum.sid=gfF3AnSz1Pwxn6HfBZgM4SMlC%2Fjj4FQS8Gol7QU7SPw%3Dm%2F9IUzJcGyt%2FgZUkPF%2BAa14uzzim%2FUuhZf3kVqEjdbIQ1JAW9FTpEZ7YlfuWbYo%2BRf5lqQjyMUHrjrVHmcegLw%3D%3D \
//   --data '{
// 	"user": {
// 		"email": "toto@email.com",
//   	"password": "my super pasword",
// 		"user_name": "toto"
// 	},
// 	"car_info": {
// 		"car_model": "tesla",
// 		"car_plate": "42"
// 	},
// 	"bank_details": {
// 		"account_holder": "toto",
// 		"bank_country": "france",
// 		"iban": "12345"
// 	}
// }'

mod setup;

use crate::setup::*;

#[tokio::test]
async fn create_account_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .post(&format!("{}/api/account", &app.address))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
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

mod setup;

use car_api::routes::account::AccountDetails;

use crate::setup::*;

use reqwest::{Client, Response};

async fn login(app: &TestApp, client: &Client) -> Response {
    let login_body = serde_json::json!({
        "email": "toto@email.com",
        "password_hash": "my super password"
    });

    let response = client
        .post(&format!("{}/login", &app.address))
        .header("Content-Type", "application/json")
        .json(&login_body)
        .send()
        .await
        .expect("Failed to execute request.");

    response
}

async fn logout(app: &TestApp, client: &Client) -> Response {
    let response = client
        .get(&format!("{}/logout", &app.address))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    response
}

async fn create_account(app: &TestApp, client: &Client) -> Response {
    let body = serde_json::json!(
        {
            "user": {
                "email": "toto@email.com",
                "password": "my super password",
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

    let response = client
        .post(&format!("{}/api/account", &app.address))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    response
}

async fn get_account_details(app: &TestApp, client: &Client) -> AccountDetails {
    let response = client
        .get(&format!("{}/api/account", &app.address))
        .header("Content-Type", "application/json")
        // .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    let value = response
        .json::<AccountDetails>()
        .await
        .expect("Failed to execute request.");

    eprintln!("{:?}", value);

    // response
    value
}

#[tokio::test]
async fn login_get_details_logout() {
    // Arrange
    let app = spawn_app().await;
    let client = Client::builder().cookie_store(true).build().unwrap();

    // Act
    let response = login(&app, &client).await;
    // Assert
    assert_eq!(401, response.status().as_u16());

    // Act
    let response = create_account(&app, &client).await;
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    // Act
    let response = login(&app, &client).await;
    // Assert
    assert_eq!(200, response.status().as_u16());

    // Act
    let _response = get_account_details(&app, &client).await;

    // Act
    let response = logout(&app, &client).await;
    // Assert
    assert_eq!(200, response.status().as_u16());

    let response = client
        .get(&format!("{}/api/account", &app.address))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(401, response.status().as_u16());
}

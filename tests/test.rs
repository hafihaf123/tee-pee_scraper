use dotenv::dotenv;
use std::env::var;
use tee_pee_scraper::authentication::Credentials;
use tee_pee_scraper::TeePeeClient;

#[test]
fn test_login_invalid() {
    let client = TeePeeClient::default();
    dotenv().unwrap();

    let invalid_credentials = Credentials::new("test_login_invalid").unwrap();
    invalid_credentials.set_password("invalid").unwrap();
    assert!(client.login(&invalid_credentials).is_err());

    assert!(client
        .get("https://skauting.tee-pee.com")
        .unwrap()
        .contains("Login"));

    invalid_credentials.remove_password().unwrap();
}

#[test]
fn test_login_valid() {
    let client = TeePeeClient::default();
    dotenv().unwrap();

    let credentials = Credentials::new(&var("TEST_USERNAME").unwrap()).unwrap();
    credentials
        .set_password(&var("TEST_CREDENTIAL").unwrap())
        .unwrap();

    assert!(client.login(&credentials).is_ok());
    assert!(!client
        .get("https://skauting.tee-pee.com")
        .unwrap()
        .contains("Login"));

    credentials.remove_password().unwrap();
}

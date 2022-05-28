use algonaut_algod::apis::common_api::health_check;
use algonaut_algod::apis::configuration::{Configuration, ApiKey};

#[tokio::main]
async fn main() {
    //algorand sandbox's configuration
    let configuration = Configuration {
        base_path: "http://localhost:4001".to_owned(),
        user_agent: Some("OpenAPI-Generator/0.0.1/rust".to_owned()),
        client: reqwest::Client::new(),
        basic_auth: None,
        oauth_access_token: None,
        bearer_access_token: None,
        api_key: Some(ApiKey {
            key: String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
            prefix: None,
        }),

    };

    println!("{:?}", health_check(&configuration).await);
}

use yup_oauth2::{ServiceAccountAuthenticator, InstalledFlowReturnMethod};
use std::default::Default;
use serde_json as json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = yup_oauth2::read_service_account_key("clientsecret.json")
        .await
        .expect("clientsecret.json");

    let mut auth = ServiceAccountAuthenticator::builder(secret)
    .persist_tokens_to_disk("tokencache.json")
    .build()
    .await
    .unwrap();

    let scopes = &["https://www.googleapis.com/auth/bigquery"];

    let token = match auth.token(scopes).await {
        Ok(token) => {
            println!("The token is {:?}", token);
            token
        }
        Err(e) => {
            println!("error: {:?}", e);
            return Ok(());
        },
    };
    dbg!(&token.as_str());

    let mut request_body = HashMap::new();
    let query = r#"
        SELECT
            t2.*
            ,t1.*
        FROM
            `hoge.fuga` AS t1
            INNER JOIN `piyo` AS t2
                ON t1.id = t2.id
    "#;
    request_body.insert("query", query);
    request_body.insert("useLegacySql", "false");
    let client = reqwest::Client::new();
    let resp = client.post("https://bigquery.googleapis.com/bigquery/v2/projects/beluga-dashboard/queries")
        .bearer_auth(&token.as_str())
        .json(&request_body)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

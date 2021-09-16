use ::reqwest::blocking::Client;
use anyhow::Result;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql};
use crate::github_queries::{test_query, TestQuery};

pub type GitObjectID = String;

pub fn run_query() -> Result<()> {
    let github_api_token =
        std::env::var("GITHUB_API_TOKEN").expect("Missing GITHUB_API_TOKEN env var");

    let variables = test_query::Variables {};

    let client = Client::builder()
        .user_agent("changelogs/0.0.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", github_api_token))
                    .unwrap(),
            ))
                .collect(),
        )
        .build()?;

    let response_body =
        post_graphql::<TestQuery, _>(&client, "https://api.github.com/graphql", variables).unwrap();

    println!("response_body {:#?}", response_body);

    let response_data: test_query::ResponseData = response_body.data.expect("missing response data");

    println!("response_data {:#?}", response_data);
    Ok(())
}

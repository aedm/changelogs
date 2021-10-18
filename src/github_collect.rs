use crate::github_queries::{pull_requests, PullRequests};
use ::reqwest::Client;
use anyhow::Result;
use graphql_client::reqwest::post_graphql;

pub type GitObjectID = String;

pub async fn run_query() -> Result<()> {
    let github_token = std::env::var("GITHUB_TOKEN").expect("Missing GITHUB_TOKEN env var");

    let client = Client::builder()
        .user_agent("changelogs/0.0.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", github_token))
                    .unwrap(),
            ))
            .collect(),
        )
        .build()?;

    cycle(&client).await;
    Ok(())
}

async fn cycle(client: &Client) -> Option<()> {
    let mut cursor = None;

    loop {
        let variables = pull_requests::Variables {
            owner: "meteor".to_string(),
            repository: "meteor".to_string(),
            branch: "devel".to_string(),
            cursor: cursor.clone(),
        };
        let response_body =
            post_graphql::<PullRequests, _>(&client, "https://api.github.com/graphql", variables)
                .await
                .unwrap();

        let response_data: pull_requests::ResponseData =
            response_body.data.expect("missing response data");

        for edge in response_data.repository?.pull_requests.edges?.iter() {
            cursor = None;
            if let Some(e) = edge {
                cursor = Some(e.cursor.to_string());
                if let Some(node) = &e.node {
                    println!(
                        "node sha={} number={}\n  msg: {}",
                        node.merge_commit.as_ref().unwrap().oid,
                        node.number,
                        node.body
                    );
                }
            }
        }
        if cursor.is_none() {
            break;
        }
    }

    Some(())
}

use crate::github_queries::{BranchCommit, CommitHistory};
use ::reqwest::Client;
use anyhow::Result;
use dotenv::var;
use graphql_client::reqwest::post_graphql;
use graphql_client::GraphQLQuery;
use std::collections::HashSet;

pub struct GithubContext {
    client: Client,
    pub owner: String,
    pub repository: String,
}

impl GithubContext {
    pub fn new(owner: &str, repository: &str) -> GithubContext {
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
            .build()
            .expect("Can't create HTTP client");

        GithubContext {
            client,
            owner: String::from(owner),
            repository: String::from(repository),
        }
    }
}

pub async fn query_github<Query: GraphQLQuery>(
    context: &GithubContext,
    variables: Query::Variables,
) -> Option<Query::ResponseData> {
    let response =
        post_graphql::<Query, _>(&context.client, "https://api.github.com/graphql", variables)
            .await;
    if let Err(err) = response {
        println!("Query failed: {:?}", err);
        None
    } else {
        response.unwrap().data
    }
}

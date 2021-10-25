use crate::github_queries::{
    branch_commit, commit_history, pull_requests, BranchCommit, CommitHistory, PullRequests,
};
use ::reqwest::Client;
use anyhow::Result;
use graphql_client::reqwest::post_graphql;
use std::collections::HashSet;

pub type GitObjectID = String;

pub async fn run_query() -> Option<()> {
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

    // fetch_pull_requests(&client).await;
    let owner = "meteor";
    let repository = "meteor";
    let branch = "devel";
    let commit_hash = fetch_commit_hash_from_branch(&client, &owner, &repository, &branch).await?;
    let res = fetch_commit_history(&client, &owner, &repository, &commit_hash, None).await?;
    println!("RES: {:?}", res);
    Some(())
}

async fn fetch_commit_hash_from_branch(
    client: &Client,
    owner: &str,
    repository: &str,
    branch: &str,
) -> Option<String> {
    let variables = branch_commit::Variables {
        owner: String::from(owner),
        repository: String::from(repository),
        branch: String::from(branch),
    };
    let response_body =
        post_graphql::<BranchCommit, _>(&client, "https://api.github.com/graphql", variables)
            .await
            .unwrap();

    let response_data: branch_commit::ResponseData =
        response_body.data.expect("missing response data");

    if let branch_commit::BranchCommitRepositoryRefTarget::Commit(commit) =
        &response_data.repository?.ref_?.target?
    {
        Some(commit.oid.clone())
    } else {
        None
    }
}

async fn fetch_pull_requests(client: &Client) -> Option<()> {
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

async fn fetch_commit_history(
    client: &Client,
    owner: &str,
    repository: &str,
    commit_hash: &str,
    cursor: Option<String>,
) -> Option<(Vec<String>, Option<String>)> {
    let variables = commit_history::Variables {
        owner: String::from(owner),
        repository: String::from(repository),
        oid: String::from(commit_hash),
        cursor: cursor.clone(),
    };
    let response_body =
        post_graphql::<CommitHistory, _>(&client, "https://api.github.com/graphql", variables)
            .await
            .unwrap();

    let response_data: commit_history::ResponseData =
        response_body.data.expect("missing response data");

    if let commit_history::CommitHistoryRepositoryObject::Commit(commit) =
        response_data.repository?.object?
    {
        let mut result_vec = vec![];
        let mut cursor = None;
        for history_item in &commit.history.edges? {
            let edge = history_item.as_ref()?;
            cursor = Some(edge.cursor.clone());
            let node = edge.node.as_ref()?;
            result_vec.push(node.oid.clone());
        }
        Some((result_vec, cursor))
    } else {
        None
    }
}

use crate::github_context::{query_github, GithubContext};
use crate::github_queries::{
    branch_commit, commit_history, pull_requests, BranchCommit, CommitHistory, PullRequests,
};
use ::reqwest::Client;
use anyhow::Result;
use chrono::prelude::*;
use dotenv::var;
use graphql_client::reqwest::post_graphql;
use graphql_client::GraphQLQuery;
use std::collections::HashSet;

pub type GitObjectID = String;
pub type DateTime = String;

type Date = chrono::DateTime<Utc>;

#[derive(Debug)]
struct PullRequest {
    commit_hash: String,
    number: i64,
    message: String,
    title: String,
    merged_at: Date,
}

#[derive(Debug)]
struct Commit {
    hash: String,
    date: Date,
}

pub async fn run_query() -> Option<()> {
    let context = GithubContext::new("meteor", "meteor");
    let until_branch = "devel";
    let since_branch = "release-2.5";

    let since_commit_hash = fetch_commit_hash_from_branch(&context, &since_branch).await?;
    let until_commit_hash = fetch_commit_hash_from_branch(&context, &until_branch).await?;
    let commits = fetch_commit_list(&context, &since_commit_hash, &until_commit_hash).await?;
    println!("Commits: {:?}", commits);

    let pull_requests = fetch_pull_requests(&context, &until_branch, &commits).await?;
    println!("Found {} pull requests:", pull_requests.len());

    pull_requests
        .iter()
        .for_each(|x| println!("{:?}", x.number));
    Some(())
}

fn parse_date(text: &str) -> Date {
    text.parse::<Date>().unwrap()
}

async fn fetch_commit_list(
    context: &GithubContext,
    since_commit_hash: &str,
    until_commit_hash: &str,
) -> Option<Vec<Commit>> {
    let mut since_set: HashSet<String> = HashSet::new();
    let mut until_set: HashSet<String> = HashSet::new();
    let mut until_list: Vec<Commit> = Vec::new();

    let mut since_cursor = None;
    let mut until_cursor = None;
    loop {
        let mut response =
            fetch_commit_history(&context, &until_commit_hash, &until_cursor).await?;
        until_cursor = response.1;
        until_set.extend(response.0.iter().map(|x| x.hash.clone()));
        until_list.append(&mut response.0);

        let response = fetch_commit_history(&context, &since_commit_hash, &since_cursor).await?;
        since_cursor = response.1;
        since_set.extend(response.0.iter().map(|x| x.hash.clone()));

        let intersection: HashSet<&String> = until_set.intersection(&since_set).collect();
        if intersection.len() > 0 {
            let commits: Vec<_> = until_list
                .into_iter()
                .take_while(|x| !intersection.contains(&x.hash))
                .collect();
            return Some(commits);
        }
    }
}

async fn fetch_commit_hash_from_branch(context: &GithubContext, branch: &str) -> Option<String> {
    let variables = branch_commit::Variables {
        owner: context.owner.clone(),
        repository: context.repository.clone(),
        branch: String::from(branch),
    };
    let response_data = query_github::<BranchCommit>(context, variables).await?;

    if let branch_commit::BranchCommitRepositoryRefTarget::Commit(commit) =
        &response_data.repository?.ref_?.target?
    {
        Some(commit.oid.clone())
    } else {
        None
    }
}

async fn fetch_pull_requests(
    context: &GithubContext,
    branch: &str,
    commits: &[Commit],
) -> Option<Vec<PullRequest>> {
    let commit_set: HashSet<_> = commits.iter().map(|x| &x.hash).collect();
    let mut pull_requests = Vec::new();
    let mut cursor = None;

    'outer: loop {
        let variables = pull_requests::Variables {
            owner: context.owner.clone(),
            repository: context.repository.clone(),
            branch: String::from(branch),
            cursor: cursor.clone(),
        };
        let response_data = query_github::<PullRequests>(context, variables).await?;
        let mut edges = response_data.repository?.pull_requests.edges?;
        // edges.reverse();
        for edge in &edges {
            cursor = None;
            if let Some(e) = edge {
                cursor = Some(e.cursor.to_string());
                if let Some(node) = &e.node {
                    // println!(
                    //     "node sha={} number={}\n  msg: {}",
                    //     node.merge_commit.as_ref().unwrap().oid,
                    //     node.number,
                    //     node.body
                    // );
                    let pr = PullRequest {
                        commit_hash: node.merge_commit.as_ref().unwrap().oid.clone(),
                        number: node.number,
                        message: node.body.clone(),
                        title: node.title.clone(),
                        merged_at: parse_date(&node.merged_at.as_ref()?),
                    };
                    println!("PR: {:#?}", pr);
                    if !commit_set.contains(&pr.commit_hash) {
                        break 'outer;
                    }
                    pull_requests.push(pr);
                }
            }
        }
        if cursor.is_none() {
            break;
        }
    }
    Some(pull_requests)
}

async fn fetch_commit_history(
    context: &GithubContext,
    commit_hash: &str,
    cursor: &Option<String>,
) -> Option<(Vec<Commit>, Option<String>)> {
    let variables = commit_history::Variables {
        owner: context.owner.clone(),
        repository: context.repository.clone(),
        oid: String::from(commit_hash),
        cursor: cursor.clone(),
    };
    let response_data = query_github::<CommitHistory>(&context, variables).await?;

    if let commit_history::CommitHistoryRepositoryObject::Commit(commit) =
        response_data.repository?.object?
    {
        let mut result_vec = vec![];
        let mut cursor = None;
        for history_item in &commit.history.edges? {
            let edge = history_item.as_ref()?;
            cursor = Some(edge.cursor.clone());
            let node = edge.node.as_ref()?;
            result_vec.push(Commit {
                date: parse_date(&node.committed_date),
                hash: node.oid.clone(),
            });
        }
        Some((result_vec, cursor))
    } else {
        None
    }
}

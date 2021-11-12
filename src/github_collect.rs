use crate::github_context::{query_github, GithubContext};
use crate::github_queries::{
    get_commit_history, get_commit_on_branch_head, get_pull_requests_ids_for_commit_ids,
    GetCommitHistory, GetCommitOnBranchHead, GetPullRequestsIdsForCommitIds,
};
use ::reqwest::Client;
use anyhow::Result;
use chrono::prelude::*;
use dotenv::var;
use graphql_client::reqwest::post_graphql;
use graphql_client::GraphQLQuery;
use std::cmp::min;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::iter::FromIterator;
use std::rc::Rc;

pub type GitObjectID = String;
pub type DateTime = String;

type Date = chrono::DateTime<Utc>;

#[derive(Debug)]
struct PullRequest {
    id: String,
    number: i64,
    title: String,
    body: String,
    merged_at: Date,
    commit_hash: String,
}

#[derive(Debug)]
struct Commit {
    id: String,
    hash: String,
    date: Date,
    first_parent_hash: Option<String>,
}

pub async fn run_query() -> Option<()> {
    let context = GithubContext::new("meteor", "meteor");
    let until_branch = "devel";
    let since_branch = "release-2.5";
    // let context = GithubContext::new("facebook", "react");
    // let until_branch = "17.0.2";
    // let since_branch = "16.8.6";

    let since_commit_hash = fetch_commit_hash_from_branch(&context, &since_branch).await?;
    let until_commit_hash = fetch_commit_hash_from_branch(&context, &until_branch).await?;
    let commits = fetch_commit_list(&context, &since_commit_hash, &until_commit_hash).await?;

    println!("Commits: {}", commits.len());

    fetch_pull_requests(&context, &commits).await?;
    // let prs: Vec<_> = commits.iter().flat_map(|x| &x.pull_requests).collect();
    // prs.iter()
    //     .for_each(|x| println!("PR {}: {}", x.number, x.title));
    // let pull_requests = fetch_pull_requests(&context, &until_branch, &commits).await?;
    // println!("Found {} pull requests:", pull_requests.len());

    // pull_requests
    //     .iter()
    //     .for_each(|x| println!("{:?}", x.number));
    Some(())
}

fn parse_date(text: &str) -> Date {
    text.parse::<Date>().unwrap()
}

fn get_main_commit_line(
    head_hash: &str,
    commits_by_hash: &HashMap<String, Rc<Commit>>,
) -> Vec<Rc<Commit>> {
    let mut commits = vec![];
    let mut hash = head_hash;
    loop {
        if let Some(commit) = commits_by_hash.get(hash) {
            commits.push(commit.clone());
            if let Some(x) = &commit.first_parent_hash {
                hash = x;
                continue;
            }
        }
        break;
    }
    commits
}

async fn fetch_pull_requests(
    context: &GithubContext,
    commits: &[Rc<Commit>],
) -> Option<Vec<Rc<PullRequest>>> {
    println!("Fetching pull request ids for {} commits...", commits.len());

    let mut pr_ids = vec![];

    const PAGE_SIZE: usize = 100;
    for page in 0..=(commits.len() / PAGE_SIZE) {
        println!("Fetching page {}...", page);
        let first = page * PAGE_SIZE;
        let last = min(first + PAGE_SIZE, commits.len());
        let commit_ids: Vec<_> = commits[first..last]
            .iter()
            .map(|it| it.id.clone())
            .collect();
        let variables = get_pull_requests_ids_for_commit_ids::Variables { commit_ids };
        let response_data =
            query_github::<GetPullRequestsIdsForCommitIds>(&context, variables).await?;
        println!("RateLimit {:?}", response_data.rate_limit);
        for node in &response_data.nodes {
            if let get_pull_requests_ids_for_commit_ids::GetPullRequestsIdsForCommitIdsNodes::Commit(commit) = node.as_ref()? {
                for n2 in commit.associated_pull_requests.as_ref()?.nodes.as_ref()? {
                    pr_ids.push(n2.as_ref()?.id.clone());
                }
            }
        }
    }

    println!("Pull request ids: {:?}", pr_ids);

    // if let get_commit_history::GetCommitHistoryRepositoryObject::Commit(commit) =
    // response_data.repository?.object?
    // {
    //     let mut result_vec = vec![];
    //     let mut cursor = None;
    //     for history_item in &commit.history.edges? {
    //         let edge = history_item.as_ref()?;
    //         cursor = Some(edge.cursor.clone());
    //         let node = edge.node.as_ref()?;
    //         let hash = node.oid.clone();
    //         let first_parent = node.parents.nodes.as_ref()?.first();
    //         let first_parent_hash = if let Some(x) = first_parent {
    //             Some(x.as_ref()?.oid.clone())
    //         } else {
    //             None
    //         };
    //         result_vec.push(Rc::new(Commit {
    //             date: parse_date(&node.committed_date),
    //             hash,
    //             id: node.id.clone(),
    //             first_parent_hash,
    //         }));
    //     }
    //     Some((result_vec, cursor))
    // } else {
    //     None
    // }

    None
}

async fn fetch_commit_list(
    context: &GithubContext,
    since_commit_hash: &str,
    until_commit_hash: &str,
) -> Option<Vec<Rc<Commit>>> {
    let mut since_commits_by_hash: HashMap<String, Rc<Commit>> = HashMap::new();
    let mut until_commits_by_hash: HashMap<String, Rc<Commit>> = HashMap::new();

    let mut since_cursor = None;
    let mut until_cursor = None;
    loop {
        let mut response =
            fetch_commit_history(&context, &until_commit_hash, &until_cursor).await?;
        until_cursor = response.1;
        for commit in response.0.into_iter() {
            until_commits_by_hash.insert(commit.hash.clone(), commit);
        }

        let response = fetch_commit_history(&context, &since_commit_hash, &since_cursor).await?;
        since_cursor = response.1;
        for commit in response.0.into_iter() {
            since_commits_by_hash.insert(commit.hash.clone(), commit);
        }

        let since_list = get_main_commit_line(since_commit_hash, &since_commits_by_hash);
        let until_list = get_main_commit_line(until_commit_hash, &until_commits_by_hash);

        let since_hashes_set: HashSet<_> = since_list.iter().map(|it| it.hash.clone()).collect();
        let until_hashes_set: HashSet<_> = until_list.iter().map(|it| it.hash.clone()).collect();
        let intersection: HashSet<String> = until_hashes_set
            .intersection(&since_hashes_set)
            .cloned()
            .collect();

        if intersection.len() > 0 {
            let commits: Vec<_> = until_list
                .into_iter()
                .filter(|it| !intersection.contains(&it.hash))
                .collect();
            return Some(commits);
        }
    }
}

async fn fetch_commit_hash_from_branch(context: &GithubContext, branch: &str) -> Option<String> {
    let variables = get_commit_on_branch_head::Variables {
        owner: context.owner.clone(),
        repository: context.repository.clone(),
        branch: String::from(branch),
    };
    let response_data = query_github::<GetCommitOnBranchHead>(context, variables).await?;

    if let get_commit_on_branch_head::GetCommitOnBranchHeadRepositoryRefTarget::Commit(commit) =
        &response_data.repository?.ref_?.target?
    {
        Some(commit.oid.clone())
    } else {
        None
    }
}

async fn fetch_commit_history(
    context: &GithubContext,
    commit_hash: &str,
    cursor: &Option<String>,
) -> Option<(Vec<Rc<Commit>>, Option<String>)> {
    println!("Fetching commit history...");
    let variables = get_commit_history::Variables {
        owner: context.owner.clone(),
        repository: context.repository.clone(),
        oid: String::from(commit_hash),
        cursor: cursor.clone(),
    };
    let response_data = query_github::<GetCommitHistory>(&context, variables).await?;

    println!("RateLimit {:?}", response_data.rate_limit);

    if let get_commit_history::GetCommitHistoryRepositoryObject::Commit(commit) =
        response_data.repository?.object?
    {
        let mut result_vec = vec![];
        let mut cursor = None;
        for history_item in &commit.history.edges? {
            let edge = history_item.as_ref()?;
            cursor = Some(edge.cursor.clone());
            let node = edge.node.as_ref()?;
            let hash = node.oid.clone();
            let first_parent = node.parents.nodes.as_ref()?.first();
            let first_parent_hash = if let Some(x) = first_parent {
                Some(x.as_ref()?.oid.clone())
            } else {
                None
            };
            result_vec.push(Rc::new(Commit {
                date: parse_date(&node.committed_date),
                hash,
                id: node.id.clone(),
                first_parent_hash,
            }));
        }
        Some((result_vec, cursor))
    } else {
        None
    }
}

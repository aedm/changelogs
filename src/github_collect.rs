use crate::github_context::{query_github, GithubContext};
use crate::github_queries::get_pull_requests_by_id::PullRequestState::MERGED;
use crate::github_queries::{
    get_commit_history, get_commit_on_branch_head, get_pull_requests_by_id,
    get_pull_requests_ids_for_commit_ids, GetCommitHistory, GetCommitOnBranchHead,
    GetPullRequestsById, GetPullRequestsIdsForCommitIds,
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
    merged_at: Option<Date>,
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
    // let context = GithubContext::new("meteor", "meteor");
    // let until_branch = "devel";
    // let since_branch = "release-2.5";
    // let context = GithubContext::new("facebook", "react");
    // let until_branch = "17.0.2";
    // let since_branch = "16.8.6";
    let context = GithubContext::new("aedm", "changelogs-test");
    let until_branch = "2-feature-branch";
    let since_branch = "1-feature-branch";

    let since_commit_hash = fetch_commit_hash_from_branch(&context, &since_branch).await?;
    let until_commit_hash = fetch_commit_hash_from_branch(&context, &until_branch).await?;
    let commits = fetch_commit_list(&context, &since_commit_hash, &until_commit_hash).await?;

    println!("Commits: {}", commits.len());

    let prs = fetch_pull_requests(&context, &commits).await.unwrap();
    println!("PRs: {:#?}", prs);

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

    let mut pr_ids_by_commit_hash = HashMap::new();

    const PAGE_SIZE: usize = 100;
    for page in 0..=(commits.len() / PAGE_SIZE) {
        println!("Fetching PR id page {}...", page);
        let first = page * PAGE_SIZE;
        let last = min(first + PAGE_SIZE, commits.len());
        let commit_ids: Vec<_> = commits[first..last]
            .iter()
            .map(|it| it.id.clone())
            .collect();
        let variables = get_pull_requests_ids_for_commit_ids::Variables { commit_ids };
        let response_data = query_github::<GetPullRequestsIdsForCommitIds>(&context, variables)
            .await
            .unwrap();
        println!("RateLimit {:?}", response_data.rate_limit);
        for node in response_data.nodes {
            println!("---1");
            if let get_pull_requests_ids_for_commit_ids::GetPullRequestsIdsForCommitIdsNodes::Commit(commit) = node.unwrap() {
                println!("---2");
                let oid = commit.oid;
                println!("---3");
                let asdf: Vec<_> = commit.associated_pull_requests.unwrap().nodes.unwrap().into_iter().map(|it| it.unwrap().id).collect();
                println!("---4");
                pr_ids_by_commit_hash.insert(oid, asdf);
                println!("---5");
            }
        }
    }

    println!("Pull request ids: {:?}", pr_ids_by_commit_hash);
    let pr_set: HashSet<_> = pr_ids_by_commit_hash
        .iter()
        .flat_map(|it| it.1)
        .cloned()
        .collect();
    let pr_list: Vec<_> = pr_set.into_iter().collect();
    let mut pull_requests = vec![];

    for page in 0..=(pr_list.len() / PAGE_SIZE) {
        println!("Fetching PR page {}...", page);
        let first = page * PAGE_SIZE;
        let last = min(first + PAGE_SIZE, pr_list.len());
        let pr_ids: Vec<_> = pr_list[first..last].iter().cloned().collect();
        let variables = get_pull_requests_by_id::Variables { pr_ids };
        let response_data = query_github::<GetPullRequestsById>(&context, variables)
            .await
            .unwrap();
        println!("RateLimit {:?}", response_data.rate_limit);
        for node in response_data.nodes {
            if let get_pull_requests_by_id::GetPullRequestsByIdNodes::PullRequest(pr) =
                node.unwrap()
            {
                println!("PR {:#?}", pr);
                if !matches!(pr.state, MERGED) {
                    continue;
                }
                let pull_reuest = PullRequest {
                    id: pr.id,
                    title: pr.title,
                    number: pr.number,
                    body: pr.body,
                    merged_at: pr.merged_at.and_then(|s| Some(parse_date(&s))),
                    commit_hash: pr.merge_commit.unwrap().oid,
                };
                if let Some(x) = pr_ids_by_commit_hash.get(&pull_reuest.commit_hash) {
                    if x.contains(&pull_reuest.id) {
                        pull_requests.push(Rc::new(pull_reuest));
                    }
                }
            }
        }
    }

    pull_requests.sort_by_key(|it| it.merged_at);
    Some(pull_requests)
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
        response_data.repository?.ref_?.target?
    {
        Some(commit.oid)
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
        for history_item in commit.history.edges? {
            let edge = history_item?;
            cursor = Some(edge.cursor);
            let node = edge.node?;
            let hash = node.oid;
            let first_parent_hash = if let Some(x) = node.parents.edges.unwrap().first() {
                x.as_ref()
                    .and_then(|edge| edge.node.as_ref())
                    .and_then(|node| Some(node.oid.clone()))
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

pub struct PullRequests;
pub mod pull_requests {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "PullRequests";
    pub const QUERY : & str = "query PullRequests($owner: String!, $repository: String!, $branch: String!, $cursor: String) {\n  repository(owner: $owner, name: $repository) {\n    pullRequests(states: [MERGED], first: 3, baseRefName: $branch, after: $cursor) {\n      edges {\n        cursor\n        node {\n          body\n          number\n          baseRefName\n          headRefName\n          mergeCommit {\n            oid\n          }\n        }\n      }\n    }\n  }\n}\n\nquery BranchCommit($owner: String!, $repository: String!, $branch: String!) {\n  repository(owner: $owner, name: $repository) {\n    ref(qualifiedName: $branch) {\n      name\n      target {\n        __typename\n        ... on Commit {\n          oid\n        }\n      }\n    }\n  }\n}\n\nquery CommitHistory($owner: String!, $repository: String!, $oid: GitObjectID!, $cursor: String) {\n  repository(owner: $owner, name: $repository) {\n    object(oid: $oid) {\n      __typename\n      ... on Commit {\n        oid\n        history(first: 5, after: $cursor) {\n          totalCount\n          edges {\n            node {\n              oid\n            }\n          }\n        }\n      }\n    }\n  }\n}\n\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type GitObjectID = crate::github_collect::GitObjectID;
    #[derive(Serialize)]
    pub struct Variables {
        pub owner: String,
        pub repository: String,
        pub branch: String,
        pub cursor: Option<String>,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub repository: Option<PullRequestsRepository>,
    }
    #[derive(Deserialize, Debug)]
    pub struct PullRequestsRepository {
        #[serde(rename = "pullRequests")]
        pub pull_requests: PullRequestsRepositoryPullRequests,
    }
    #[derive(Deserialize, Debug)]
    pub struct PullRequestsRepositoryPullRequests {
        pub edges: Option<Vec<Option<PullRequestsRepositoryPullRequestsEdges>>>,
    }
    #[derive(Deserialize, Debug)]
    pub struct PullRequestsRepositoryPullRequestsEdges {
        pub cursor: String,
        pub node: Option<PullRequestsRepositoryPullRequestsEdgesNode>,
    }
    #[derive(Deserialize, Debug)]
    pub struct PullRequestsRepositoryPullRequestsEdgesNode {
        pub body: String,
        pub number: Int,
        #[serde(rename = "baseRefName")]
        pub base_ref_name: String,
        #[serde(rename = "headRefName")]
        pub head_ref_name: String,
        #[serde(rename = "mergeCommit")]
        pub merge_commit: Option<PullRequestsRepositoryPullRequestsEdgesNodeMergeCommit>,
    }
    #[derive(Deserialize, Debug)]
    pub struct PullRequestsRepositoryPullRequestsEdgesNodeMergeCommit {
        pub oid: GitObjectID,
    }
}
impl graphql_client::GraphQLQuery for PullRequests {
    type Variables = pull_requests::Variables;
    type ResponseData = pull_requests::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: pull_requests::QUERY,
            operation_name: pull_requests::OPERATION_NAME,
        }
    }
}
pub struct BranchCommit;
pub mod branch_commit {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "BranchCommit";
    pub const QUERY : & str = "query PullRequests($owner: String!, $repository: String!, $branch: String!, $cursor: String) {\n  repository(owner: $owner, name: $repository) {\n    pullRequests(states: [MERGED], first: 3, baseRefName: $branch, after: $cursor) {\n      edges {\n        cursor\n        node {\n          body\n          number\n          baseRefName\n          headRefName\n          mergeCommit {\n            oid\n          }\n        }\n      }\n    }\n  }\n}\n\nquery BranchCommit($owner: String!, $repository: String!, $branch: String!) {\n  repository(owner: $owner, name: $repository) {\n    ref(qualifiedName: $branch) {\n      name\n      target {\n        __typename\n        ... on Commit {\n          oid\n        }\n      }\n    }\n  }\n}\n\nquery CommitHistory($owner: String!, $repository: String!, $oid: GitObjectID!, $cursor: String) {\n  repository(owner: $owner, name: $repository) {\n    object(oid: $oid) {\n      __typename\n      ... on Commit {\n        oid\n        history(first: 5, after: $cursor) {\n          totalCount\n          edges {\n            node {\n              oid\n            }\n          }\n        }\n      }\n    }\n  }\n}\n\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type GitObjectID = crate::github_collect::GitObjectID;
    #[derive(Serialize)]
    pub struct Variables {
        pub owner: String,
        pub repository: String,
        pub branch: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub repository: Option<BranchCommitRepository>,
    }
    #[derive(Deserialize, Debug)]
    pub struct BranchCommitRepository {
        #[serde(rename = "ref")]
        pub ref_: Option<BranchCommitRepositoryRef>,
    }
    #[derive(Deserialize, Debug)]
    pub struct BranchCommitRepositoryRef {
        pub name: String,
        pub target: Option<BranchCommitRepositoryRefTarget>,
    }
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum BranchCommitRepositoryRefTarget {
        Blob,
        Commit(BranchCommitRepositoryRefTargetOnCommit),
        Tag,
        Tree,
    }
    #[derive(Deserialize, Debug)]
    pub struct BranchCommitRepositoryRefTargetOnCommit {
        pub oid: GitObjectID,
    }
}
impl graphql_client::GraphQLQuery for BranchCommit {
    type Variables = branch_commit::Variables;
    type ResponseData = branch_commit::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: branch_commit::QUERY,
            operation_name: branch_commit::OPERATION_NAME,
        }
    }
}
pub struct CommitHistory;
pub mod commit_history {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CommitHistory";
    pub const QUERY : & str = "query PullRequests($owner: String!, $repository: String!, $branch: String!, $cursor: String) {\n  repository(owner: $owner, name: $repository) {\n    pullRequests(states: [MERGED], first: 3, baseRefName: $branch, after: $cursor) {\n      edges {\n        cursor\n        node {\n          body\n          number\n          baseRefName\n          headRefName\n          mergeCommit {\n            oid\n          }\n        }\n      }\n    }\n  }\n}\n\nquery BranchCommit($owner: String!, $repository: String!, $branch: String!) {\n  repository(owner: $owner, name: $repository) {\n    ref(qualifiedName: $branch) {\n      name\n      target {\n        __typename\n        ... on Commit {\n          oid\n        }\n      }\n    }\n  }\n}\n\nquery CommitHistory($owner: String!, $repository: String!, $oid: GitObjectID!, $cursor: String) {\n  repository(owner: $owner, name: $repository) {\n    object(oid: $oid) {\n      __typename\n      ... on Commit {\n        oid\n        history(first: 5, after: $cursor) {\n          totalCount\n          edges {\n            node {\n              oid\n            }\n          }\n        }\n      }\n    }\n  }\n}\n\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type GitObjectID = crate::github_collect::GitObjectID;
    #[derive(Serialize)]
    pub struct Variables {
        pub owner: String,
        pub repository: String,
        pub oid: GitObjectID,
        pub cursor: Option<String>,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub repository: Option<CommitHistoryRepository>,
    }
    #[derive(Deserialize, Debug)]
    pub struct CommitHistoryRepository {
        pub object: Option<CommitHistoryRepositoryObject>,
    }
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum CommitHistoryRepositoryObject {
        Blob,
        Commit(CommitHistoryRepositoryObjectOnCommit),
        Tag,
        Tree,
    }
    #[derive(Deserialize, Debug)]
    pub struct CommitHistoryRepositoryObjectOnCommit {
        pub oid: GitObjectID,
        pub history: CommitHistoryRepositoryObjectOnCommitHistory,
    }
    #[derive(Deserialize, Debug)]
    pub struct CommitHistoryRepositoryObjectOnCommitHistory {
        #[serde(rename = "totalCount")]
        pub total_count: Int,
        pub edges: Option<Vec<Option<CommitHistoryRepositoryObjectOnCommitHistoryEdges>>>,
    }
    #[derive(Deserialize, Debug)]
    pub struct CommitHistoryRepositoryObjectOnCommitHistoryEdges {
        pub node: Option<CommitHistoryRepositoryObjectOnCommitHistoryEdgesNode>,
    }
    #[derive(Deserialize, Debug)]
    pub struct CommitHistoryRepositoryObjectOnCommitHistoryEdgesNode {
        pub oid: GitObjectID,
    }
}
impl graphql_client::GraphQLQuery for CommitHistory {
    type Variables = commit_history::Variables;
    type ResponseData = commit_history::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: commit_history::QUERY,
            operation_name: commit_history::OPERATION_NAME,
        }
    }
}

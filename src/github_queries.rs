pub struct PullRequests;
pub mod pull_requests {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "PullRequests";
    pub const QUERY : & str = "query PullRequests($owner: String!, $repository: String!, $branch: String!, $cursor: String) {\n  repository(owner: $owner, name: $repository) {\n    pullRequests(states: [MERGED], first: 3, baseRefName: $branch, after: $cursor) {\n      edges {\n        cursor\n        node {\n          body\n          number\n          baseRefName\n          headRefName\n          mergeCommit {\n            oid\n          }\n        }\n      }\n    }\n  }\n}\n\nquery BranchHistory($owner: String!, $repository: String!, $branch: String!) {\n  repository(owner: $owner, name: $repository) {\n    ref(qualifiedName: $branch) {\n      name\n      target {\n        __typename\n        ... on Commit {\n          oid\n          history(first: 5) {\n            edges {\n              node {\n                oid\n                associatedPullRequests(first: 1) {\n                  edges {\n                    node {\n                      body\n                    }\n                  }\n                }\n              }\n            }\n          }\n        }\n      }\n    }\n  }\n}" ;
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
pub struct BranchHistory;
pub mod branch_history {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "BranchHistory";
    pub const QUERY : & str = "query PullRequests($owner: String!, $repository: String!, $branch: String!, $cursor: String) {\n  repository(owner: $owner, name: $repository) {\n    pullRequests(states: [MERGED], first: 3, baseRefName: $branch, after: $cursor) {\n      edges {\n        cursor\n        node {\n          body\n          number\n          baseRefName\n          headRefName\n          mergeCommit {\n            oid\n          }\n        }\n      }\n    }\n  }\n}\n\nquery BranchHistory($owner: String!, $repository: String!, $branch: String!) {\n  repository(owner: $owner, name: $repository) {\n    ref(qualifiedName: $branch) {\n      name\n      target {\n        __typename\n        ... on Commit {\n          oid\n          history(first: 5) {\n            edges {\n              node {\n                oid\n                associatedPullRequests(first: 1) {\n                  edges {\n                    node {\n                      body\n                    }\n                  }\n                }\n              }\n            }\n          }\n        }\n      }\n    }\n  }\n}" ;
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
        pub repository: Option<BranchHistoryRepository>,
    }
    #[derive(Deserialize, Debug)]
    pub struct BranchHistoryRepository {
        #[serde(rename = "ref")]
        pub ref_: Option<BranchHistoryRepositoryRef>,
    }
    #[derive(Deserialize, Debug)]
    pub struct BranchHistoryRepositoryRef {
        pub name: String,
        pub target: Option<BranchHistoryRepositoryRefTarget>,
    }
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum BranchHistoryRepositoryRefTarget {
        Blob,
        Commit(BranchHistoryRepositoryRefTargetOnCommit),
        Tag,
        Tree,
    }
    #[derive(Deserialize, Debug)]
    pub struct BranchHistoryRepositoryRefTargetOnCommit {
        pub oid: GitObjectID,
        pub history: BranchHistoryRepositoryRefTargetOnCommitHistory,
    }
    #[derive(Deserialize, Debug)]
    pub struct BranchHistoryRepositoryRefTargetOnCommitHistory {
        pub edges: Option<Vec<Option<BranchHistoryRepositoryRefTargetOnCommitHistoryEdges>>>,
    }
    #[derive(Deserialize, Debug)]
    pub struct BranchHistoryRepositoryRefTargetOnCommitHistoryEdges {
        pub node: Option<BranchHistoryRepositoryRefTargetOnCommitHistoryEdgesNode>,
    }
    #[derive(Deserialize, Debug)]
    pub struct BranchHistoryRepositoryRefTargetOnCommitHistoryEdgesNode {
        pub oid: GitObjectID,
        #[serde(rename = "associatedPullRequests")]
        pub associated_pull_requests:
            Option<BranchHistoryRepositoryRefTargetOnCommitHistoryEdgesNodeAssociatedPullRequests>,
    }
    #[derive(Deserialize, Debug)]
    pub struct BranchHistoryRepositoryRefTargetOnCommitHistoryEdgesNodeAssociatedPullRequests { pub edges : Option < Vec < Option < BranchHistoryRepositoryRefTargetOnCommitHistoryEdgesNodeAssociatedPullRequestsEdges >> > , }
    #[derive(Deserialize, Debug)]
    pub struct BranchHistoryRepositoryRefTargetOnCommitHistoryEdgesNodeAssociatedPullRequestsEdges {
        pub node: Option<
            BranchHistoryRepositoryRefTargetOnCommitHistoryEdgesNodeAssociatedPullRequestsEdgesNode,
        >,
    }
    #[derive(Deserialize, Debug)]
    pub struct BranchHistoryRepositoryRefTargetOnCommitHistoryEdgesNodeAssociatedPullRequestsEdgesNode {
        pub body: String,
    }
}
impl graphql_client::GraphQLQuery for BranchHistory {
    type Variables = branch_history::Variables;
    type ResponseData = branch_history::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: branch_history::QUERY,
            operation_name: branch_history::OPERATION_NAME,
        }
    }
}

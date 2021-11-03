pub struct BranchCommit;
pub mod branch_commit {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "BranchCommit";
    pub const QUERY : & str = "#query PullRequests($owner: String!, $repository: String!, $branch: String!, $cursor: String) {\n#  repository(owner: $owner, name: $repository) {\n#    pullRequests(states: [MERGED], first: 100, baseRefName: $branch, after: $cursor,orderBy: { field: UPDATED_AT, direction: DESC }) {\n#      edges {\n#        cursor\n#        node {\n#          body\n#          number\n#          baseRefName\n#          headRefName\n#          title,\n#          mergedAt,\n#          updatedAt,\n#          mergeCommit {\n#            oid\n#          }\n#        }\n#      }\n#    }\n#  }\n#}\n\nquery BranchCommit($owner: String!, $repository: String!, $branch: String!) {\n  repository(owner: $owner, name: $repository) {\n    ref(qualifiedName: $branch) {\n      name\n      target {\n        __typename\n        ... on Commit {\n          oid\n        }\n      }\n    }\n  }\n}\n\nquery CommitHistory($owner: String!, $repository: String!, $oid: GitObjectID!, $cursor: String) {\n  ...GetRateLimit\n  repository(owner: $owner, name: $repository) {\n    object(oid: $oid) {\n      __typename\n      ... on Commit {\n        oid\n        history(first: 100, after: $cursor) {\n          totalCount\n          edges {\n            cursor\n            node {\n              oid\n              committedDate\n              ...GetPullRequests\n            }\n          }\n        }\n      }\n    }\n  }\n}\n\nfragment GetRateLimit on Query {\n  rateLimit {\n    limit\n    cost\n    remaining\n    resetAt\n  }\n}\n\nfragment GetPullRequests on Commit {\n  associatedPullRequests(first: 100) {\n    edges {\n      node {\n        number\n        title\n        body\n        mergedAt\n        mergeCommit {\n          oid\n        }\n      }\n    }\n  }\n}" ;
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
    pub const QUERY : & str = "#query PullRequests($owner: String!, $repository: String!, $branch: String!, $cursor: String) {\n#  repository(owner: $owner, name: $repository) {\n#    pullRequests(states: [MERGED], first: 100, baseRefName: $branch, after: $cursor,orderBy: { field: UPDATED_AT, direction: DESC }) {\n#      edges {\n#        cursor\n#        node {\n#          body\n#          number\n#          baseRefName\n#          headRefName\n#          title,\n#          mergedAt,\n#          updatedAt,\n#          mergeCommit {\n#            oid\n#          }\n#        }\n#      }\n#    }\n#  }\n#}\n\nquery BranchCommit($owner: String!, $repository: String!, $branch: String!) {\n  repository(owner: $owner, name: $repository) {\n    ref(qualifiedName: $branch) {\n      name\n      target {\n        __typename\n        ... on Commit {\n          oid\n        }\n      }\n    }\n  }\n}\n\nquery CommitHistory($owner: String!, $repository: String!, $oid: GitObjectID!, $cursor: String) {\n  ...GetRateLimit\n  repository(owner: $owner, name: $repository) {\n    object(oid: $oid) {\n      __typename\n      ... on Commit {\n        oid\n        history(first: 100, after: $cursor) {\n          totalCount\n          edges {\n            cursor\n            node {\n              oid\n              committedDate\n              ...GetPullRequests\n            }\n          }\n        }\n      }\n    }\n  }\n}\n\nfragment GetRateLimit on Query {\n  rateLimit {\n    limit\n    cost\n    remaining\n    resetAt\n  }\n}\n\nfragment GetPullRequests on Commit {\n  associatedPullRequests(first: 100) {\n    edges {\n      node {\n        number\n        title\n        body\n        mergedAt\n        mergeCommit {\n          oid\n        }\n      }\n    }\n  }\n}" ;
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
    type DateTime = crate::github_collect::DateTime;
    #[derive(Serialize)]
    pub struct Variables {
        pub owner: String,
        pub repository: String,
        pub oid: GitObjectID,
        pub cursor: Option<String>,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct GetRateLimit {
        #[serde(rename = "rateLimit")]
        pub rate_limit: Option<GetRateLimitRateLimit>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetRateLimitRateLimit {
        pub limit: Int,
        pub cost: Int,
        pub remaining: Int,
        #[serde(rename = "resetAt")]
        pub reset_at: DateTime,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPullRequests {
        #[serde(rename = "associatedPullRequests")]
        pub associated_pull_requests: Option<GetPullRequestsAssociatedPullRequests>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPullRequestsAssociatedPullRequests {
        pub edges: Option<Vec<Option<GetPullRequestsAssociatedPullRequestsEdges>>>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPullRequestsAssociatedPullRequestsEdges {
        pub node: Option<GetPullRequestsAssociatedPullRequestsEdgesNode>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPullRequestsAssociatedPullRequestsEdgesNode {
        pub number: Int,
        pub title: String,
        pub body: String,
        #[serde(rename = "mergedAt")]
        pub merged_at: Option<DateTime>,
        #[serde(rename = "mergeCommit")]
        pub merge_commit: Option<GetPullRequestsAssociatedPullRequestsEdgesNodeMergeCommit>,
    }
    #[derive(Deserialize, Debug)]
    pub struct GetPullRequestsAssociatedPullRequestsEdgesNodeMergeCommit {
        pub oid: GitObjectID,
    }
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(flatten)]
        pub get_rate_limit: GetRateLimit,
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
        pub cursor: String,
        pub node: Option<CommitHistoryRepositoryObjectOnCommitHistoryEdgesNode>,
    }
    #[derive(Deserialize, Debug)]
    pub struct CommitHistoryRepositoryObjectOnCommitHistoryEdgesNode {
        pub oid: GitObjectID,
        #[serde(rename = "committedDate")]
        pub committed_date: DateTime,
        #[serde(flatten)]
        pub get_pull_requests: GetPullRequests,
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

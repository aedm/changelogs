pub struct TestQuery;
pub mod test_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "TestQuery";
    pub const QUERY : & str = "query TestQuery {\n  repository(owner:\"aedm\", name:\"changelogs-test\") {\n    pullRequests(states: [MERGED], first: 10, baseRefName: \"main\") {\n      edges {\n        cursor\n        node {\n          body\n          number\n          baseRefName\n          headRefName\n          mergeCommit {\n            oid\n          }\n        }\n      }\n    }\n  }\n}\n\nquery PullRequests($owner: String!, $repository: String!, $branch: String!, $cursor: String) {\n  repository(owner: $owner, name: $repository) {\n    pullRequests(states: [MERGED], first: 3, baseRefName: $branch, after: $cursor) {\n      edges {\n        cursor\n        node {\n          body\n          number\n          baseRefName\n          headRefName\n          mergeCommit {\n            oid\n          }\n        }\n      }\n    }\n  }\n}" ;
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
    pub struct Variables;
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub repository: Option<TestQueryRepository>,
    }
    #[derive(Deserialize, Debug)]
    pub struct TestQueryRepository {
        #[serde(rename = "pullRequests")]
        pub pull_requests: TestQueryRepositoryPullRequests,
    }
    #[derive(Deserialize, Debug)]
    pub struct TestQueryRepositoryPullRequests {
        pub edges: Option<Vec<Option<TestQueryRepositoryPullRequestsEdges>>>,
    }
    #[derive(Deserialize, Debug)]
    pub struct TestQueryRepositoryPullRequestsEdges {
        pub cursor: String,
        pub node: Option<TestQueryRepositoryPullRequestsEdgesNode>,
    }
    #[derive(Deserialize, Debug)]
    pub struct TestQueryRepositoryPullRequestsEdgesNode {
        pub body: String,
        pub number: Int,
        #[serde(rename = "baseRefName")]
        pub base_ref_name: String,
        #[serde(rename = "headRefName")]
        pub head_ref_name: String,
        #[serde(rename = "mergeCommit")]
        pub merge_commit: Option<TestQueryRepositoryPullRequestsEdgesNodeMergeCommit>,
    }
    #[derive(Deserialize, Debug)]
    pub struct TestQueryRepositoryPullRequestsEdgesNodeMergeCommit {
        pub oid: GitObjectID,
    }
}
impl graphql_client::GraphQLQuery for TestQuery {
    type Variables = test_query::Variables;
    type ResponseData = test_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: test_query::QUERY,
            operation_name: test_query::OPERATION_NAME,
        }
    }
}
pub struct PullRequests;
pub mod pull_requests {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "PullRequests";
    pub const QUERY : & str = "query TestQuery {\n  repository(owner:\"aedm\", name:\"changelogs-test\") {\n    pullRequests(states: [MERGED], first: 10, baseRefName: \"main\") {\n      edges {\n        cursor\n        node {\n          body\n          number\n          baseRefName\n          headRefName\n          mergeCommit {\n            oid\n          }\n        }\n      }\n    }\n  }\n}\n\nquery PullRequests($owner: String!, $repository: String!, $branch: String!, $cursor: String) {\n  repository(owner: $owner, name: $repository) {\n    pullRequests(states: [MERGED], first: 3, baseRefName: $branch, after: $cursor) {\n      edges {\n        cursor\n        node {\n          body\n          number\n          baseRefName\n          headRefName\n          mergeCommit {\n            oid\n          }\n        }\n      }\n    }\n  }\n}" ;
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

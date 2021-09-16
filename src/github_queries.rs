pub struct TestQuery;
pub mod test_query {
    #![allow(dead_code,unused_imports)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "TestQuery";
    pub const QUERY : & str = "query TestQuery {\n  repository(owner:\"aedm\", name:\"changelogs-test\") {\n    pullRequests(states: [MERGED], first: 10, baseRefName: \"main\") {\n      edges {\n        cursor\n        node {\n          body\n          number\n          baseRefName\n          headRefName\n          mergeCommit {\n            oid\n          }\n        }\n      }\n    }\n  }\n}" ;
    #[allow(unused_imports)]
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

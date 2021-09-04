use ::reqwest::blocking::Client;
use anyhow::*;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use log::*;
use prettytable::*;
use structopt::StructOpt;

#[allow(clippy::upper_case_acronyms)]
type URI = String;

type GitObjectID = String;

#[derive(GraphQLQuery)]
#[graphql(
schema_path = "src/graphql/github-schema.docs.graphql",
query_path = "src/graphql/queries.graphql",
response_derives = "Debug"
)]
struct TestQuery;

#[derive(StructOpt)]
#[structopt(author, about)]
struct Command {
    #[structopt(name = "repository")]
    repo: String,
}

fn parse_repo_name(repo_name: &str) -> Result<(&str, &str), anyhow::Error> {
    let mut parts = repo_name.split('/');
    match (parts.next(), parts.next()) {
        (Some(owner), Some(name)) => Ok((owner, name)),
        _ => Err(format_err!("wrong format for the repository name param (we expect something like facebook/graphql)"))
    }
}

pub fn run_query() -> Result<(), anyhow::Error> {
    env_logger::init();

    let github_api_token =
        std::env::var("GITHUB_API_TOKEN").expect("Missing GITHUB_API_TOKEN env var");

    // let args = Command::from_args();
    //
    // let repo = args.repo;
    // let (owner, name) = parse_repo_name(&repo).unwrap_or(("tomhoule", "graphql-client"));
    //
    let variables = test_query::Variables {
        // owner: owner.to_string(),
        // name: name.to_string(),
    };

    let client = Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", github_api_token))
                    .unwrap(),
            ))
                .collect(),
        )
        .build()?;

    let response_body =
        post_graphql::<TestQuery, _>(&client, "https://api.github.com/graphql", variables).unwrap();

    info!("{:?}", response_body);

    let response_data: test_query::ResponseData = response_body.data.expect("missing response data");

    println!("{:?}", response_data);
    // let stars: Option<i64> = response_data
    //     .repository
    //     .as_ref()
    //     .map(|repo| repo.stargazers.total_count);

    // println!("{}/{} - 🌟 {}", owner, name, stars.unwrap_or(0),);
    // println!("{}", owner, name, stars.unwrap_or(0),);

    // let mut table = prettytable::Table::new();
    // table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    // table.set_titles(row!(b => "issue", "comments"));
    //
    // for issue in response_data
    //     .repository
    //     .expect("missing repository")
    //     .issues
    //     .nodes
    //     .expect("issue nodes is null")
    //     .iter()
    //     .flatten()
    // {
    //     table.add_row(row!(issue.title, issue.comments.total_count));
    // }
    //
    // table.printstd();
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn parse_repo_name_works() {
//         assert_eq!(
//             parse_repo_name("graphql-rust/graphql-client").unwrap(),
//             ("graphql-rust", "graphql-client")
//         );
//         assert!(parse_repo_name("abcd").is_err());
//     }
// }
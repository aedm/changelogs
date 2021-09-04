use clap::{Clap};
use std::error::Error;
use crate::github_query::run_query;

/// Collects changelog entries from a git branch
#[derive(Clap)]
pub struct CollectCommand {
    /// Git branch where pull requests are collected
    #[clap(short, long)]
    since_branch: String,
}

impl CollectCommand {
    pub fn run(&self) {
        let result = self.get_stuff();
        if let Err(err) = result {
            println!("Error: {:?}", err);
        }
    }

    pub fn get_stuff(&self) -> Result<(), Box<dyn Error>> {
        println!("since {}", self.since_branch);
        let _ = run_query();

        // let octocrab = octocrab::instance();
        // // let mut current_page = octocrab.issues("octocrab", "repo")
        // //     .list()
        // //     // Optional Parameters
        // //     .creator("octocrab")
        // //     .state(params::State::All)
        // //     .per_page(50)
        // //     .send()
        // //     .await?;
        // let mut current_page = octocrab
        //     .orgs("rust-lang")
        //     .list_repos()
        //     .repo_type(params::repos::Type::Sources)
        //     .per_page(100)
        //     .send()
        //     .await?;
        // // let issue = octocrab::instance().pulls("octocrab", "repo").get(404).await?;
        // let mut prs = current_page.take_items();
        //
        // while let Some(mut next_page) = octocrab.get_page(&current_page.next).await? {
        //     prs.extend(next_page.take_items());
        //
        //     for pr in prs.drain(..) {
        //         println!("{:#?}", pr);
        //     }
        //
        //     // page.take_items()
        //     // for issue in page.into_iter() {
        //     //     println!("{}", issue.title);
        //     // }
        //     current_page = next_page;
        // }

        Ok(())
    }
}
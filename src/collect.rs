use crate::github_collect::run_query;
use clap::Clap;
use std::any::Any;
use std::error::Error;

/// Collects changelog entries from a git branch
#[derive(Clap)]
pub struct CollectCommand {
    /// Git branch where pull requests are collected
    #[clap(short, long)]
    since_branch: String,
}

impl CollectCommand {
    pub async fn run(&self) {
        let result = self.get_stuff().await;
        if let Err(err) = result {
            println!("Error: {:?}", err);
        }
    }

    pub async fn get_stuff(&self) -> Result<(), Box<dyn Error>> {
        println!("since {}", self.since_branch);
        let _ = run_query().await;

        Ok(())
    }
}

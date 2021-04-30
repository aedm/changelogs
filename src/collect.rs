use clap::{Clap};

/// Collects changelog entries from a git branch
#[derive(Clap)]
pub struct CollectCommand {
    /// Git branch where pull requests are collected
    #[clap(short, long)]
    since_branch: String,
}

impl CollectCommand {
    pub fn run(&self) {
        println!("since {}", self.since_branch);
    }
}
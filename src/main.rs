use std::path::PathBuf;

use clap::Parser;
use gitless::{GitRepo, GitlessError, GitlessResult};

#[derive(Debug, Parser)]
pub struct Args {
    /// Git URL you would like to clone and degit from
    #[arg(long, short)]
    url: String,

    /// Destination of cloned and degitted path
    #[arg(long = "dest")]
    dest_path: PathBuf,

    /// Git branch or tag to clone and degit from
    #[arg(long, short)]
    branch: Option<String>,

    /// Git commit hash to clone and degit from
    #[arg(long, short)]
    rev: Option<String>,
}

fn main() -> GitlessResult<()> {
    let Args {
        url,
        dest_path,
        branch,
        rev,
    } = Args::parse();

    match (branch, rev) {
        (None, None) => {
            GitRepo::clone_default(&url, dest_path.clone())?.degit()?;
        }
        (Some(_), Some(_)) => {
            return Err(GitlessError::Message(
                "both branch and url args are provided, you can only pass one of each",
            ));
        }
        (None, Some(hash)) => {
            GitRepo::clone_from_rev(&url, &hash, dest_path.clone())?.degit()?;
        }
        (Some(branch), None) => {
            GitRepo::clone_from_branch_or_tag(&url, &branch, dest_path.clone())?.degit()?;
        }
    }
    println!("gitless repo cloned successfuly at: {:?}", dest_path);

    Ok(())
}

mod error;

use anyhow::Context;
use error::GitlessResult;
use git2::{build::RepoBuilder, Oid, Repository};
use std::{path::{Path, PathBuf}, fs};

#[derive(Debug, Clone)]
pub struct Gitless {
    url: &'static str,
}

impl Gitless {
    pub fn new(url: &'static str) -> Self {
        Gitless { url }
    }

    pub fn clone<P: AsRef<PathBuf>>(&self, into: P) -> GitlessResult<Degit> {
        let repo = Repository::clone(self.url, into.as_ref())?;
        Ok(repo.into())
    }

    pub fn clone_from_branch_or_tag<P: AsRef<Path>>(
        &self,
        name: &'static str,
        into: P,
    ) -> GitlessResult<Degit> {
        let repo = RepoBuilder::new()
            .branch(name)
            .clone(self.url, into.as_ref())?;
        Ok(repo.into())
    }

    pub fn clone_from_rev<P: AsRef<Path>>(
        &self,
        hash: &'static str,
        into: P,
    ) -> GitlessResult<Degit> {
        let mut repo_builder = RepoBuilder::new();
        let repo = repo_builder.clone(self.url, into.as_ref())?;
        let commit_oid = Oid::from_str(hash).context("failed to parse str")?;
        let commit_object = repo.find_object(commit_oid, None)?;
        repo.reset(&commit_object, git2::ResetType::Hard, None)?;
				
				let repo_path = repo.path().to_path_buf();
				let degit = Degit {into_path: repo_path};
				Ok(degit)
    }
}

#[derive(Debug)]
pub struct Degit {
	into_path: PathBuf
}

impl Degit {
	pub fn degit(&self) -> GitlessResult<()> {
		let git_path = self.into_path.join(".git");
		fs::remove_dir_all(git_path).context("failed to degit")?;
		Ok(())
	}
}

impl From<Repository> for Degit {
	fn from(r: Repository) -> Self {
			Degit { into_path: r.path().to_path_buf() }
	}
}
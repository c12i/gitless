mod error;

use anyhow::Context;
use git2::{build::RepoBuilder, Oid, Repository};
use std::{fs, path::PathBuf};

pub use error::{GitlessError, GitlessResult};

#[derive(Debug, Clone)]
pub struct GitRepo;

impl GitRepo {
    pub fn clone_default(url: &str, dest: PathBuf) -> GitlessResult<Degit> {
        let repo = Repository::clone(url, dest.as_path())?;
        Ok(repo.into())
    }

    pub fn clone_from_branch_or_tag(url: &str, name: &str, dest: PathBuf) -> GitlessResult<Degit> {
        let repo = RepoBuilder::new().branch(name).clone(url, dest.as_path())?;
        Ok(repo.into())
    }

    pub fn clone_from_rev(url: &str, hash: &str, dest: PathBuf) -> GitlessResult<Degit> {
        let mut repo_builder = RepoBuilder::new();
        let repo = repo_builder.clone(url, dest.as_path())?;
        let commit_oid = Oid::from_str(hash).context("failed to parse str")?;
        let commit_object = repo.find_object(commit_oid, None)?;
        repo.reset(&commit_object, git2::ResetType::Hard, None)?;

        let repo_path = repo.path().to_path_buf();
        let degit = Degit {
            into_path: repo_path,
        };
        Ok(degit)
    }
}

#[derive(Debug)]
pub struct Degit {
    into_path: PathBuf,
}

impl Degit {
    pub fn degit(&self) -> GitlessResult<()> {
        fs::remove_dir_all(self.into_path.clone()).context("failed to degit")?;
        Ok(())
    }
}

impl From<Repository> for Degit {
    fn from(r: Repository) -> Self {
        Degit {
            into_path: r.path().to_path_buf(),
        }
    }
}

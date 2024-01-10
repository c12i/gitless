#[derive(thiserror::Error, Debug)]
pub enum GitlessError {
    #[error("git2 error: {0:?}")]
    Git2Error(#[from] git2::Error),
    #[error("unexpected error: {0:?}")]
    Other(#[from] anyhow::Error),
}

pub type GitlessResult<T> = Result<T, GitlessError>;

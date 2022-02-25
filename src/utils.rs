use thiserror::Error;
use url::Url;

#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("the URL provided appears to be malformed")]
    ParseError(#[from] Option<url::ParseError>),
    #[error("the URL provided has no separator to indicate an owner and repository")]
    MissingSeparator,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct RepositoryNamespace {
    pub owner: String,
    pub repository: String,
}

pub fn namespace<S>(url: S) -> Result<RepositoryNamespace>
where
    S: AsRef<str>,
{
    let url = Url::parse(url.as_ref())?;
    let path = url.path();
    let (owner, repository) = path
        .strip_prefix('/')
        .unwrap_or(path)
        .split_once('/')
        .ok_or(Error::MissingSeparator)?;

    Ok(RepositoryNamespace {
        owner: owner.to_owned(),
        repository: repository.to_owned(),
    })
}

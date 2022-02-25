use crate::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("there was an issue making the request")]
    Ureq(#[from] ureq::Error),
    #[error("there was an issue deserializing a response to JSON")]
    Io(#[from] std::io::Error),
    #[error("there was an issue fitting JSON to a strong type")]
    Json(#[from] serde_path_to_error::Error<serde_json::Error>),
}

pub type Result<T> = std::result::Result<T, Error>;

fn get<T>(endpoint: &str, token: Option<&str>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let request = ureq::get(endpoint);

    if let Some(token) = token {
        request.set("Authorization", &format!("token {}", token));
    }

    let response = ureq::get(endpoint).call()?;
    let string = response.into_string()?;
    let deserializer = &mut serde_json::Deserializer::from_str(&string);

    Ok(serde_path_to_error::deserialize(deserializer)?)
}

pub fn get_repository<A, B, C>(owner: A, repository: B, token: Option<C>) -> Result<Repository>
where
    A: AsRef<str>,
    B: AsRef<str>,
    C: AsRef<str>,
{
    get(
        &format!(
            "https://api.github.com/repos/{}/{}",
            owner.as_ref(),
            repository.as_ref()
        ),
        token.as_ref().map(|x| x.as_ref()),
    )
}

pub fn get_repository_releases<A, B, C>(
    owner: A,
    repository: B,
    token: Option<C>,
) -> Result<Vec<Release>>
where
    A: AsRef<str>,
    B: AsRef<str>,
    C: AsRef<str>,
{
    get(
        &format!(
            "https://api.github.com/repos/{}/{}/releases",
            owner.as_ref(),
            repository.as_ref()
        ),
        token.as_ref().map(|x| x.as_ref()),
    )
}

use chrono::{DateTime, Local};
use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Getters, CopyGetters)]
#[getset(get, get)]
pub struct Release {
    url: String,
    assets_url: String,
    upload_url: String,
    html_url: String,
    id: usize,
    author: ReleaseAuthor,
    node_id: String,
    tag_name: String,
    target_commitish: String,
    name: String,
    draft: bool,
    prerelease: bool,
    created_at: DateTime<Local>,
    published_at: DateTime<Local>,
    assets: Vec<ReleaseAsset>,
    tarball_url: String,
    zipball_url: String,
    body: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, CopyGetters)]
#[getset(get, get)]
pub struct ReleaseAuthor {
    login: String,
    id: usize,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    user_type: String,
    site_admin: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, CopyGetters)]
#[getset(get, get)]
pub struct ReleaseAsset {
    url: String,
    id: usize,
    node_id: String,
    name: String,
    label: String,
    uploader: ReleaseAuthor,
    content_type: String,
    state: String,
    size: usize,
    download_count: usize,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    browser_download_url: String,
}


use chrono::{DateTime, Local};
use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};

use crate::User;

#[derive(Clone, Debug, Serialize, Deserialize, Getters, CopyGetters)]
#[getset(get, get)]
pub struct Release {
    url: String,
    assets_url: String,
    upload_url: String,
    html_url: String,
    id: usize,
    author: User,
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
pub struct ReleaseAsset {
    url: String,
    id: usize,
    node_id: String,
    name: String,
    label: String,
    uploader: User,
    content_type: String,
    state: String,
    size: usize,
    download_count: usize,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    browser_download_url: String,
}

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::User;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Release {
    pub url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub html_url: String,
    pub id: usize,
    pub author: User,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: DateTime<Local>,
    pub published_at: DateTime<Local>,
    pub assets: Vec<ReleaseAsset>,
    pub tarball_url: String,
    pub zipball_url: String,
    pub body: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseAsset {
    pub url: String,
    pub id: usize,
    pub node_id: String,
    pub name: String,
    pub label: Option<String>,
    pub uploader: User,
    pub content_type: String,
    pub state: String,
    pub size: usize,
    pub download_count: usize,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub browser_download_url: String,
}

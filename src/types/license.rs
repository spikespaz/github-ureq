use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Getters, CopyGetters)]
#[getset(get, get)]
pub struct License {
    key: String,
    name: String,
    spdx_id: String,
    url: Option<String>,
    node_id: String,
}

use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Getters, CopyGetters)]
#[getset(get, get)]
pub struct License {
    key: String,
    name: String,
    spx_id: String,
    url: String,
    node_id: String,
}

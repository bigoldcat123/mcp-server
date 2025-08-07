use serde::{Deserialize, Serialize};

use unknown::Unknown;

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
struct InitializedNotification {
    jsonrpc:String,
    method:String,
    params:Option<Unknown>
}


pub mod init;
pub mod tool;
use crate::{util::Unknown, Package};

use serde::{Deserialize, Serialize};


Package!(
    pub struct CommonRequest {
        pub id:Option<i32>,
        params:Option<Unknown>
    }
);

impl TryFrom<Unknown> for CommonRequest {
    type Error = String;
    fn try_from(value: Unknown) -> std::result::Result<Self, Self::Error> {
        let mut params = value.unwrap_as_map().ok_or("err")?;
        Ok(CommonRequest { jsonrpc: params.remove("jsonrpc").ok_or("err")?.unwrap_as_string().ok_or("err")?,
            id: if let Some(id) = params.remove("id") {
                Some(id.unwrap_as_number().ok_or("err")?)
            }else {
                None
            },
            method:  params.remove("method").ok_or("err")?.unwrap_as_string().ok_or("err")?,
            params: if let Some(x) = params.remove("params") {
                Some(x)
            } else {
                None
            }
        })
    }
}

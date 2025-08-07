use std::collections::HashMap;

use unknown::Unknown;
use crate::Request;
use crate::Package;
use serde::{Deserialize,Serialize};

use super::CommonRequest;

#[derive(Serialize,Deserialize,Debug)]
pub struct CallToolRequestParams {
    pub name:String,
    pub arguments:Option<HashMap<String,Unknown>>
}

Request!(
    pub struct CallToolRequest {
        pub params:CallToolRequestParams
    }
);
impl CallToolRequest {
    pub fn new(jsonrpc:String,method:String,id:i32,name:String,arguments:Option<HashMap<String,Unknown>>) -> Self {
        Self {
            jsonrpc,
            method,
            id,
            params:CallToolRequestParams {
                name,
                arguments
            }
        }
    }
}
impl TryFrom<CommonRequest> for CallToolRequest {
    type Error = String;
    fn try_from(value: CommonRequest) -> Result<Self, Self::Error> {
        let mut params = value.params.ok_or("err")?.unwrap_as_map().ok_or("err")?;
        let name = params.remove("name").ok_or("err")?.unwrap_as_string().ok_or("err")?;
        let arg;
        if let Some(a) = params.remove("arguments") {
            arg = a.unwrap_as_map();
        }else {
            arg = None;
        }

        Ok(Self::new(value.jsonrpc, value.method, value.id.ok_or("err")?, name, arg))
    }
}

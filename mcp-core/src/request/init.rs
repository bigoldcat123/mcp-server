use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use unknown::Unknown;

use crate::{ Implementation, Request};
use crate::Package;

use super::CommonRequest;


#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ClientCapabilitiesRoots {
    list_changed: Option<bool>,
}

impl TryFrom<Unknown> for ClientCapabilitiesRoots {
    type Error = String;
    fn try_from(value: Unknown) -> std::result::Result<Self, Self::Error> {
        let mut map = value.unwrap_as_map().ok_or("Invalid params")?;
        let list_changed = if let Some(v) = map.remove("listChanged") {
            Some(v.unwrap_as_bool().ok_or("err")?)
        } else {
            None
        };
        Ok(ClientCapabilitiesRoots {
            list_changed,
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ClientCapabilities {
    experimental: Option<HashMap<String, Unknown>>,
    roots: Option<ClientCapabilitiesRoots>,
    sampling: Option<HashMap<String, Unknown>>,
    elicitation: Option<HashMap<String, Unknown>>,
}
impl TryFrom<Unknown> for ClientCapabilities {
    type Error = String;
    fn try_from(value: Unknown) -> std::result::Result<Self, Self::Error> {
        let mut map = value.unwrap_as_map().ok_or("Invalid params")?;
        let experimental = if let Some(e) = map.remove("experimental") {
            Some(e.unwrap_as_map().ok_or("err")?)
        }else {
            None
        };
        let roots = if let Some(r) = map.remove("roots") {
            Some(r.try_into()?)
        }else {
            None
        };
        let sampling = if let Some(s) = map.remove("sampling") {
            Some(s.unwrap_as_map().ok_or("err")?)
        }else {
            None
        };
        let elicitation = if let Some(e) = map.remove("elicitation") {
            Some(e.unwrap_as_map().ok_or("err")?)
        }else {
            None
        };
        Ok(ClientCapabilities {
            experimental,
            roots,
            sampling,
            elicitation,
        })
    }
}



#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InitializeRequesParam {
    pub client_info: Implementation,
    pub capabilities: ClientCapabilities,
    pub protocol_version: String,
}
impl TryFrom<Unknown> for InitializeRequesParam {
    type Error = String;
    fn try_from(value: Unknown) -> std::result::Result<Self, Self::Error> {
        let mut map = value.unwrap_as_map().ok_or("Invalid params")?;
        let protocol_version = map.remove("protocolVersion").ok_or("err")?.unwrap_as_string().ok_or("err")?;
        let client_info = map.remove("clientInfo").ok_or("err")?.try_into().map_err(|e| "".to_string())?;
        let capabilities = map.remove("capabilities").ok_or("err")?.try_into()?;
        Ok(InitializeRequesParam {
            client_info,
            capabilities,
            protocol_version,
        })
    }
}

Request!(
    pub struct InitializeRequest {
        pub params: InitializeRequesParam
    }
);

impl TryFrom<CommonRequest> for InitializeRequest {
    type Error = String;
    fn try_from(value: CommonRequest) -> std::result::Result<Self, Self::Error> {
        Ok(InitializeRequest { jsonrpc: value.jsonrpc,
            id: value.id.ok_or("err")?,
            method: value.method,
            params: value.params.ok_or("err")?.try_into()?
        })
    }
}

// #![allow(unused)]

use crate::util::Object;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod macros;
pub mod util;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ClientCapabilitiesRoots {
    list_changed: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ClientCapabilities {
    experimental: Option<HashMap<String, Object>>,
    roots: Option<ClientCapabilitiesRoots>,
    sampling: Option<HashMap<String, Object>>,
    elicitation: Option<HashMap<String, Object>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct InitializeRequesParam {
    client_info: Implementation,
    capabilities: ClientCapabilities,
    protocol_version: String,
}
BaseMetadata!(
    pub struct Implementation {
        version: String
    }
);
impl Implementation {
    pub fn new(name: String, title: Option<String>, version: String) -> Self {
        Implementation { name, title, version }
    }
}

Request!(
    pub struct InitializeRequest {
        params: InitializeRequesParam
    }
);
#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prompts {
    list_changed: Option<bool>
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resources {
    list_changed: Option<bool>,
    subscribe: Option<bool>
}
#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tools {
    list_changed: Option<bool>
}
#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    experimental:Option<HashMap<String, Object>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<Object>,
    #[serde(skip_serializing_if = "Option::is_none")]
    completions: Option<Object>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompts:Option<Prompts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Tools>
}
impl ServerCapabilities {
    pub fn new(experimental:Option<HashMap<String, Object>>, logging: Option<Object>, completions: Option<Object>, prompts:Option<Prompts>, tools: Option<Tools>) -> Self {
        ServerCapabilities {
            experimental,
            logging,
            completions,
            prompts,
            tools
        }
    }
}
#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
struct InnerInitializeResult {
    protocol_version:String,
    capabilities: ServerCapabilities,
    server_info: Implementation,
    instructions:Option<String>
}
Package!(
    pub struct InitializeResult {
        result: InnerInitializeResult
    }
);
impl InitializeResult {
    pub fn new(json:String,id:i32,protocol_version: String, capabilities: ServerCapabilities, server_info: Implementation, instructions: Option<String>) -> Self {

        Self{
            jsonrpc:json,
            id,
            result: InnerInitializeResult {

            protocol_version,
            capabilities,
            server_info,
            instructions,
        }}
    }
}

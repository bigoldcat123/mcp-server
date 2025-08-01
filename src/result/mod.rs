pub mod tools;
pub mod prompt;
pub mod resoures;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{util::Unknown, Implementation, Package, Result};



#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prompts {
    list_changed: Option<bool>
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resources {
    #[serde(skip_serializing_if = "Option::is_none")]
    list_changed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscribe: Option<bool>
}
#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tools {
    #[serde(skip_serializing_if = "Option::is_none")]
    list_changed: Option<bool>
}
impl Tools {
    pub fn new(list_changed: Option<bool>) -> Self {
        Tools { list_changed }
    }
}
#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {

    #[serde(skip_serializing_if = "Option::is_none")]
    experimental:Option<HashMap<String, Unknown>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<Unknown>,
    #[serde(skip_serializing_if = "Option::is_none")]
    completions: Option<Unknown>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompts:Option<Prompts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Tools>
}
impl ServerCapabilities {
    pub fn new(experimental:Option<HashMap<String, Unknown>>, logging: Option<Unknown>, completions: Option<Unknown>, prompts:Option<Prompts>, tools: Option<Tools>) -> Self {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    instructions:Option<String>
}
Result!(
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

#[derive(Debug,Serialize,Deserialize)]
pub struct ResultError {
    code:i32,
    message:String,
    data:Option<HashMap<String, Unknown>>
}

Result!(
    pub struct CommonResult {
        pub result:Option<HashMap<String, Unknown>>,
        pub error:Option<ResultError>
    }
);

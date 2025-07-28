// #![allow(unused)]

use crate::util::Object;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::{Lines, Result, Stdin, StdinLock, Write}};
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
    // #[serde(skip_serializing_if = "Option::is_none")]
    experimental:Option<HashMap<String, Object>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<Object>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    completions: Option<Object>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    prompts:Option<Prompts>,
    // #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
struct InitializedNotification {
    jsonrpc:String,
    method:String,
    params:Option<Object>
}

pub struct  McpServer {
    lines:Lines<StdinLock<'static>>,
    log_file:File
}
impl McpServer {
    pub fn new(stdin:Stdin, log_file:File) -> Self {
        Self {
            lines:stdin.lines(),
            log_file
        }
    }
    pub fn run(mut self) -> Result<()>{
        self.handle_initialization()?;
        Ok(())
    }
    fn handle_initialization(&mut self) ->Result<()> {
        if let Some(next) = self.lines.next() {
            let line = next?;
            self.log(&line);
            let _req = serde_json::from_str::<InitializeRequest>(&line)?;
            let res = InitializeResult::new(_req.jsonrpc, _req.id, _req.params.protocol_version, ServerCapabilities::new(None, None, None, None, Some(Tools { list_changed: Some(false) })), Implementation::new("ExampleServer".to_string(), Some("Example Server Display Name".to_string()), "2.0".to_string()), Some("this is a instruction!".to_string()));
            let res = serde_json::to_string(&res)?;
            self.log(format!("|---{}", res).as_str());
            println!("{res}");
        }
        if let Some(next) = self.lines.next() {
            let init_notification = next?;
            self.log(&init_notification);

            let _notification = serde_json::from_str::<InitializedNotification>(&init_notification)?;
        }
        if let Some(next) = self.lines.next() {
            let init_notification = next?;
            self.log(&init_notification);
        }
        Ok(())
    }
    fn log(&mut self, msg:&str) {
        self.log_file.write_fmt(format_args!("{}: {} \n", file!(), msg)).expect("writing to log file");
    }
}

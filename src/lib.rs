#![allow(unused)]

use crate::util::Unknown;
use constant::RequestMethod;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs::File, io::{Lines, Result, Stdin, StdinLock, Write}};
pub mod macros;
pub mod util;
pub mod constant;

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
struct ClientCapabilities {
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
struct InitializeRequesParam {
    client_info: Implementation,
    capabilities: ClientCapabilities,
    protocol_version: String,
}
impl TryFrom<Unknown> for InitializeRequesParam {
    type Error = String;
    fn try_from(value: Unknown) -> std::result::Result<Self, Self::Error> {
        let mut map = value.unwrap_as_map().ok_or("Invalid params")?;
        let protocol_version = map.remove("protocolVersion").ok_or("err")?.unwrap_as_string().ok_or("err")?;
        let client_info = map.remove("clientInfo").ok_or("err")?.try_into()?;
        let capabilities = map.remove("capabilities").ok_or("err")?.try_into()?;
        Ok(InitializeRequesParam {
            client_info,
            capabilities,
            protocol_version,
        })
    }
}
BaseMetadata!(
    pub struct Implementation {
        version: String
    }
);
impl TryFrom<Unknown> for Implementation {
    type Error = String;
    fn try_from(value: Unknown) -> std::result::Result<Self, Self::Error> {
        let mut imp = value.unwrap_as_map().ok_or("err")?;
        let version = imp.remove("version").ok_or("err")?.unwrap_as_string().ok_or("err")?;
        let name = imp.remove("name").ok_or("err")?.unwrap_as_string().ok_or("err")?;
        let title = if let Some(t) = imp.remove("title") {
            Some(t.unwrap_as_string().ok_or("err")?)
        }else {
            None
        };
        Ok(Implementation { version, name, title })

    }
}
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

impl TryFrom<CommonRequest> for InitializeRequest {
    type Error = String;
    fn try_from(value: CommonRequest) -> std::result::Result<Self, Self::Error> {
        Ok(InitializeRequest { jsonrpc: value.jsonrpc,
            id: value.id,
            method: value.method,
            params: value.params.try_into()?
        })
    }
}

Request!(
    pub struct CommonRequest {
        params:Unknown
    }
);

impl TryFrom<Unknown> for CommonRequest {
    type Error = String;
    fn try_from(value: Unknown) -> std::result::Result<Self, Self::Error> {
        let mut params = value.unwrap_as_map().ok_or("err")?;
        Ok(CommonRequest { jsonrpc: params.remove("jsonrpc").ok_or("err")?.unwrap_as_string().ok_or("err")?,
            id: params.remove("id").ok_or("err")?.unwrap_as_number().ok_or("err")?,
            method:  params.remove("method").ok_or("err")?.unwrap_as_string().ok_or("err")?,
            params: params.remove("params").ok_or("err")?
        })
    }
}


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
    experimental:Option<HashMap<String, Unknown>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<Unknown>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    completions: Option<Unknown>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    prompts:Option<Prompts>,
    // #[serde(skip_serializing_if = "Option::is_none")]
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
    params:Option<Unknown>
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
    pub fn run(mut self) -> std::result::Result<(), Box<dyn Error>>{
        // self.handle_initialization()?;
        for line in self.lines {
            if let Ok(line) = line {
                if let Ok (req) = serde_json::from_str::<CommonRequest>(&line) {
                    match req.method.as_str().into() {
                        RequestMethod::Initialize => {
                            let _req:InitializeRequest = req.try_into()?;
                            let res = InitializeResult::new(_req.jsonrpc, _req.id, _req.params.protocol_version, ServerCapabilities::new(None, None, None, None, Some(Tools { list_changed: Some(false) })), Implementation::new("ExampleServer".to_string(), Some("Example Server Display Name".to_string()), "2.0".to_string()), Some("this is a instruction!".to_string()));
                            let res = serde_json::to_string(&res)?;
                            // self.log(&res);
                            println!("{res}");
                        },
                        _ => {}
                    }
                }
                // match Notification!
            }
        }
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

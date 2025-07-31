#![allow(unused)]

use crate::util::Unknown;
use constant::{ notification::Notification, resources::Resource, tools::Tool, RequestMethod};
use result::{prompt::ListPromptResult, resoures::ListResourceResult, tools::{InputSchema, ListToolsResult, ToolDescription}, InitializeResult, ServerCapabilities, Tools};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fmt::format, fs::File, io::{Lines, Result, Stdin, StdinLock, Write}};
pub mod macros;
pub mod util;
pub mod constant;
pub mod result;
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
            id: value.id.ok_or("err")?,
            method: value.method,
            params: value.params.ok_or("err")?.try_into()?
        })
    }
}

Package!(
    pub struct CommonRequest {
        id:Option<i32>,
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

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
struct InitializedNotification {
    jsonrpc:String,
    method:String,
    params:Option<Unknown>
}

fn log(msg:&str) {
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("/Users/dadigua/Desktop/mcp-server/log.log")
        .unwrap();
    f.write_fmt(format_args!("{}\n",msg)).unwrap();
}
pub struct  McpServer {
    lines:Lines<StdinLock<'static>>,
}
impl McpServer {
    pub fn new(stdin:Stdin, log_file:File) -> Self {
        Self {
            lines:stdin.lines(),
        }
    }
    pub fn run(mut self) -> std::result::Result<(), Box<dyn Error>>{
        // self.handle_initialization()?;
        log("start!");
        for line in self.lines.into_iter() {
            if let Ok(line) = line {
                log(&line);
                if let Ok (req) = serde_json::from_str::<CommonRequest>(&line) {
                    match req.method.as_str().into() {
                        RequestMethod::Initialize => {
                            Self::handle_initialization(req.try_into()?)?;
                        },
                        RequestMethod::Notifications(noti) => {
                            match noti {
                                Notification::Initialized => {
                                    log("Initialized");
                                }
                            }
                        }
                        RequestMethod::Tools(tool) => {
                            match tool {
                                Tool::List => {
                                    log("List Tool");
                                    let r = ListToolsResult::new(req.jsonrpc, req.id.ok_or("err")?, vec![
                                        ToolDescription::new("HelloWorld".to_string(), Some(String::from("say hello world")), Some(String::from("call it to say hello world")), InputSchema::new(None, None), None,None),
                                    ]);
                                    let r = serde_json::to_string(&r).unwrap();
                                    println!("{r}");
                                    log(r.as_str());
                                }
                            }
                        }
                        RequestMethod::Resources(resource) => {
                            match resource {
                                Resource::List => {
                                    log("List Resources");
                                    let r = ListResourceResult::new(req.jsonrpc, req.id.ok_or("err")?, vec![]);
                                    let r = serde_json::to_string(&r).unwrap();
                                    println!("{r}");
                                    log(r.as_str());
                                }
                            }
                        }
                        RequestMethod::Prompts(prompt) => {
                            match prompt {
                                constant::prompt::Prompt::List => {
                                    log("List Prompts");
                                    let r = ListPromptResult::new(req.jsonrpc, req.id.ok_or("err")?, vec![]);
                                    let r = serde_json::to_string(&r).unwrap();
                                    println!("{r}");
                                    log(r.as_str());
                                }
                            }
                        }
                    }
                }
                // match Notification!
            }
        }
        Ok(())
    }
    fn handle_initialization(req:InitializeRequest) ->std::result::Result<(), Box<dyn Error>> {
        let _req:InitializeRequest = req.try_into()?;
        let res = InitializeResult::new(_req.jsonrpc, _req.id, _req.params.protocol_version, ServerCapabilities::new(None, None, None, None, Some(Tools::new(Some(false)))), Implementation::new("ExampleServer".to_string(), Some("Example Server Display Name".to_string()), "2.0".to_string()), Some("this is a instruction!".to_string()));
        let res = serde_json::to_string(&res)?;
        println!("{res}");
        log(res.as_str());
        Ok(())
    }


}

#![allow(unused)]

pub mod macros;
pub mod util;
pub mod constant;
pub mod result;
pub mod request;
pub mod notification;
pub mod tool;
use crate::util::Unknown;
use constant::{ notification::Notification, resources::Resource, tools::Tool, RequestMethod};
use request::{init::InitializeRequest, tool::CallToolRequest, CommonRequest};
use result::{prompt::ListPromptResult, resoures::ListResourceResult, tools::{InputSchema, ListToolsResult, ToolDescription}, InitializeResult, ServerCapabilities, Tools};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fmt::format, fs::File, io::{Lines, Result, Stdin, StdinLock, Write}};

BaseMetadata!(
    pub struct Implementation {
        version: String
    }
);
impl TryFrom<Unknown> for Implementation {
    type Error = serde_json::Error;
    fn try_from(value: Unknown) -> std::result::Result<Self, Self::Error> {
        // let mut imp = value.unwrap_as_map().ok_or("err")?;
        // let version = imp.remove("version").ok_or("err")?.unwrap_as_string().ok_or("err")?;
        // let name = imp.remove("name").ok_or("err")?.unwrap_as_string().ok_or("err")?;
        // let title = if let Some(t) = imp.remove("title") {
        //     Some(t.unwrap_as_string().ok_or("err")?)
        // }else {
        //     None
        // };
        // Ok(Implementation { version, name, title })
        let e = serde_json::to_string(&value)?;
        serde_json::from_str(&e)
    }
}
impl Implementation {
    pub fn new(name: String, title: Option<String>, version: String) -> Self {
        Implementation { name, title, version }
    }
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
                                        ToolDescription::new("HelloWorld".to_string(), Some(String::from("say hello world")), Some(String::from("call it to say hello world")), InputSchema::new({
                                            let mut properties = HashMap::new();
                                            properties.insert("name".to_string(),Unknown::Object({
                                                let mut des = HashMap::new();
                                                des.insert("type".to_string(), Unknown::String("string".to_string()));
                                                des.insert("description".to_string(), Unknown::String("name of the person to say hello to".to_string()));
                                                des
                                            }));
                                            Some(properties)
                                        }, Some(vec!["name".to_string()])), None,None),
                                    ]);
                                    let r = serde_json::to_string(&r).unwrap();
                                    println!("{r}");
                                    log(r.as_str());
                                }
                                Tool::Call => {
                                    log("Tool Call");
                                    let req =  CallToolRequest::try_from(req)?;
                                    let tool = |name:String| {
                                        log(name.as_str())
                                    };
                                    tool(req.params.arguments.ok_or("err")?.remove("name").ok_or("err")?.unwrap_as_string().ok_or("err")?);
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

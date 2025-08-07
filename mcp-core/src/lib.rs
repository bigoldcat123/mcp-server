#![allow(unused)]

pub mod macros;
pub mod constant;
pub mod result;
pub mod request;
pub mod notification;
pub mod tool;
use unknown::{Array, Object, String, Unknown,IntoUnknown};
use constant::{ notification::Notification, resources::Resource, tools::Tool, RequestMethod};
use request::{init::InitializeRequest, tool::CallToolRequest, CommonRequest};
use result::{prompt::{ListPromptResult, Prompt, PromptArgument}, resoures::ListResourceResult, tools::{InputSchema, ListToolsResult, ToolDescription}, CommonResult, InitializeResult, ServerCapabilities, Tools};
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
    pub fn new(name: impl Into<String>, title: Option<impl Into<String>>, version: impl Into<String>) -> Self {
        Implementation { name: name.into(), title: title.map(|t| t.into()), version: version.into() }
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
                                },
                                Notification::Cancelled => {
                                    log("Cancelled");
                                }
                            }
                        }
                        RequestMethod::Tools(tool) => {
                            match tool {
                                Tool::List => {
                                    log("List Tool");
                                    let r = ListToolsResult::new(req.jsonrpc, req.id.ok_or("err")?, vec![
                                        ToolDescription::new("minus".to_string(), Some(String::from("minus(a,b) => a-b")), Some(String::from("call it to do minus between two numbers")), InputSchema::new({
                                            Some(Object!{
                                                "num1" => Object!{
                                                    "type" => String!("number"),
                                                    "description" => String!("first number to subtract")
                                                },
                                                "num2" => Object!{
                                                    "type" => String!("number"),
                                                    "description" => String!("second number to subtract")
                                                }
                                            })
                                        }, Some(vec!["num1".to_string(),"num2".to_string()])), None,None),
                                    ]);
                                    let r = serde_json::to_string(&r).unwrap();
                                    println!("{r}");
                                    log(r.as_str());
                                }
                                Tool::Call => {
                                    log("Tool Call");
                                    let req =  CallToolRequest::try_from(req)?;
                                    let tool = |a:f64,b:f64| -> f64 {
                                        a - b
                                    };
                                    let mut args = req.params.arguments.ok_or("err")?;
                                    let mut num1 = args.remove("num1").ok_or("err")?.unwrap_as_float().ok_or("err")?;
                                    let mut num2 = args.remove("num2").ok_or("err")?.unwrap_as_float().ok_or("err")?;
                                    let res = tool(num1, num2);
                                    let r = CommonResult::ok(req.jsonrpc, req.id,
                                        Some(Object!{
                                            "content" => Array![ Object!{
                                                    "type" => String!("text"),
                                                    "text" => String!("{num1} - {num2} = {res}")
                                                }
                                            ]
                                        })
                                    );
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
                                    let r = ListPromptResult::new(req.jsonrpc, req.id.ok_or("err")?, vec![
                                        Prompt::new("code_review", Some("Request Code Review"), Some("Asks the LLM to analyze code quality and suggest improvements"), vec![
                                            PromptArgument::new("code", Option::<String>::None, Some("The code to review"), Some(true))
                                        ], None)
                                    ]);
                                    let r = serde_json::to_string(&r).unwrap();
                                    println!("{r}");
                                    log(r.as_str());
                                }
                                constant::prompt::Prompt::Get => {
                                    log("get prompt!");
                                    let mut params = req.params.ok_or("err")?.unwrap_as_map().ok_or("err")?;
                                    let name = params.remove("name").ok_or("err")?.unwrap_as_string().ok_or("err")?;
                                    let mut arguments = params.remove("arguments").ok_or("err")?.unwrap_as_map().ok_or("err")?;
                                    let code = arguments.remove("code").ok_or("err")?.unwrap_as_string().ok_or("err")?;

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
        let res = InitializeResult::new(_req.jsonrpc, _req.id, _req.params.protocol_version,
            ServerCapabilities::new(None, None, None, None, Some(Tools::new(Some(false)))),
            Implementation::new("ExampleServer",
                Some("Example Server Display Name"),
                "2.0"),
            Some("this is a instruction!")
        );
        let res = serde_json::to_string(&res)?;
        println!("{res}");
        log(res.as_str());
        Ok(())
    }


}

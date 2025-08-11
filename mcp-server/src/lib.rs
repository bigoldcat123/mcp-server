pub mod tool;
use std::{error::Error, io::{Lines, StdinLock}};

use mcp_core::{constant::{self, notification::Notification, resources::Resource, tools::Tool, RequestMethod}, log, request::{init::InitializeRequest, tool::CallToolRequest, CommonRequest}, result::{self, prompt::{GetPromptResult, ListPromptResult, PromptArgument, PromptDescription, PromptMessage}, resoures::{ListResourceResult, ReadResourceResult, ResourceContent, ResourceDescription}, tools::{InputSchema, ListToolsResult, ToolDescription}, CommonResult, ContentBlock, InitializeResult, ServerCapabilities, Tools}, Implementation};
use unknown::{Array, Object,String,Unknown,IntoUnknown};

pub struct McpServerBuilder {
    pub tools:Vec<ToolDescription>,
    pub resources:Vec<ResourceDescription>,
    pub prompts:Vec<PromptDescription>
}
impl McpServerBuilder {
    pub fn new() -> Self {
        McpServerBuilder {
            tools:vec![],
            resources:vec![],
            prompts:vec![]
        }
    }
    pub fn tools(mut self, mut tools:Vec<ToolDescription>) -> Self {
        self.tools.append(&mut tools);
        self
    }
    pub fn resources(mut self, mut resources:Vec<ResourceDescription>) -> Self {
        self.resources.append(&mut resources);
        self
    }
    pub fn prompts(mut self, mut prompts:Vec<PromptDescription>) -> Self {
        self.prompts.append(&mut prompts);
        self
    }
    pub fn build(self) -> McpServer {
        McpServer {
            lines:std::io::stdin().lines(),
            tools:self.tools,
            resources:self.resources,
            prompts:self.prompts
        }
    }
}



pub struct  McpServer {
    lines:Lines<StdinLock<'static>>,
    pub tools:Vec<ToolDescription>,
    pub resources:Vec<ResourceDescription>,
    pub prompts:Vec<PromptDescription>
}
impl McpServer {
    pub fn builder() -> McpServerBuilder {
        McpServerBuilder::new()
    }
    // pub fn new(stdin:Stdin, _log_file:File) -> Self {
    //     Self {
    //         lines:stdin.lines(),
    //         tools:Vec::new(),
    //         resources:Vec::new(),
    //         prompts:Vec::new()
    //     }
    // }
    pub fn run( self) -> std::result::Result<(), Box<dyn Error>>{
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
                                    let  num1 = args.remove("num1").ok_or("err")?.unwrap_as_float().ok_or("err")?;
                                    let  num2 = args.remove("num2").ok_or("err")?.unwrap_as_float().ok_or("err")?;
                                    let res = tool(num1, num2);
                                    let r = CommonResult::ok(req.jsonrpc, req.id,
                                        Some(Object!{
                                            "content" => Array![ Object!{
                                                    "type" => "text",
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
                                    let r = ListResourceResult::new(req.jsonrpc, req.id.ok_or("err")?, vec![
                                        result::resoures::ResourceDescription::new("lib.rs", Some("Rust Software Application Lib File"), "file:///Users/dadigua/Desktop/mcp-server/mcp-core/src/lib.rs", Some("Rust Software Application Main File"),Option::<String>::None, None, None, None)
                                    ]);
                                    let r = serde_json::to_string(&r).unwrap();
                                    println!("{r}");
                                    log(r.as_str());
                                },
                                Resource::Read => {
                                    let param = req.params.ok_or("err")?;
                                    let mut param = param.unwrap_as_map().ok_or("err")?;
                                    let uri = param.remove("uri").ok_or("err")?.unwrap_as_string().ok_or("err")?;
                                    log(format!("Read Resource: {}", uri).as_str());
                                    let res = ReadResourceResult::new(req.jsonrpc, req.id.ok_or("err")?,  vec![
                                        ResourceContent::new_text("Resource::Read", uri, Some("text/x-rust"))
                                    ]);
                                    let res = serde_json::to_string(&res).unwrap();
                                    println!("{res}");
                                    log(res.as_str());
                                }
                            }
                        }
                        RequestMethod::Prompts(prompt) => {
                            match prompt {
                                constant::prompt::Prompt::List => {
                                    log("List Prompts");
                                    let r = ListPromptResult::new(req.jsonrpc, req.id.ok_or("err")?, vec![
                                        PromptDescription::new("code_review", Some("Request Code Review"), Some("Asks the LLM to analyze code quality and suggest improvements"), vec![
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
                                    let _name = params.remove("name").ok_or("err")?.unwrap_as_string().ok_or("err")?;
                                    let mut arguments = params.remove("arguments").ok_or("err")?.unwrap_as_map().ok_or("err")?;
                                    let code = arguments.remove("code").ok_or("err")?.unwrap_as_string().ok_or("err")?;
                                    let res = GetPromptResult::new(req.jsonrpc, req.id.ok_or("err")?, Some("Code review prompt"), vec![
                                        PromptMessage::new(result::Role::User, ContentBlock::new_text(format!("Please review this Python code:\n {}",code),None, None))
                                    ]);
                                    let res = serde_json::to_string(&res).unwrap();
                                    println!("{res}");
                                    log(&res);
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
            ServerCapabilities::new(None, None, None, None, Some(Tools::new(Some(false))),None),
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

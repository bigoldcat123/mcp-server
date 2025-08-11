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
use result::{prompt::{GetPromptResult, ListPromptResult, PromptDescription, PromptArgument, PromptMessage}, resoures::{ListResourceResult, ReadResourceResult, ResourceContent}, tools::{InputSchema, ListToolsResult, ToolDescription}, CommonResult, ContentBlock, InitializeResult, ServerCapabilities, Tools};
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

pub fn log(msg:&str) {
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("/Users/dadigua/Desktop/mcp-server/log.log")
        .unwrap();
    f.write_fmt(format_args!("{}\n",msg)).unwrap();
}

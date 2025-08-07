use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::{ BaseMetadata, Result};
use unknown::Unknown;

use super::{ContentBlock, Role};


BaseMetadata!(
    pub struct PromptArgument {
        #[serde(skip_serializing_if = "Option::is_none")]
        description:Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        required:Option<bool>
    }
);
impl PromptArgument {
    pub fn new(name:impl Into<String>, title:Option<impl Into<String>>, description:Option<impl Into<String>>, required:Option<bool>) -> Self {
        Self {
            name:name.into(),
            title:title.map(Into::into),
            description:description.map(Into::into),
            required
        }
    }
}

BaseMetadata!(
    pub struct Prompt {
        #[serde(skip_serializing_if = "Option::is_none")]
        description:Option<String>,
        arguments:Vec<PromptArgument>,
        #[serde(skip_serializing_if = "Option::is_none")]
        _meta:Option<Unknown>
    }
);
impl Prompt {
    pub fn new(name:impl Into<String>,title:Option<impl Into<String>>
        ,description:Option<impl Into<String>>, arguments:Vec<PromptArgument>, _meta:Option<Unknown>) -> Self {
        Self {
            name: name.into(),
            title:title.map(Into::into),
            description:description.map(Into::into),
            arguments,
            _meta
        }
    }
}
#[derive(Debug,Serialize,Deserialize)]
struct InnerListPromptResult {
    prompts:Vec<Prompt>
}

Result! (
    pub struct ListPromptResult {
        result: InnerListPromptResult
    }
);

impl ListPromptResult {
    pub fn new(jsonrpc:impl Into<String>,id:i32,prompts: Vec<Prompt>) -> Self {
        Self {
            jsonrpc: jsonrpc.into(),
            id,
            result: InnerListPromptResult { prompts }
        }
    }
}
#[derive(Debug,Serialize,Deserialize)]
pub struct PromptMessage {
    role:Role,
    content:ContentBlock
}

impl PromptMessage {
    pub fn new(role:Role,content:ContentBlock) -> Self {
        Self {
            role,
            content
        }
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct InnerGetPromptResult {
    description:Option<String>,
    messages:Vec<PromptMessage>
}

Result!(
    pub struct GetPromptResult {
        result:InnerGetPromptResult
    }
);
impl GetPromptResult {
    pub fn new(jsonrpc:impl Into<String>,id:i32,description:Option<impl Into<String>>,messages:Vec<PromptMessage>) -> Self {
        Self {
            jsonrpc: jsonrpc.into(),
            id,
            result: InnerGetPromptResult { description: description.map(|s| s.into()), messages }
        }
    }
}

mod test {
    use crate::result::prompt::{Prompt, PromptArgument};

    use super::ListPromptResult;

    #[test]
    fn serialize() {
        let res = ListPromptResult::new("jsonrpc".to_string(), 1, vec![
            Prompt::new("abc",Some( "title"),Some( "descrption"), vec![
                PromptArgument::new("arg1", Some("arg1_title"), Some("arg1_description"), Some(true)),
                PromptArgument::new("arg2", Some("arg2_title"), Some("arg2_description"), Some(true))
            ], None)
        ]);
        let r = serde_json::to_string_pretty(&res).unwrap();
        println!("{r}");
    }

    #[test]
    fn deserialize() {
    }
}

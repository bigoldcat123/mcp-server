pub mod tools;
pub mod prompt;
pub mod resoures;
use std::collections::HashMap;
use unknown::Unknown;

use serde::{Deserialize, Serialize};

use crate::{ Implementation, Package, Result};

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}


#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    audience:Option<Vec<Role>>,
    priority:i32,
    last_modified:Option<String>
}

impl Annotation {
    pub fn new(audience:Option<Vec<Role>>,priority:i32,last_modified:Option<String>) -> Self {
        Self {
            audience,
            priority,
            last_modified
        }
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
    pub fn new(jsonrpc:impl Into<String>,id:i32,protocol_version: impl Into<String>, capabilities: ServerCapabilities,
        server_info: Implementation, instructions: Option<impl Into<String>>) -> Self {

        Self{
            jsonrpc:jsonrpc.into(),
            id,
            result: InnerInitializeResult {
            protocol_version:protocol_version.into(),
            capabilities,
            server_info,
            instructions:instructions.map(Into::into),
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub result:Option<Unknown>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error:Option<ResultError>
    }
);
impl CommonResult {
    pub fn ok(jsonrpc:String,id:i32,result:Option<Unknown>) -> Self {
        Self {
            jsonrpc,
            id,
            result,
            error:None,
        }
    }
    pub fn error(jsonrpc:String,id:i32,error:ResultError) -> Self {
        Self {
            jsonrpc,
            id,
            result:None,
            error:Some(error),
        }
    }
}
#[derive(Debug,Serialize,Deserialize)]
#[serde(tag="type",rename_all_fields="camelCase", rename_all = "camelCase")]
pub enum  ContentBlock {
    Text {
        text:String,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations:Option<Annotation>,
        #[serde(skip_serializing_if = "Option::is_none")]
        _meta:Option<Unknown>
    },
    Image{
        data:String,
        mime_type:String,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations:Option<Annotation>,
        #[serde(skip_serializing_if = "Option::is_none")]
        _meta:Option<Unknown>
    },
    Audio {
        data:String,
        mime_type:String,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations:Option<Annotation>,
        #[serde(skip_serializing_if = "Option::is_none")]
        _meta:Option<Unknown>
    }
}
impl ContentBlock {
    pub fn new_text(text: impl Into<String>, annotations: Option<Annotation>, _meta: Option<Unknown>) -> Self {
        Self::Text {
            text: text.into(),
            annotations,
            _meta,
        }
    }
    pub fn new_image(data: impl Into<String>, mime_type: impl Into<String>, annotations: Option<Annotation>, _meta: Option<Unknown>) -> Self {
        Self::Image {
            data: data.into(),
            mime_type: mime_type.into(),
            annotations,
            _meta,
        }
    }
    pub fn new_audio(data: impl Into<String>, mime_type: impl Into<String>, annotations: Option<Annotation>, _meta: Option<Unknown>) -> Self {
        Self::Audio {
            data: data.into(),
            mime_type: mime_type.into(),
            annotations,
            _meta,
        }
    }
}



#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};

    use super::ContentBlock;



    #[derive(Debug,Serialize,Deserialize)]
    struct A {
        content:ContentBlock
    }


    #[test]
    fn test_enum_serialize() {
        let a = A{content:ContentBlock::Image { data: "()".to_string(), mime_type: "()".to_string(), annotations: None, _meta: None }};
        let json = serde_json::to_string(&a).unwrap();
        println!("{}",json);
        let x:A = serde_json::from_str(json.as_str()).unwrap();
        println!("{:?}",x);
    }
}

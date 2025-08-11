
use std::collections::HashMap;

use crate::{ BaseMetadata, Result};
use unknown::Unknown;
use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize,Debug,Default)]
#[serde(rename_all = "camelCase")]
pub struct ToolAnnotations {
    #[serde(skip_serializing_if = "Option::is_none")]
    title:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only_hint:Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destructive_hint:Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idempotent_hint:Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_world_hint:Option<bool>,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct InputSchema {
    r#type:String,// "object"
    #[serde(skip_serializing_if = "Option::is_none")]
    properties:Option<Unknown>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required:Option<Vec<String>>
}
impl InputSchema {
    pub fn new(properties:Option<Unknown>,required:Option<Vec<String>>) -> Self {
        Self {
            r#type:"object".to_string(),
            properties,
            required
        }
    }
}
type OutputSchema = InputSchema;

BaseMetadata!(
    pub struct ToolDescription {
        #[serde(skip_serializing_if = "Option::is_none")]
        description:Option<String>,
        input_schema:InputSchema,
        #[serde(skip_serializing_if = "Option::is_none")]
        output_schema:Option<OutputSchema>,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations:Option<ToolAnnotations>
    }
);
impl ToolDescription {
    pub fn new(name:impl Into<String>,title:Option<impl Into<String>>,description:Option<impl Into<String>>,input_schema:InputSchema,output_schema:Option<OutputSchema>,annotations:Option<ToolAnnotations>) -> Self {
        Self {
            name: name.into(),
            title: title.map(|t| t.into()),
            description: description.map(|d| d.into()),
            input_schema,
            output_schema,
            annotations
        }
    }
}
#[derive(Serialize,Deserialize,Debug)]
struct InnerListToolsResult {
    tools:Vec<ToolDescription>
}
Result!(
    pub struct ListToolsResult {
        result: InnerListToolsResult
    }
);
impl ListToolsResult {
    pub fn new(jsonrpc:String,id:i32,tools: Vec<ToolDescription>) -> Self {
        Self {
            jsonrpc,
            id,
            result: InnerListToolsResult { tools }
        }
    }
}

mod test {

    use std::collections::HashMap;

    use unknown::{Object, Unknown,String,IntoUnknown};

    use super::{InputSchema, ListToolsResult, OutputSchema, ToolDescription};

    #[test]
    fn create() {
        let r = ListToolsResult::new("2.0".to_string(), 1, vec![
            ToolDescription::new("tool1".to_string(),
                Some(String::from("title")),
                Some(String::from("description")),
                InputSchema::new(Some(
                    Object!(
                        "localtion" => Object! {
                            "type" => String!("string"),
                            "description" => String!("city name or zip code")
                        }
                    )
                ), Some(vec!["location".to_string()])),
                None,
                None),
        ]);
        let r = serde_json::to_string_pretty(&r).unwrap();
        println!("{r}");
    }
}

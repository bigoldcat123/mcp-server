use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use unknown::Unknown;

use crate::{ BaseMetadata, ResourceContents, Result};

use super::Annotation;



BaseMetadata!(
    pub struct ResourceDescription {
        uri:String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description:Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        mime_type:Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations:Option<Annotation>,
        #[serde(skip_serializing_if = "Option::is_none")]
        size:Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        _meta:Option<HashMap<String, Unknown>>
    }
);
impl ResourceDescription {
    pub fn new(name:impl Into<String>,title:Option<impl Into<String>>,uri:impl Into<String>,description:Option<impl Into<String>>,mime_type:Option<impl Into<String>>,annotations:Option<Annotation>,size:Option<i32>,_meta:Option<HashMap<String, Unknown>>) -> Self {
        Self {
            name: name.into(),
            title: title.map(|t| t.into()),
            uri: uri.into(),
            description: description.map(|d| d.into()),
            mime_type: mime_type.map(|m| m.into()),
            annotations,
            size,
            _meta
        }
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct InnerListResourceResult {
    resources: Vec<ResourceDescription>
}
impl InnerListResourceResult {
    pub fn new(resources: Vec<ResourceDescription>) -> Self {
        Self { resources }
    }
}

Result!(
    pub struct ListResourceResult {
        result: InnerListResourceResult
    }
);
impl ListResourceResult {
    pub fn new(jsonrpc:String,id:i32,resources: Vec<ResourceDescription>) -> Self {
        Self {
            jsonrpc,
            id,
            result: InnerListResourceResult { resources }
        }
    }
}


ResourceContents!(
    pub struct TextResourceContents {
        text:String
    }
);

ResourceContents!(
    pub struct BlobResourceContents {
        blob:String
    }
);
#[derive(Debug,Serialize,Deserialize)]
#[serde(untagged)]
pub enum ResourceContent {
    Text(TextResourceContents),
    Blob(BlobResourceContents)
}
impl ResourceContent  {
    pub fn new_text(text:impl Into<String>,uri:impl Into<String>,mime_type:Option<impl Into<String>>) -> Self {
        Self::Text(TextResourceContents{
            text:text.into(),
            uri:uri.into(),
            mime_type:mime_type.map(|mime_type| mime_type.into()),
            _meta:None
        })
    }
    pub fn new_blob(blob:impl Into<String>,uri:impl Into<String>,mime_type:Option<impl Into<String>>) -> Self {
        Self::Blob(BlobResourceContents{
            blob:blob.into(),
            uri:uri.into(),
            mime_type:mime_type.map(|mime_type| mime_type.into()),
            _meta:None
        })
    }
}
#[derive(Serialize,Deserialize,Debug)]
pub struct InnerReadResourceResult {
    contents:Vec<ResourceContent>
}

Result!(
    pub struct ReadResourceResult {
        pub result:InnerReadResourceResult
    }
);


impl ReadResourceResult {
    pub fn new(jsonrpc:impl Into<String>,id:i32,contents:Vec<ResourceContent>) -> Self {
        Self{
            jsonrpc:jsonrpc.into(),
            id,
            result: InnerReadResourceResult { contents }
        }
    }
}

mod test {
    use crate::result::resoures::{ListResourceResult, ResourceDescription};


    #[test]
    fn deserialize() {
        let json = r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "resources": [
                    {
                        "name": "example",
                        "title": "Example Resource",
                        "uri": "https://example.com",
                        "description": "An example resource",
                        "mime_type": "text/plain",
                        "annotations": {
                            "audience": ["user", "assistant"],
                            "priority": 1,
                            "last_modified": "2023-01-01T00:00:00Z"
                        },
                        "size": 1024,
                        "_meta": {
                            "key1": "value1",
                            "key2": "value2"
                        }
                    }
                ]
            }
        }"#;

        let result: ListResourceResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.jsonrpc, "2.0");
        assert_eq!(result.id, 1);
        assert_eq!(result.result.resources.len(), 1);
    }
    #[test]
    fn serialize() {
        let result = ListResourceResult::new("2.0".to_string(), 1, vec![
            ResourceDescription::new("name".to_string(), Some("title".to_string()), "uri".to_string(),Some( "description".to_string()), Some("mime_type".to_string()), None, Some(1024),None)
        ]);
        let json = serde_json::to_string_pretty(&result).unwrap();
        println!("{json}");
    }
}

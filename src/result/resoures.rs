use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{util::Unknown, BaseMetadata, Result};

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceAnnotation {
    audience:Option<Vec<Role>>,
    priority:i32,
    last_modified:Option<String>
}

impl ResourceAnnotation {
    pub fn new(audience:Option<Vec<Role>>,priority:i32,last_modified:Option<String>) -> Self {
        Self {
            audience,
            priority,
            last_modified
        }
    }
}

BaseMetadata!(
    pub struct Resource {
        uri:String,
        description:Option<String>,
        mime_type:Option<String>,
        annotations:Option<ResourceAnnotation>,
        size:Option<i32>,
        _meta:Option<HashMap<String, Unknown>>
    }
);
impl Resource {
    pub fn new(name:String,title:Option<String>,uri:String,description:Option<String>,mime_type:Option<String>,annotations:Option<ResourceAnnotation>,size:Option<i32>,_meta:Option<HashMap<String, Unknown>>) -> Self {
        Self {
            name,
            title,
            uri,
            description,
            mime_type,
            annotations,
            size,
            _meta
        }
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct InnerListResourceResult {
    resources: Vec<Resource>
}
impl InnerListResourceResult {
    pub fn new(resources: Vec<Resource>) -> Self {
        Self { resources }
    }
}

Result!(
    pub struct ListResourceResult {
        result: InnerListResourceResult
    }
);
impl ListResourceResult {
    pub fn new(jsonrpc:String,id:i32,resources: Vec<Resource>) -> Self {
        Self {
            jsonrpc,
            id,
            result: InnerListResourceResult { resources }
        }
    }
}


mod test {
    use crate::result::resoures::{ListResourceResult, Resource};


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
            Resource::new("name".to_string(), Some("title".to_string()), "uri".to_string(),Some( "description".to_string()), Some("mime_type".to_string()), None, Some(1024),None)
        ]);
        let json = serde_json::to_string_pretty(&result).unwrap();
        println!("{json}");
    }
}

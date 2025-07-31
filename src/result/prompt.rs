use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::{util::Unknown, BaseMetadata, Result};

BaseMetadata!(
    pub struct PromptArgument {
        description:Option<String>,
        required:Option<bool>
    }
);

BaseMetadata!(
    pub struct Prompt {
        description:Option<String>,
        arguments:Option<PromptArgument>,
        _meta:Option<HashMap<String,Unknown>>
    }
);

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
    pub fn new(jsonrpc:String,id:i32,prompts: Vec<Prompt>) -> Self {
        Self {
            jsonrpc,
            id,
            result: InnerListPromptResult { prompts }
        }
    }
}
mod test {
    use crate::result::prompt::{Prompt, PromptArgument};

    use super::ListPromptResult;

    #[test]
    fn serialize() {
        let res = ListPromptResult::new("jsonrpc".to_string(), 1, vec![]);
        let r = serde_json::to_string(&res).unwrap();
        assert_eq!(r, "{\"jsonrpc\":\"jsonrpc\",\"id\":1,\"result\":{\"prompts\":[]}}");
    }

    #[test]
    fn deserialize() {
    }
}

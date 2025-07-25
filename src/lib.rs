#![allow(unused)]

use crate::util::Object;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod util;
///``` typescript
/// export interface Request {
///     method: string;
///     params?: {
///         _meta?: {
///             progressToken?: ProgressToken;
///             [key: string]: unknown;
///         };
///         [key: string]: unknown;
///     };
/// }
///```
macro_rules! Request {
    (pub struct $name:ident { $($filed_name:ident:$type:ty),* }) => {
        #[derive(Serialize,Deserialize,Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            jsonrpc: String,
            id:i32,
            method: String,
            $(
                $filed_name:$type,
            )*
        }
    };
}
macro_rules! BaseMetadata {
    (pub struct $name:ident { $( $filed_name:ident:$type:ty ),* }) => {
        #[derive(Deserialize,Serialize,Debug)]
        #[serde(rename_all = "camelCase")]
        struct $name {
            name:String,
            title:Option<String>,
            $(
                $filed_name:$type
            )*
        }
    };
}
struct Unknow {}

#[derive(Deserialize, Serialize,Debug)]
#[serde(rename_all = "camelCase")]
 struct ClientCapabilitiesRoots {
    list_changed: Option<bool>,
}

#[derive(Deserialize, Serialize,Debug)]
 struct ClientCapabilities {
    experimental: Option<HashMap<String, Object>>,
    roots: Option<ClientCapabilitiesRoots>,
    sampling: Option<HashMap<String,Object>>,
    elicitation: Option<HashMap<String,Object>>,
}

#[derive(Deserialize, Serialize,Debug)]
#[serde(rename_all = "camelCase")]
 struct InitializeRequesParam {
    client_info: Implementation,
    capabilities: ClientCapabilities,
    protocol_version:String
}
BaseMetadata!(
    pub struct Implementation {
        version: String
    }
);

Request!(
    pub struct InitializeRequest {
        params: InitializeRequesParam
    }
);

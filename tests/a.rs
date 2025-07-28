use std::{fs::File, io::Read};

use mcp_server::{util::Object, Implementation, InitializeRequest, InitializeResult, ServerCapabilities};
use serde::{Deserialize, Serialize};

#[test]
fn test_json() {
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Stu {
        name: Object,
        home_number: i32,
    }
    let stu = Stu {
        name: Object::Number(111),
        home_number: 111,
    };
    let res = serde_json::to_string(&stu).unwrap();
    println!("{}", res);
    let e = serde_json::from_str::<Stu>(&res.as_str()).unwrap();
    println!("{:?}", e);
}

#[test]
fn test_de() {
    let mut s = String::new();
    let _ = File::open("/Users/dadigua/Desktop/mcp-server/log.json")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    println!("{:?}", s);
    let e = serde_json::from_str::<InitializeRequest>(&s).unwrap();
    println!("{:?}",e);
}


#[test]
fn seri_result() {
    let req = InitializeResult::new("2.0".to_string(), 1, "2025-06-18".to_string(), ServerCapabilities::new(None, None, None, None, None), Implementation::new("ExampleServer".to_string(), Some("Example Server Display Name".to_string()), "222.222".to_string()), Some("this is a instruction!".to_string()));
    let res = serde_json::to_string_pretty(&req).unwrap();
    println!("{}", res);
}

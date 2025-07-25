use std::{fs::File, io::Read};

use mcp_server::{InitializeRequest, util::Object};
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

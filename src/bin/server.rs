use std::{
    fs::File,
    io::{Write, stdin},
};

use mcp_server::{Implementation, InitializeRequest, InitializeResult, ServerCapabilities};

fn main() {
    let mut f = File::create("/Users/dadigua/Desktop/mcp-server/log.log").unwrap();
    let stdin = stdin();
    stdin.lines().for_each(|l| {
        if let Ok(l) = l {
            let req = serde_json::from_str::<InitializeRequest>(&l);
            // println!("from server {l}");
            if req.is_ok() {
                let res = InitializeResult::new("2.0".to_string(), 0, "2025-03-26".to_string(), ServerCapabilities::new(None, None, None, None, None), Implementation::new("ExampleServer".to_string(), Some("Example Server Display Name".to_string()), "2.0".to_string()), Some("this is a instruction!".to_string()));
                let res = serde_json::to_string(&res).unwrap();
                println!("{res}");
                write!(f, "{}\n", res).unwrap();

            }
            write!(f, "{}\n", l).unwrap();
        } else {
            eprintln!("error!")
        }
    });
}

use std::{
    fs::File,
    io::{Write, stdin},
};

use mcp_server::InitializeRequest;

fn main() {
    let mut f = File::create("/Users/dadigua/Desktop/mcp-server/log.log").unwrap();
    let stdin = stdin();
    stdin.lines().for_each(|l| {
        if let Ok(l) = l {
            let req = serde_json::from_str::<InitializeRequest>(&l);
            println!("from server {l}");
            write!(f, "{}\n", l).unwrap();
            write!(f, "{:?}\n", req).unwrap();
        } else {
            eprintln!("error!")
        }
    });
}

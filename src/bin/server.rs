use std::{
    fs::File,
    io::stdin,
};

use mcp_server::McpServer;

fn main() {
    let f = File::create("/Users/dadigua/Desktop/mcp-server/log.log").unwrap();

    let stdin = stdin();
    let server = McpServer::new(stdin,f);
    server.run().unwrap();
}

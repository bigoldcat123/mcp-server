
use mcp_core::result::tools::{InputSchema, ToolDescription};
use mcp_server::McpServer;
use unknown::{Object,IntoUnknown,Unknown};


fn main() {

    let server = McpServer::builder()
        .tools(vec![
            ToolDescription::new("add", Some("title"), Some("description"), InputSchema::new(Some(Object!{
                "field_name" => Object!{
                    "type" => "string",
                    "description" => "Field description"
                }
            }), Some(vec![])), None, None)
        ]).build();
    if let Err(e) = server.run() {
        eprintln!("Error: {}", e);
    }
}

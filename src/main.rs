use rmcp::{
    handler::server::ServerHandler,
    model::{Implementation, ProtocolVersion, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
    transport::stdio,
    ServiceExt,
};

#[derive(Clone)]
struct SerialFluxServer {
    tool_router: rmcp::handler::server::tool::ToolRouter<Self>,
}

#[tool_router]
impl SerialFluxServer {
    fn new() -> Self {
        Self { tool_router: Self::tool_router() }
    }

    #[tool(description = "Return Serial Flux MCP server version and integration status")]
    async fn sf_server_info(&self) -> String {
        serde_json::json!({
            "name": "Serial Flux MCP",
            "version": env!("CARGO_PKG_VERSION"),
            "backend": "not connected",
            "hardware_control": false
        }).to_string()
    }

    #[tool(description = "List connections managed by the Serial Flux bridge (currently unavailable)")]
    async fn sf_list_connections(&self) -> String {
        serde_json::json!({
            "connections": [],
            "bridge_connected": false,
            "message": "C# Serial Flux bridge has not been implemented yet"
        }).to_string()
    }
}

#[tool_handler]
impl ServerHandler for SerialFluxServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::LATEST,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("Serial Flux MCP prototype. Hardware tools require a future bridge.".into()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // MCP stdio requires stdout to contain protocol messages only.
    eprintln!("Serial Flux MCP server starting");
    let service = SerialFluxServer::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}

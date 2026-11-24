use rmcp::{
    handler::server::ServerHandler,
    model::{Implementation, ProtocolVersion, ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
    handler::server::wrapper::Parameters,
    transport::stdio,
    ServiceExt,
};

#[derive(Clone)]
struct SerialFluxServer {
    tool_router: rmcp::handler::server::tool::ToolRouter<Self>,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
struct ConnectionArgs {
    /// Connection name or unique ID (case-insensitive).
    id: String,
}
#[derive(serde::Deserialize, schemars::JsonSchema)]
struct SendArgs {
    /// Connection name or unique ID (case-insensitive).
    id: String, data: String, format: String,
    interpret_escapes: Option<bool>,
}
#[derive(serde::Deserialize, schemars::JsonSchema)]
struct ReceiveArgs {
    /// Connection name or unique ID (case-insensitive).
    id: String, max_bytes: u32,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
struct VersionHistoryArgs {
    /// Maximum number of newest releases; omit for all. Zero returns no entries.
    limit: Option<u32>,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
struct CreateArgs {
    name: String, connect: Option<bool>,
    port: Option<String>, baud_rate: Option<u32>, data_bits: Option<u8>, parity: Option<String>, stop_bits: Option<f64>, flow_control: Option<String>, dtr: Option<bool>, rts: Option<bool>,
}
#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
struct ConfigureArgs {
    id: String, reconnect: Option<bool>,
    port: Option<String>, baud_rate: Option<u32>, data_bits: Option<u8>, parity: Option<String>, stop_bits: Option<f64>, flow_control: Option<String>, dtr: Option<bool>, rts: Option<bool>,
}
#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
struct RenameArgs { id: String, name: String, reconnect: Option<bool> }
#[derive(serde::Deserialize, schemars::JsonSchema)]
struct QueryArgs { id: String, data: String, format: String, interpret_escapes: Option<bool>, timeout_ms: Option<u32> }
#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
struct ProjectOpenArgs { path: String, unsaved_changes: Option<String> }
#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
struct ProjectPolicyArgs { unsaved_changes: Option<String> }
#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
struct ProjectSaveAsArgs { path: String, overwrite: Option<bool> }
fn ascii_bytes(data: &str, escapes: bool) -> Result<Vec<u8>, String> {
    if !data.is_ascii() { return Err("ASCII required".into()); }
    if !escapes { return Ok(data.as_bytes().to_vec()); }
    let mut it = data.bytes(); let mut out = Vec::new();
    while let Some(b) = it.next() { if b != 92 { out.push(b); continue; }
        out.push(match it.next() { Some(b'r') => 13, Some(b'n') => 10, Some(b't') => 9, Some(b'0') => 0, Some(b'b') => 8, Some(b'e') => 27, Some(92) => 92,
            Some(b'x') => { let a = it.next().ok_or("Incomplete hex escape")?; let b = it.next().ok_or("Incomplete hex escape")?; let h = [a,b]; u8::from_str_radix(std::str::from_utf8(&h).map_err(|_| "Invalid hex escape")?,16).map_err(|_| "Invalid hex escape")? }, _ => return Err("Invalid escape; disable interpret_escapes for literal text".into()) });
    } Ok(out)
}
async fn request_args(command: &str, args: impl serde::Serialize) -> String {
    let mut value = serde_json::to_value(args).unwrap(); value["command"] = serde_json::json!(command); bridge_request(&value.to_string()).await.to_string()
}
#[tool_router]
impl SerialFluxServer {
    fn new() -> Self {
        Self { tool_router: Self::tool_router() }
    }

    #[tool(description = "Current project information and modified status")]
    async fn sf_project_info(&self) -> String { bridge_request("project_info").await.to_string() }
    #[tool(description = "Recently opened project paths")]
    async fn sf_project_list_recent(&self) -> String { bridge_request("project_list_recent").await.to_string() }
    #[tool(description = "Save current named project")]
    async fn sf_project_save(&self) -> String { bridge_request("project_save").await.to_string() }
    #[tool(description = "Open full .sflux path; unsaved_changes reject (default), save or discard")]
    async fn sf_project_open(&self, Parameters(args): Parameters<ProjectOpenArgs>) -> String { request_args("project_open", args).await }
    #[tool(description = "Save to full .sflux path; overwrite defaults false")]
    async fn sf_project_save_as(&self, Parameters(args): Parameters<ProjectSaveAsArgs>) -> String { request_args("project_save_as", args).await }
    #[tool(description = "Reset to untitled project using SF New behavior; unsaved_changes reject, save or discard")]
    async fn sf_project_new(&self, Parameters(args): Parameters<ProjectPolicyArgs>) -> String { request_args("project_new", args).await }
    #[tool(description = "Close project and reset to untitled; unsaved_changes reject, save or discard")]
    async fn sf_project_close(&self, Parameters(args): Parameters<ProjectPolicyArgs>) -> String { request_args("project_close", args).await }
    #[tool(description = "Reload current project; unsaved_changes reject, save or discard")]
    async fn sf_project_reload(&self, Parameters(args): Parameters<ProjectPolicyArgs>) -> String { request_args("project_reload", args).await }
    #[tool(description = "Return MCP server release history, newest first; optional limit")]
    async fn sf_version_history(&self, Parameters(args): Parameters<VersionHistoryArgs>) -> String {
        let mut entries: Vec<serde_json::Value> = match serde_json::from_str(include_str!("../VersionHistory.json")) {
            Ok(v) => v,
            Err(e) => return serde_json::json!({"error":e.to_string()}).to_string(),
        };
        let total = entries.len();
        if let Some(limit) = args.limit { entries.truncate(limit as usize); }
        serde_json::json!({"current_version":env!("CARGO_PKG_VERSION"),"total":total,"count":entries.len(),"versions":entries}).to_string()
    }

    #[tool(description = "Return Serial Flux MCP server version and integration status")]
    async fn sf_server_info(&self) -> String {
        let bridge = bridge_request("status").await;
        serde_json::json!({"name":"Serial Flux MCP", "version":env!("CARGO_PKG_VERSION"),
            "hardware_control":bridge.get("hardware_control").and_then(|v| v.as_bool()).unwrap_or(false), "bridge":bridge}).to_string()
    }

    #[tool(description = "List configured Serial Flux connections and their live connection status")]
    async fn sf_list_connections(&self) -> String {
        bridge_request("list_connections").await.to_string()
    }

    #[tool(description = "List available Windows COM ports")]
    async fn sf_list_ports(&self) -> String { bridge_request("list_ports").await.to_string() }
    #[tool(description = "Get status by connection name or unique ID")]
    async fn sf_connection_status(&self, Parameters(args): Parameters<ConnectionArgs>) -> String {
        bridge_request(&serde_json::json!({"command":"connection_status","id":args.id}).to_string()).await.to_string()
    }
    #[tool(description = "Open a configured connection")]
    async fn sf_connect(&self, Parameters(args): Parameters<ConnectionArgs>) -> String {
        bridge_request(&serde_json::json!({"command":"connect","id":args.id}).to_string()).await.to_string()
    }
    #[tool(description = "Close a configured connection")]
    async fn sf_disconnect(&self, Parameters(args): Parameters<ConnectionArgs>) -> String {
        bridge_request(&serde_json::json!({"command":"disconnect","id":args.id}).to_string()).await.to_string()
    }
    #[tool(description = "Clear only the MCP receive buffer")]
    async fn sf_clear_receive(&self, Parameters(args): Parameters<ConnectionArgs>) -> String {
        bridge_request(&serde_json::json!({"command":"clear_receive","id":args.id}).to_string()).await.to_string()
    }
    #[tool(description = "Send data to an already connected device. Format ascii, hex, or binary (space-separated 8-bit groups). No automatic newline.")]
    async fn sf_send(&self, Parameters(args): Parameters<SendArgs>) -> String {
        let bytes: Result<Vec<u8>, String> = match args.format.to_lowercase().as_str() {
            "ascii" if args.data.is_ascii() => ascii_bytes(&args.data, args.interpret_escapes.unwrap_or(true)),
            "hex" => { let h: String = args.data.chars().filter(|c| !c.is_whitespace()).collect();
                if h.len()%2 != 0 || !h.is_ascii() { Err("Invalid hex".into()) } else { (0..h.len()).step_by(2).map(|i| u8::from_str_radix(&h[i..i+2],16).map_err(|_| "Invalid hex".into())).collect() } },
            "binary" => args.data.split_whitespace().map(|b| if b.len()==8 { u8::from_str_radix(b,2).map_err(|_| "Invalid binary".into()) } else { Err("Expected 8-bit groups".into()) }).collect(),
            _ => Err("Expected ASCII, hex or binary".into())
        };
        match bytes {
            Ok(b) if !b.is_empty() && b.len() <= 4096 => { let hex: String = b.iter().map(|x| format!("{:02X}",x)).collect::<Vec<_>>().join(""); bridge_request(&serde_json::json!({"command":"send","id":args.id,"hex":hex}).to_string()).await.to_string() },
            Ok(_) => serde_json::json!({"error":"Send requires 1–4096 bytes"}).to_string(),
            Err(e) => serde_json::json!({"error":e}).to_string()
        }
    }
    #[tool(description = "Create serial connection with SF defaults")]
    async fn sf_create_connection(&self, Parameters(args): Parameters<CreateArgs>) -> String { request_args("create_connection", args).await }
    #[tool(description = "Patch serial connection settings")]
    async fn sf_configure_connection(&self, Parameters(args): Parameters<ConfigureArgs>) -> String { request_args("configure_connection", args).await }
    #[tool(description = "Rename connection")]
    async fn sf_rename_connection(&self, Parameters(args): Parameters<RenameArgs>) -> String { request_args("rename_connection", args).await }
    #[tool(description = "Delete disconnected non-primary connection")]
    async fn sf_delete_connection(&self, Parameters(args): Parameters<ConnectionArgs>) -> String { request_args("delete_connection", args).await }
    #[tool(description = "Get connection configuration")]
    async fn sf_get_connection_config(&self, Parameters(args): Parameters<ConnectionArgs>) -> String { request_args("get_connection_config", args).await }
    #[tool(description = "Reconnect configured connection")]
    async fn sf_reconnect(&self, Parameters(args): Parameters<ConnectionArgs>) -> String { request_args("reconnect", args).await }
    #[tool(description = "Send and collect response for timeout_ms (default 1000, max 20000); clears MCP buffer first")]
    async fn sf_query(&self, Parameters(args): Parameters<QueryArgs>) -> String {
        let bytes: Result<Vec<u8>, String> = match args.format.to_lowercase().as_str() {
            "ascii" if args.data.is_ascii() => ascii_bytes(&args.data, args.interpret_escapes.unwrap_or(true)),
            "hex" => { let h: String = args.data.chars().filter(|c| !c.is_whitespace()).collect();
                if h.len()%2 != 0 || !h.is_ascii() { Err("Invalid hex".into()) } else { (0..h.len()).step_by(2).map(|i| u8::from_str_radix(&h[i..i+2],16).map_err(|_| "Invalid hex".into())).collect() } },
            "binary" => args.data.split_whitespace().map(|b| if b.len()==8 { u8::from_str_radix(b,2).map_err(|_| "Invalid binary".into()) } else { Err("Expected 8-bit groups".into()) }).collect(),
            _ => Err("Expected ASCII, hex or binary".into())
        };
        match bytes { Ok(b) if !b.is_empty() && b.len() <= 4096 => { let hex = b.iter().map(|x| format!("{:02X}",x)).collect::<Vec<_>>().join(""); bridge_request(&serde_json::json!({"command":"query","id":args.id,"hex":hex,"timeout_ms":args.timeout_ms.unwrap_or(1000).clamp(1,20000)}).to_string()).await.to_string() }, Ok(_) => serde_json::json!({"error":"Send requires 1–4096 bytes"}).to_string(), Err(e) => serde_json::json!({"error":e}).to_string() }
    }
    #[tool(description = "Read up to max_bytes from the MCP receive buffer without consuming bytes; returns ASCII and hex")]
    async fn sf_receive(&self, Parameters(args): Parameters<ReceiveArgs>) -> String {
        bridge_request(&serde_json::json!({"command":"receive","id":args.id,"max_bytes":args.max_bytes}).to_string()).await.to_string()
    }

}

#[tool_handler]
impl ServerHandler for SerialFluxServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::LATEST,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("Serial Flux MCP bridge. Enable access in SF Tools > MCP Server Settings.".into()),
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

#[cfg(windows)]
async fn bridge_request(command: &str) -> serde_json::Value {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::windows::named_pipe::ClientOptions;
    let result = tokio::time::timeout(std::time::Duration::from_secs(30), async {
        let mut pipe = ClientOptions::new().open(r"\\.\pipe\SerialFlux.Mcp.Api.v1")?;
        pipe.write_all(format!("{}\n", command).as_bytes()).await?;
        let mut reader = BufReader::new(pipe);
        let mut response = String::new();
        reader.read_line(&mut response).await?;
        serde_json::from_str::<serde_json::Value>(&response).map_err(std::io::Error::other)
    }).await;
    match result {
        Ok(Ok(value)) => value,
        Ok(Err(error)) => serde_json::json!({"bridge_connected":false,"error":error.to_string()}),
        Err(_) => serde_json::json!({"bridge_connected":false,"error":"Bridge request timed out"}),
    }
}

#[cfg(not(windows))]
async fn bridge_request(_command: &str) -> serde_json::Value {
    serde_json::json!({"bridge_connected":false,"error":"Windows bridge required"})
}

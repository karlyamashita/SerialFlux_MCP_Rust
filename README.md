# Serial Flux MCP — Rust

Independent Rust MCP server prototype for KTYLabs Serial Flux. No Visual Studio solution is required.

## Requirements
- Rust stable toolchain (`rustup`, `cargo`)
- Windows x64 or Windows ARM64 (native builds)

## Build
```powershell
cargo build --release
```

Run the server with `cargo run --release` or the generated `target/release/serialflux-mcp.exe`.

The server uses MCP **stdio** transport. It must be launched by an MCP client rather than used as an interactive terminal application. stdout is reserved for MCP JSON-RPC traffic; diagnostic logs go to stderr.

## MCP tools (prototype)
- `sf_server_info`: reports server version and backend status.
- `sf_list_connections`: returns an empty connection list and explains that the C# bridge is not implemented.

**No serial ports are opened and no hardware is controlled by this prototype.**

## Example MCP client configuration
```json
{
  "mcpServers": {
    "serialflux": {
      "command": "C:\\path\\to\\serialflux-mcp.exe"
    }
  }
}
```

The exact client configuration format depends on your MCP client.

## Next phase
Add an authenticated local named-pipe bridge in the C# Serial Flux application. The Rust MCP server will call the bridge rather than opening ports already owned by the GUI. Later, individual backends can move into Rust.

## Note
This source scaffold has not been compiled or tested in this environment. Validate it with `cargo check` against the resolved SDK version before shipping.

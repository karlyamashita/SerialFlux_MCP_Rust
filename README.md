# Serial Flux MCP v0.1.4
Build with cargo build --release. Run updated SF and open Tools > MCP Server Settings. Enable bridge; read-only is default. Enable hardware control only for trusted clients. Settings persist per Windows user. Existing environment variable is used only when no settings file exists. MCP Inspector uses STDIO and the Rust executable. List connections to obtain IDs. Receive is a non-destructive snapshot of oldest buffered bytes (64 KiB per connection, oldest dropped on overflow); clear_receive clears MCP buffer only. Send supports ASCII, hex and binary groups, no implicit terminator, 4096 bytes maximum. Shared RTT aliases are not separately captured. Do not retry timed-out hardware writes automatically. Windows compilation and runtime testing required.

Connection-specific tools accept a connection name (e.g. Default) or unique ID in the id field with the updated C# bridge. Duplicate names require a unique ID.

## Universal Windows installer (0.1.4)
Run BuildInstaller.bat on Windows with Rust/rustup, Visual Studio C++ build tools (x64 and ARM64 MSVC and Windows SDK), and NSIS 3.x installed. Both targets must build successfully. Cargo.toml is the version source; Cargo.lock must match. The installer and SHA-256 text file are written to Downloads. Close active MCP clients before upgrading. The installer selects the native architecture and creates an optional desktop shortcut to the offline HTML guide. STDIO remains unchanged. This release does not change the MCP tools or ASCII parsing.

## Version management
Run `VersionManager.bat` (Python with Tkinter required). New Release increments the patch version; Save Entry synchronizes Cargo metadata, JSON history, changelog and offline guide. Rebuild the installer to embed the new history. `sf_version_history` accepts optional `limit` (omit for all).

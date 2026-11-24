@echo off
setlocal
cd /d "%~dp0"
title Serial Flux MCP Inspector
set "MCP_EXE=%~dp0target\release\serialflux-mcp.exe"
where cargo >nul 2>&1
if errorlevel 1 goto :nocargo
where npx >nul 2>&1
if errorlevel 1 goto :nonpx
echo Building latest Serial Flux MCP server...
cargo build --release --bin serialflux-mcp
if errorlevel 1 goto :failed
if not exist "%MCP_EXE%" goto :failed
rem Find four available local ports, preferring Inspector defaults.
for /f "tokens=1-4" %%A in ('powershell -NoProfile -Command "$used=@(); $out=@(); foreach($p in @(6274,6277,6275,6278)){while($true){if($used -contains $p){$p++;continue};$l=[Net.Sockets.TcpListener]::new([Net.IPAddress]::Loopback,$p);try{$l.Start();$out+=$p;$used+=$p;break}catch{$p++}finally{$l.Stop()}}};$out -join ' '"') do (
    set "CLIENT_PORT=%%A"
    set "SERVER_PORT=%%B"
    set "MCP_SANDBOX_PORT=%%C"
    set "MCP_APP_ORIGIN_PORT=%%D"
)
if not defined CLIENT_PORT goto :failed
echo Starting MCP Inspector on http://127.0.0.1:%CLIENT_PORT%
echo Server: "%MCP_EXE%"
echo Click Connect, then Tools and List Tools in Inspector.
echo Press Ctrl+C in this window to stop Inspector.
call npx --yes @modelcontextprotocol/inspector "%MCP_EXE%"
if errorlevel 1 goto :failed
exit /b 0
:nocargo
echo Rust Cargo not found. Install Rust or check PATH.
goto :failed
:nonpx
echo Node.js/npm not found. Install Node.js LTS.
:failed
echo Unable to start Inspector. Check the messages above.
pause
exit /b 1

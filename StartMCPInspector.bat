@echo off
setlocal
cd /d "%~dp0"
set "MCP_EXE=%~dp0target\release\serialflux-mcp.exe"
if not exist "%MCP_EXE%" (
    echo MCP executable not found. Run cargo build --release first.
    pause
    exit /b 1
)
where npx >nul 2>&1
if errorlevel 1 (
    echo Node.js/npm not found. Install Node.js LTS first.
    pause
    exit /b 1
)
call npx @modelcontextprotocol/inspector "%MCP_EXE%"
pause

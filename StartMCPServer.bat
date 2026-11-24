@echo off
setlocal
cd /d "%~dp0"
set "MCP_EXE=%~dp0target\release\serialflux-mcp.exe"
if not exist "%MCP_EXE%" (
    echo MCP executable not found. Run cargo build --release first. 1>&2
    exit /b 1
)
"%MCP_EXE%"
exit /b %ERRORLEVEL%

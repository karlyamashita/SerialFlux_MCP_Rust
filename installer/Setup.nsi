Unicode true
!include "MUI2.nsh"
!include "LogicLib.nsh"
!include "x64.nsh"
Name "Serial Flux MCP Server ${VERSION}"
OutFile "${OUTPUT}"
InstallDir "$PROGRAMFILES64\KTYLabs\SerialFlux MCP Server"
RequestExecutionLevel admin
VIProductVersion "${VERSION}.0"
VIAddVersionKey "ProductName" "Serial Flux MCP Server"
VIAddVersionKey "CompanyName" "KTYLabs"
VIAddVersionKey "LegalCopyright" "Copyright (C) 2026 KTYLabs"
VIAddVersionKey "FileVersion" "${VERSION}"
VIAddVersionKey "FileDescription" "Serial Flux MCP Server Installer"
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_COMPONENTS
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "English"
Var NativeArch
Function .onInit
  SetShellVarContext all
  SetRegView 64
  System::Call 'kernel32::GetCurrentProcess() p.r0'
  System::Call 'kernel32::IsWow64Process2(p r0, *i .r1, *i .r2) i.r3'
  ${If} $3 == 0
    MessageBox MB_ICONSTOP "Unable to detect native architecture."
    Abort
  ${EndIf}
  IntOp $2 $2 & 0xFFFF
  ${If} $2 == 43620
    StrCpy $NativeArch "ARM64"
  ${ElseIf} $2 == 34404
    StrCpy $NativeArch "x64"
  ${Else}
    MessageBox MB_ICONSTOP "Only x64 and ARM64 Windows are supported."
    Abort
  ${EndIf}
  ReadRegStr $0 HKLM "Software\KTYLabs\SerialFluxMCP" "InstallDir"
  ${If} $0 != ""
    StrCpy $INSTDIR $0
  ${EndIf}
FunctionEnd
Section "MCP server and guide" Main
  SectionIn RO
  SetShellVarContext all
  SetRegView 64
  SetOutPath "$INSTDIR"
  SetOverwrite on
  ClearErrors
  ${If} $NativeArch == "ARM64"
    File /oname=serialflux-mcp.exe "..\target\aarch64-pc-windows-msvc\release\serialflux-mcp.exe"
  ${Else}
    File /oname=serialflux-mcp.exe "..\target\x86_64-pc-windows-msvc\release\serialflux-mcp.exe"
  ${EndIf}
  IfErrors 0 +3
    MessageBox MB_ICONSTOP "Cannot install executable. Close MCP clients and retry."
    Abort
  SetOutPath "$INSTDIR\Documentation"
  File "..\Documentation\index.html"
  File "guide.ico"
  FileOpen $0 "$INSTDIR\Documentation\release.js" w
  FileWrite $0 'window.releaseInfo = {version: "${VERSION}", architecture: "$NativeArch"};'
  FileClose $0
  SetOutPath "$INSTDIR\Examples"
  File "..\Examples\python_example.py"
  SetOutPath "$INSTDIR"
  WriteUninstaller "$INSTDIR\Uninstall.exe"
  CreateDirectory "$SMPROGRAMS\Serial Flux MCP Server"
  CreateShortCut "$SMPROGRAMS\Serial Flux MCP Server\MCP Server Guide.lnk" "$INSTDIR\Documentation\index.html" "" "$INSTDIR\Documentation\guide.ico"
  CreateShortCut "$SMPROGRAMS\Serial Flux MCP Server\Uninstall.lnk" "$INSTDIR\Uninstall.exe"
  WriteRegStr HKLM "Software\KTYLabs\SerialFluxMCP" "InstallDir" "$INSTDIR"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\SerialFluxMCP" "DisplayName" "Serial Flux MCP Server"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\SerialFluxMCP" "DisplayVersion" "${VERSION}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\SerialFluxMCP" "Publisher" "KTYLabs"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\SerialFluxMCP" "InstallLocation" "$INSTDIR"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\SerialFluxMCP" "UninstallString" '$\"$INSTDIR\Uninstall.exe$\"'
SectionEnd
Section "Desktop guide shortcut" Desktop
  CreateShortCut "$DESKTOP\Serial Flux MCP Server Guide.lnk" "$INSTDIR\Documentation\index.html" "" "$INSTDIR\Documentation\guide.ico"
SectionEnd
Section "Uninstall"
  SetShellVarContext all
  SetRegView 64
  Delete "$DESKTOP\Serial Flux MCP Server Guide.lnk"
  Delete "$SMPROGRAMS\Serial Flux MCP Server\MCP Server Guide.lnk"
  Delete "$SMPROGRAMS\Serial Flux MCP Server\Uninstall.lnk"
  RMDir "$SMPROGRAMS\Serial Flux MCP Server"
  Delete "$INSTDIR\serialflux-mcp.exe"
  Delete "$INSTDIR\Documentation\index.html"
  Delete "$INSTDIR\Documentation\guide.ico"
  Delete "$INSTDIR\Documentation\release.js"
  Delete "$INSTDIR\Examples\python_example.py"
  Delete "$INSTDIR\Uninstall.exe"
  RMDir "$INSTDIR\Documentation"
  RMDir "$INSTDIR\Examples"
  RMDir "$INSTDIR"
  DeleteRegKey HKLM "Software\KTYLabs\SerialFluxMCP"
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\SerialFluxMCP"
SectionEnd

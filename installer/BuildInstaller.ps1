$ErrorActionPreference = 'Stop'
Set-Location (Split-Path $PSScriptRoot -Parent)
function Run($exe, $arguments) { & $exe @arguments; if ($LASTEXITCODE -ne 0) { throw "$exe failed ($LASTEXITCODE)" } }
try {
    $version = [regex]::Match((Get-Content Cargo.toml -Raw), '(?m)^version\s*=\s*"(\d+\.\d+\.\d+)"').Groups[1].Value
    if (!$version) { throw 'Expected numeric Cargo package version' }
    $nsis = Get-Command makensis.exe -ErrorAction SilentlyContinue
    if ($nsis) { $compiler = $nsis.Source } else {
        $compiler = @(${env:ProgramFiles(x86)}, $env:ProgramFiles) | Where-Object { $_ } | ForEach-Object { Join-Path $_ 'NSIS\makensis.exe' } | Where-Object { Test-Path $_ } | Select-Object -First 1
    }
    if (!$compiler) { throw 'Install NSIS 3.x or add makensis.exe to PATH.' }
    foreach ($arch in @('x86_64-pc-windows-msvc','aarch64-pc-windows-msvc')) {
        Run rustup @('target','add',$arch)
        Run cargo @('build','--locked','--release','--target',$arch,'--target-dir','target')
    }
    $downloads = (New-Object -ComObject Shell.Application).NameSpace('shell:Downloads').Self.Path
    if (!$downloads) { throw 'Downloads folder not found' }
    $output = Join-Path $downloads "SerialFlux_MCP_Server_Setup_$version.exe"
    Run $compiler @("/DVERSION=$version", "/DOUTPUT=$output", 'installer\Setup.nsi')
    $hash = (Get-FileHash -Algorithm SHA256 -LiteralPath $output).Hash
    "$hash  $([IO.Path]::GetFileName($output))" | Set-Content -Encoding ASCII "$output.sha256.txt"
    Write-Host "Created: $output"
    exit 0
} catch { Write-Host $_ -ForegroundColor Red; exit 1 }

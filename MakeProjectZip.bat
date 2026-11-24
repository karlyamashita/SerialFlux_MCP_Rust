@echo off
setlocal
set "PROJECT_ROOT=%~dp0"
powershell.exe -NoProfile -Command "$ErrorActionPreference='Stop'; $root=(Get-Item -LiteralPath $env:PROJECT_ROOT).FullName.TrimEnd('\'); $name=Split-Path $root -Leaf; $stamp=Get-Date -Format 'yyyyMMdd-HHmmss'; $downloads=(New-Object -ComObject Shell.Application).NameSpace('shell:Downloads').Self.Path; if(-not $downloads) { throw 'Downloads folder not found' }; $zip=Join-Path $downloads ($name+'_Source_'+$stamp+'.zip'); Add-Type -AssemblyName System.IO.Compression; Add-Type -AssemblyName System.IO.Compression.FileSystem; $archive=[IO.Compression.ZipFile]::Open($zip,[IO.Compression.ZipArchiveMode]::Create); try { function Add-Folder($dir) { foreach($item in Get-ChildItem -LiteralPath $dir -Force) { if($item.PSIsContainer) { if($item.Name -notin @('target','.git')) { Add-Folder $item.FullName } } elseif($item.Extension -ne '.zip') { $entry=$name+'/'+$item.FullName.Substring($root.Length+1).Replace('\','/'); [void][IO.Compression.ZipFileExtensions]::CreateEntryFromFile($archive,$item.FullName,$entry,[IO.Compression.CompressionLevel]::Optimal) } } }; Add-Folder $root } catch { $archive.Dispose(); Remove-Item -LiteralPath $zip -Force; throw } finally { $archive.Dispose() }; Write-Host ('Created: '+$zip)"
if errorlevel 1 (
    echo ZIP creation failed.
    pause
    exit /b 1
)
echo Done.
pause

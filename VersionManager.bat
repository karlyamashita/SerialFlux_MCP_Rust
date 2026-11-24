@echo off
cd /d "%~dp0"
python VersionManager.py
if errorlevel 1 pause

@echo off
setlocal
title Theasus - Remove Start Menu Shortcuts
echo ========================================================
echo   Theasus Keyboard Control Center - Remove Shortcuts
echo ========================================================
echo.

powershell -NoProfile -ExecutionPolicy Bypass -Command ^
  "$p = [Environment]::GetFolderPath('Programs'); " ^
  "Remove-Item (Join-Path $p 'Theasus.lnk') -ErrorAction SilentlyContinue; " ^
  "Remove-Item (Join-Path $p 'Thesus.lnk') -ErrorAction SilentlyContinue;"

reg delete "HKCU\Software\Microsoft\Windows\CurrentVersion\App Paths\theasus.exe" /f >nul 2>&1
reg delete "HKCU\Software\Microsoft\Windows\CurrentVersion\App Paths\thesus.exe" /f >nul 2>&1

echo [SUCCESS] Removed Start Menu shortcuts and App Paths.
echo.
pause

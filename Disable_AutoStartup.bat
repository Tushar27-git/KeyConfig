@echo off
setlocal
title Theasus - Disable Auto-Startup
echo ========================================================
echo   Theasus Keyboard Control Center - Disable Auto-Startup
echo ========================================================
echo.

echo Removing Theasus from Windows Registry:
echo HKCU\Software\Microsoft\Windows\CurrentVersion\Run
echo.

reg delete "HKCU\Software\Microsoft\Windows\CurrentVersion\Run" /v "Theasus" /f >nul 2>&1

if %ERRORLEVEL% EQU 0 (
    echo [SUCCESS] Theasus auto-startup has been removed.
) else (
    echo [INFO] Auto-startup was not configured or already removed.
)

echo.
pause

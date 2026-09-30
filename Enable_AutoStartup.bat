@echo off
setlocal EnableDelayedExpansion
title Theasus - Enable Auto-Startup
echo ========================================================
echo   Theasus Keyboard Control Center - Enable Auto-Startup
echo ========================================================
echo.

set "EXE_PATH=%~dp0Theasus.exe"
if not exist "!EXE_PATH!" (
    set "EXE_PATH=%~dp0target\release\theasus.exe"
)

if not exist "!EXE_PATH!" (
    echo [ERROR] Theasus executable not found!
    echo Looked for:
    echo   - %~dp0Theasus.exe
    echo   - %~dp0target\release\theasus.exe
    echo.
    echo Please run "cargo build --release" to generate the release binary.
    echo.
    pause
    exit /b 1
)

echo Found executable: "!EXE_PATH!"
echo.
echo Select Startup Mode:
echo   [1] Normal Window (Default)
echo   [2] Minimized to Taskbar (Background Remapping)
echo.
set /p MODE="Enter choice [1 or 2, default=1]: "

if "%MODE%"=="2" (
    set "CMD_ARGS=--autostart --minimized"
    set "MODE_LABEL=Minimized (Background)"
) else (
    set "CMD_ARGS=--autostart"
    set "MODE_LABEL=Normal Window"
)

echo.
echo Registering in Windows Registry:
echo Key:   HKCU\Software\Microsoft\Windows\CurrentVersion\Run
echo Value: Theasus
echo Mode:  !MODE_LABEL!
echo Data:  "!EXE_PATH!" !CMD_ARGS!
echo.

reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Run" /v "Theasus" /t REG_SZ /d "\"!EXE_PATH!\" !CMD_ARGS!" /f >nul 2>&1

if !ERRORLEVEL! EQU 0 (
    echo ========================================================
    echo  [SUCCESS] Auto-startup registered successfully!
    echo  Theasus will now automatically launch on Windows boot,
    echo  restart, and logon.
    echo ========================================================
) else (
    echo [FAILED] Failed to register startup. Error code: !ERRORLEVEL!
)

echo.
pause

@echo off
setlocal EnableDelayedExpansion
title Theasus - Install Start Menu Shortcut
echo ========================================================
echo   Theasus Keyboard Control Center - Start Menu Setup
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

set "ICON_PATH=%~dp0assets\theasus.ico"
if not exist "!ICON_PATH!" (
    set "ICON_PATH=!EXE_PATH!"
)

echo Target: "!EXE_PATH!"
echo Icon:   "!ICON_PATH!"
echo.
echo Installing shortcuts into:
echo   %%APPDATA%%\Microsoft\Windows\Start Menu\Programs\Theasus.lnk
echo   %%APPDATA%%\Microsoft\Windows\Start Menu\Programs\Thesus.lnk
echo.

powershell -NoProfile -ExecutionPolicy Bypass -Command ^
  "$ws = New-Object -ComObject WScript.Shell; " ^
  "$p = [Environment]::GetFolderPath('Programs'); " ^
  "$s1 = $ws.CreateShortcut([System.IO.Path]::Combine($p, 'Theasus.lnk')); " ^
  "$s1.TargetPath = '!EXE_PATH!'; " ^
  "$s1.WorkingDirectory = '%~dp0'; " ^
  "$s1.Description = 'Theasus Keyboard Control Center (Native Remapper)'; " ^
  "$s1.IconLocation = '!ICON_PATH!,0'; " ^
  "$s1.Save(); " ^
  "$s2 = $ws.CreateShortcut([System.IO.Path]::Combine($p, 'Thesus.lnk')); " ^
  "$s2.TargetPath = '!EXE_PATH!'; " ^
  "$s2.WorkingDirectory = '%~dp0'; " ^
  "$s2.Description = 'Thesus Keyboard Control Center'; " ^
  "$s2.IconLocation = '!ICON_PATH!,0'; " ^
  "$s2.Save();"

reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\App Paths\theasus.exe" /ve /t REG_SZ /d "!EXE_PATH!" /f >nul 2>&1
reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\App Paths\theasus.exe" /v "Path" /t REG_SZ /d "%~dp0" /f >nul 2>&1
reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\App Paths\thesus.exe" /ve /t REG_SZ /d "!EXE_PATH!" /f >nul 2>&1
reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\App Paths\thesus.exe" /v "Path" /t REG_SZ /d "%~dp0" /f >nul 2>&1

echo ========================================================
echo  [SUCCESS] Start Menu and Windows Search integration complete!
echo.
echo  You can now press the Windows key and search:
echo    - "Theasus"
echo    - "thesus"
echo.
echo  The application will appear right in the search results!
echo ========================================================
echo.
pause

@echo off
title ZenDesign Studio
cd /d "%~dp0"

echo ========================================================
echo   ZenDesign Studio - Luxury Autonomous Creative Studio
echo ========================================================
echo.

:: Locate zen-design-server executable
set "EXE="
if exist "server\target\release\zen-design-server.exe" (
    set "EXE=server\target\release\zen-design-server.exe"
) else if exist "zen-design-server.exe" (
    set "EXE=zen-design-server.exe"
) else if exist "target\release\zen-design-server.exe" (
    set "EXE=target\release\zen-design-server.exe"
) else (
    echo Building zen-design-server (release mode)...
    cargo build --release --manifest-path server/Cargo.toml
    if exist "server\target\release\zen-design-server.exe" (
        set "EXE=server\target\release\zen-design-server.exe"
    )
)

if "%EXE%"=="" (
    echo [ERROR] Could not find or build zen-design-server.exe.
    pause
    exit /b 1
)

echo Starting ZenDesign daemon...
start "" /b "%EXE%"

:: Allow 1.5s for port binding
timeout /t 2 /nobreak >nul

:: Launch in native Application Window mode (frameless, no address bar)
echo Launching ZenDesign Desktop Window...
where msedge >nul 2>nul
if %errorlevel%==0 (
    start msedge --app=http://127.0.0.1:7456
    exit
)

where chrome >nul 2>nul
if %errorlevel%==0 (
    start chrome --app=http://127.0.0.1:7456
    exit
)

start http://127.0.0.1:7456
exit

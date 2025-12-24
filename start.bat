@echo off
echo ========================================
echo Windows Activity Monitor for Discord
echo ========================================
echo.

REM 检查是否已配置Discord App ID
findstr /C:"YOUR_DISCORD_APP_ID" src\main.rs > nul
if %errorlevel% equ 0 (
    echo [ERROR] Discord Application ID not configured!
    echo.
    echo Please edit src\main.rs and set your Discord App ID:
    echo   const DISCORD_APP_ID: ^&str = "your_app_id_here";
    echo.
    echo Get your App ID at: https://discord.com/developers/applications
    echo.
    pause
    exit /b 1
)

echo Checking if Discord is running...
tasklist /FI "IMAGENAME eq Discord.exe" 2>NUL | find /I /N "Discord.exe">NUL
if "%ERRORLEVEL%"=="1" (
    echo [WARNING] Discord is not running!
    echo Please start Discord first.
    echo.
    pause
)

echo.
echo Starting activity monitor...
echo Press Ctrl+C to stop
echo.

cargo run --release

pause


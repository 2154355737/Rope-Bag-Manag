@echo off
chcp 65001 >nul
echo Starting Rope Manager Backend Service...
echo.

echo Setting environment variables...
set DATABASE_PATH=data.db
set HOST=127.0.0.1
set ALIST_PASSWORD=ahk12378dx
set PORT=15201
set JWT_SECRET=your-secret-key-change-in-production
set UPLOAD_PATH=uploads

echo Database path: %DATABASE_PATH%
echo Upload path: %UPLOAD_PATH%
echo Server address: http://%HOST%:%PORT%

echo.
echo Compiling and starting service...
cargo run

pause 
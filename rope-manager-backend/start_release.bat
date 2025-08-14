@echo off
chcp 65001 >nul
setlocal enabledelayedexpansion

echo 🚀 绳包管理器后端启动
echo ====================
echo.

REM 定义可能的可执行文件路径
set EXE_PATH1=rope-manager-backend.exe
set EXE_PATH2=target\release\rope-manager-backend.exe
set EXE_PATH3=C:\Users\Administrator\Desktop\结绳社区\rope-manager-backend.exe

REM 检查可执行文件
set FOUND_EXE=
if exist "%EXE_PATH1%" (
    set FOUND_EXE=%EXE_PATH1%
) else if exist "%EXE_PATH2%" (
    set FOUND_EXE=%EXE_PATH2%
) else if exist "%EXE_PATH3%" (
    set FOUND_EXE=%EXE_PATH3%
) else (
    echo ❌ 未找到可执行文件
    pause
    exit /b 1
)

echo ✅ 找到: !FOUND_EXE!

REM 设置环境变量
set DATABASE_PATH=data.db
set HOST=127.0.0.1
set ALIST_PASSWORD=ahk12378dx
set PORT=15201
set JWT_SECRET=your-secret-key-change-in-production
set UPLOAD_PATH=uploads

REM 设置日志级别
if "%1"=="debug" (
    set RUST_LOG=debug
) else if "%1"=="trace" (
    set RUST_LOG=trace
) else if "%1"=="warn" (
    set RUST_LOG=warn
) else (
    set RUST_LOG=info
)

REM 创建必要目录
if not exist logs mkdir logs
if not exist uploads mkdir uploads
if not exist backups mkdir backups

echo 🌐 服务地址: http://%HOST%:%PORT%
echo ⚠️  按 Ctrl+C 停止服务
echo.

REM 启动服务
"!FOUND_EXE!" 
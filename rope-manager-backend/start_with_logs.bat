@echo off
echo 🚀 绳包管理器后端服务启动脚本
echo ================================

if "%1"=="" (
    echo 📋 使用方法:
    echo   %0 dev    - 开发模式 (DEBUG级别)
    echo   %0 prod   - 生产模式 (INFO级别)
    echo   %0 quiet  - 安静模式 (WARN级别)
    echo   %0 debug  - 调试模式 (TRACE级别)
    echo.
    set /p mode="请选择运行模式 [dev/prod/quiet/debug]: "
) else (
    set mode=%1
)

if "%mode%"=="dev" (
    echo 🔧 启动开发模式 - DEBUG级别日志
    set RUST_LOG=debug
    goto :start
)

if "%mode%"=="prod" (
    echo 🌐 启动生产模式 - INFO级别日志
    set RUST_LOG=info
    goto :start
)

if "%mode%"=="quiet" (
    echo 🔇 启动安静模式 - WARN级别日志
    set RUST_LOG=warn
    goto :start
)

if "%mode%"=="debug" (
    echo 🐛 启动调试模式 - TRACE级别日志
    set RUST_LOG=trace
    goto :start
)

echo ❌ 无效的模式: %mode%
echo 支持的模式: dev, prod, quiet, debug
pause
exit /b 1

:start
echo.
echo 📊 当前日志级别: %RUST_LOG%
echo 🌐 服务地址: http://127.0.0.1:15201
echo 📋 API文档: http://127.0.0.1:15201/swagger-ui/
echo.
echo ⚠️  按 Ctrl+C 停止服务
echo.

REM 创建logs目录
if not exist logs mkdir logs

REM 启动服务并记录日志
cargo run 2>&1 | tee logs/server-%date:~0,4%%date:~5,2%%date:~8,2%.log

pause 
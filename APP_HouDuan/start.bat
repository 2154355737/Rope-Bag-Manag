@echo off
chcp 65001 >nul
echo ========================================
echo 结绳社区后端服务启动脚本
echo ========================================
echo.

:: 检查是否在正确的目录
if not exist "Cargo.toml" (
    echo 错误：未找到 Cargo.toml 文件
    echo 请确保在 APP_HouDuan 目录下运行此脚本
    pause
    exit /b 1
)

:: 显示当前目录
echo 当前目录: %CD%
echo.

:: 检查 Rust 环境
echo 检查 Rust 环境...
cargo --version >nul 2>&1
if errorlevel 1 (
    echo 错误：未找到 cargo 命令
    echo 请确保已安装 Rust 开发环境
    echo 下载地址: https://rustup.rs/
    pause
    exit /b 1
)

:: 显示 Rust 版本信息
echo Rust 环境检查通过
cargo --version
echo.

:: 检查数据库目录
if not exist "data" (
    echo 创建数据库目录...
    mkdir data
)

:: 检查上传目录
if not exist "uploads" (
    echo 创建上传目录...
    mkdir uploads
)

:: 检查临时目录
if not exist "temp" (
    echo 创建临时目录...
    mkdir temp
)

:: 检查日志目录
if not exist "logs" (
    echo 创建日志目录...
    mkdir logs
)

echo 目录检查完成
echo.

:: 提示启动信息
echo 正在启动后端服务...
echo 服务地址: http://127.0.0.1:15201
echo 按 Ctrl+C 停止服务
echo.
echo ========================================
echo.

:: 启动服务
cargo run

:: 如果服务异常退出，暂停以便查看错误信息
if errorlevel 1 (
    echo.
    echo ========================================
    echo 服务启动失败，请检查上述错误信息
    echo ========================================
    pause
) 
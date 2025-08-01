@echo off
chcp 65001 >nul

echo ===============================================
echo           自动化停止 Nginx 服务
echo ===============================================
echo.

cd /d "%~dp0"

:: 检查是否在运行
if not exist "logs\nginx.pid" (
    echo [信息] Nginx 未运行
    exit /b 0
)

echo [停止] 正在停止 Nginx...

:: 优雅停止
nginx.exe -s quit

:: 等待停止完成
timeout /t 3 >nul

:: 检查是否已停止
if not exist "logs\nginx.pid" (
    echo [成功] ✓ Nginx 已停止
    exit /b 0
) else (
    echo [强制] 正在强制停止 Nginx...
    taskkill /f /im nginx.exe >nul 2>&1
    if exist "logs\nginx.pid" del "logs\nginx.pid" >nul 2>&1
    echo [成功] ✓ Nginx 已强制停止
    exit /b 0
) 
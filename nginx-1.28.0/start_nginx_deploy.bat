@echo off
REM 专门用于自动化部署的Nginx启动脚本
REM 不进行端口检查，直接启动

cd /d "%~dp0"

echo [部署] 启动 Nginx 服务...

REM 测试配置文件
nginx.exe -t >nul 2>&1
if errorlevel 1 (
    echo [错误] 配置文件语法错误
    exit /b 1
)

REM 直接启动Nginx，不检查端口占用
start /b "" nginx.exe

REM 等待启动
timeout /t 5 >nul

REM 简单检查是否启动成功
if exist "logs\nginx.pid" (
    echo [成功] Nginx 启动成功
    exit /b 0
) else (
    echo [警告] 无法确认 Nginx 启动状态
    exit /b 0
) 
@echo off
chcp 65001 >nul

echo ===============================================
echo           快速启动 Nginx 服务
echo ===============================================
echo.

cd /d "%~dp0"

:: 检查是否已运行
if exist "logs\nginx.pid" (
    echo [信息] Nginx 已经在运行中
    echo [访问] http://localhost
    pause
    exit /b 0
)

:: 测试配置文件
echo [检查] 正在测试配置文件...
nginx.exe -t
if errorlevel 1 (
    echo [错误] 配置文件有误，无法启动
    echo [建议] 请检查 conf\nginx.conf 文件
    pause
    exit /b 1
)
echo [成功] ✓ 配置文件语法正确

:: 检查端口占用
netstat -an | find ":80 " | find "LISTENING" >nul
if not errorlevel 1 (
    echo [错误] 端口 80 已被占用，无法启动
    echo [占用] 以下程序正在使用端口 80:
    netstat -ano | find ":80 " | find "LISTENING"
    echo [建议] 请关闭占用端口的程序或使用完整管理脚本
    pause
    exit /b 1
)

:: 启动 Nginx
echo [启动] 正在启动 Nginx...
start /b "" nginx.exe

:: 等待启动完成
timeout /t 3 >nul

:: 检查启动结果
if exist "logs\nginx.pid" (
    echo [成功] ✓ Nginx 启动成功！
    echo [访问] http://localhost
    echo [健康] http://localhost/health
    echo [管理] 双击 nginx_manager.bat 进行更多操作
    echo.
    echo [提示] 如需部署前端应用，请将构建文件放入 html 目录
) else (
    echo [失败] ✗ Nginx 启动失败
    echo [日志] 查看错误信息:
    if exist "logs\error.log" (
        echo --- 错误日志 (最新5行) ---
        powershell "Get-Content 'logs\error.log' -Tail 5"
    )
    echo [建议] 请运行 nginx_manager.bat 进行详细诊断
)

echo.
pause 
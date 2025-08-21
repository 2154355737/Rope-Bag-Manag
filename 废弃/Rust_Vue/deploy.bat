@echo off
echo 🚀 开始部署前端文件到nginx...

REM 检查dist目录是否存在
if not exist "dist" (
    echo ❌ dist目录不存在，请先运行 pnpm build
    pause
    exit /b 1
)

REM 设置nginx html目录路径
set NGINX_HTML_DIR=D:\vue\zhiyuanshequ\rust\nginx-1.28.0\html

REM 检查nginx目录是否存在
if not exist "%NGINX_HTML_DIR%" (
    echo ❌ nginx html目录不存在: %NGINX_HTML_DIR%
    echo 正在创建目录...
    mkdir "%NGINX_HTML_DIR%"
)

echo 📁 目标目录: %NGINX_HTML_DIR%

REM 备份现有文件（可选）
if exist "%NGINX_HTML_DIR%\index.html" (
    echo 📦 备份现有文件...
    if not exist "%NGINX_HTML_DIR%\backup" mkdir "%NGINX_HTML_DIR%\backup"
    xcopy "%NGINX_HTML_DIR%\*" "%NGINX_HTML_DIR%\backup\" /E /I /Y >nul 2>&1
)

REM 清理目标目录（保留backup文件夹）
echo 🧹 清理目标目录...
for /f "delims=" %%i in ('dir /b /ad "%NGINX_HTML_DIR%"') do (
    if not "%%i"=="backup" (
        rmdir /s /q "%NGINX_HTML_DIR%\%%i" 2>nul
    )
)
del /q "%NGINX_HTML_DIR%\*" 2>nul

REM 复制新文件
echo 📋 复制构建文件...
xcopy "dist\*" "%NGINX_HTML_DIR%\" /E /I /Y

if %errorlevel% equ 0 (
    echo ✅ 部署成功！
    echo 🌐 访问地址: http://39.105.113.219
    echo 📝 请确保nginx服务正在运行
) else (
    echo ❌ 部署失败，错误代码: %errorlevel%
)

echo.
echo 🔄 重启nginx服务...
taskkill /f /im nginx.exe >nul 2>&1
timeout /t 2 >nul
start "" "D:\vue\zhiyuanshequ\rust\nginx-1.28.0\nginx.exe"

echo ✅ 部署完成！
echo 📋 配置说明:
echo    • 前端文件: %NGINX_HTML_DIR%
echo    • API代理: /api/ -> http://127.0.0.1:15201
echo    • 文件代理: /uploads/ -> http://127.0.0.1:15201
echo    • 访问地址: http://39.105.113.219
pause 
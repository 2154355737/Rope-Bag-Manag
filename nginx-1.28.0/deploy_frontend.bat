@echo off
chcp 65001 >nul

echo ===============================================
echo           前端应用部署脚本
echo ===============================================
echo.

cd /d "%~dp0"

:: 前端构建文件路径
set FRONTEND_DIST=..\Rust_Vue\dist
set NGINX_HTML=html

:: 检查前端构建文件是否存在
if not exist "%FRONTEND_DIST%" (
    echo [错误] 前端构建目录不存在: %FRONTEND_DIST%
    echo [建议] 请先构建前端应用:
    echo         cd ..\Rust_Vue
    echo         npm run build
    pause
    exit /b 1
)

if not exist "%FRONTEND_DIST%\index.html" (
    echo [错误] 前端构建文件不完整，缺少 index.html
    echo [建议] 请重新构建前端应用:
    echo         cd ..\Rust_Vue
    echo         npm run build
    pause
    exit /b 1
)

:: 备份现有文件
if exist "%NGINX_HTML%\index.html" (
    echo [备份] 正在备份现有文件...
    if not exist "backup" mkdir backup
    set BACKUP_NAME=backup\html_backup_%date:~0,4%%date:~5,2%%date:~8,2%_%time:~0,2%%time:~3,2%%time:~6,2%
    xcopy "%NGINX_HTML%\*" "!BACKUP_NAME!\" /s /e /i >nul
    echo [成功] ✓ 备份完成: !BACKUP_NAME!
)

:: 清理旧文件（保留50x.html错误页面）
echo [清理] 正在清理旧的前端文件...
for /f %%i in ('dir /b "%NGINX_HTML%\*" 2^>nul ^| findstr /v "50x.html"') do (
    if exist "%NGINX_HTML%\%%i" (
        if "%%i" neq "50x.html" (
            if exist "%NGINX_HTML%\%%i\*" (
                rmdir /s /q "%NGINX_HTML%\%%i" 2>nul
            ) else (
                del /q "%NGINX_HTML%\%%i" 2>nul
            )
        )
    )
)

:: 复制新的构建文件
echo [部署] 正在复制前端构建文件...
xcopy "%FRONTEND_DIST%\*" "%NGINX_HTML%\" /s /e /y >nul
if errorlevel 1 (
    echo [错误] 文件复制失败
    pause
    exit /b 1
)

echo [成功] ✓ 前端应用部署完成！
echo.

:: 检查nginx状态
if exist "logs\nginx.pid" (
    echo [状态] Nginx 正在运行中
    echo [重载] 正在重载配置...
    nginx.exe -s reload
    if errorlevel 0 (
        echo [成功] ✓ 配置重载完成
    ) else (
        echo [警告] 配置重载失败，请手动重启nginx
    )
) else (
    echo [状态] Nginx 未运行
    echo [建议] 请运行 start_nginx.bat 启动服务
)

echo.
echo [访问] http://localhost
echo [健康] http://localhost/health
echo.

:: 显示部署信息
echo [信息] 部署详情:
echo         源目录: %FRONTEND_DIST%
echo         目标目录: %NGINX_HTML%
echo         部署时间: %date% %time%

pause 
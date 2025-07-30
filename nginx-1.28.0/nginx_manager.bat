@echo off
chcp 65001 >nul
setlocal enabledelayedexpansion

:: ===========================================
:: Nginx 管理脚本
:: 支持启动、停止、重启、重载、状态检查等操作
:: ===========================================

set NGINX_DIR=%~dp0
set NGINX_EXE=%NGINX_DIR%nginx.exe
set NGINX_CONF=%NGINX_DIR%conf\nginx.conf
set PID_FILE=%NGINX_DIR%logs\nginx.pid

:: 颜色设置
set RED=[91m
set GREEN=[92m
set YELLOW=[93m
set BLUE=[94m
set RESET=[0m

echo %BLUE%===============================================%RESET%
echo %BLUE%           Nginx 服务管理脚本%RESET%
echo %BLUE%===============================================%RESET%
echo.

if "%1"=="" goto MENU

:: 直接执行命令参数
if /i "%1"=="start" goto START
if /i "%1"=="stop" goto STOP
if /i "%1"=="restart" goto RESTART
if /i "%1"=="reload" goto RELOAD
if /i "%1"=="status" goto STATUS
if /i "%1"=="test" goto TEST
if /i "%1"=="logs" goto LOGS
goto HELP

:MENU
echo %YELLOW%请选择操作:%RESET%
echo %GREEN%[1]%RESET% 启动 Nginx
echo %GREEN%[2]%RESET% 停止 Nginx
echo %GREEN%[3]%RESET% 重启 Nginx
echo %GREEN%[4]%RESET% 重载配置
echo %GREEN%[5]%RESET% 检查状态
echo %GREEN%[6]%RESET% 测试配置
echo %GREEN%[7]%RESET% 查看日志
echo %GREEN%[8]%RESET% 清理日志
echo %GREEN%[0]%RESET% 退出
echo.
set /p choice=%YELLOW%请输入选择 (0-8): %RESET%

if "%choice%"=="1" goto START
if "%choice%"=="2" goto STOP
if "%choice%"=="3" goto RESTART
if "%choice%"=="4" goto RELOAD
if "%choice%"=="5" goto STATUS
if "%choice%"=="6" goto TEST
if "%choice%"=="7" goto LOGS
if "%choice%"=="8" goto CLEAN_LOGS
if "%choice%"=="0" goto EXIT
echo %RED%无效选择，请重新输入%RESET%
echo.
goto MENU

:START
echo %YELLOW%正在启动 Nginx...%RESET%
if exist "%PID_FILE%" (
    echo %RED%Nginx 已经在运行中%RESET%
    goto STATUS
)

:: 检查端口占用
netstat -an | find ":80 " | find "LISTENING" >nul
if not errorlevel 1 (
    echo %RED%端口 80 已被占用，请检查其他程序%RESET%
    netstat -ano | find ":80 " | find "LISTENING"
    pause
    goto MENU
)

start /b "" "%NGINX_EXE%" -c "%NGINX_CONF%"
timeout /t 2 >nul

if exist "%PID_FILE%" (
    echo %GREEN%✓ Nginx 启动成功%RESET%
    echo %BLUE%访问地址: http://localhost%RESET%
    echo %BLUE%健康检查: http://localhost/health%RESET%
) else (
    echo %RED%✗ Nginx 启动失败，请检查配置文件%RESET%
    goto TEST
)
goto MENU_OR_EXIT

:STOP
echo %YELLOW%正在停止 Nginx...%RESET%
if not exist "%PID_FILE%" (
    echo %RED%Nginx 未运行%RESET%
    goto MENU_OR_EXIT
)

"%NGINX_EXE%" -s quit
timeout /t 3 >nul

if not exist "%PID_FILE%" (
    echo %GREEN%✓ Nginx 已停止%RESET%
) else (
    echo %YELLOW%强制停止 Nginx 进程...%RESET%
    taskkill /f /im nginx.exe >nul 2>&1
    if exist "%PID_FILE%" del "%PID_FILE%" >nul 2>&1
    echo %GREEN%✓ Nginx 已强制停止%RESET%
)
goto MENU_OR_EXIT

:RESTART
echo %YELLOW%正在重启 Nginx...%RESET%
call :STOP_SILENT
timeout /t 1 >nul
call :START_SILENT
goto MENU_OR_EXIT

:RELOAD
echo %YELLOW%正在重载 Nginx 配置...%RESET%
if not exist "%PID_FILE%" (
    echo %RED%Nginx 未运行，无法重载配置%RESET%
    goto MENU_OR_EXIT
)

"%NGINX_EXE%" -s reload
if errorlevel 1 (
    echo %RED%✗ 配置重载失败%RESET%
    goto TEST
) else (
    echo %GREEN%✓ 配置重载成功%RESET%
)
goto MENU_OR_EXIT

:STATUS
echo %YELLOW%Nginx 服务状态:%RESET%
if exist "%PID_FILE%" (
    set /p pid=<"%PID_FILE%"
    tasklist /fi "PID eq !pid!" | find "nginx.exe" >nul
    if not errorlevel 1 (
        echo %GREEN%✓ Nginx 正在运行 (PID: !pid!)%RESET%
        
        :: 检查端口监听
        netstat -an | find ":80 " | find "LISTENING" >nul
        if not errorlevel 1 (
            echo %GREEN%✓ 端口 80 正在监听%RESET%
        ) else (
            echo %RED%✗ 端口 80 未监听%RESET%
        )
        
        :: 健康检查
        curl -s http://localhost/health >nul 2>&1
        if not errorlevel 1 (
            echo %GREEN%✓ 健康检查通过%RESET%
        ) else (
            echo %YELLOW%⚠ 健康检查失败%RESET%
        )
    ) else (
        echo %RED%✗ PID 文件存在但进程不存在%RESET%
        del "%PID_FILE%" >nul 2>&1
    )
) else (
    echo %RED%✗ Nginx 未运行%RESET%
)
goto MENU_OR_EXIT

:TEST
echo %YELLOW%正在测试 Nginx 配置...%RESET%
"%NGINX_EXE%" -t -c "%NGINX_CONF%"
if errorlevel 1 (
    echo %RED%✗ 配置文件有错误%RESET%
) else (
    echo %GREEN%✓ 配置文件语法正确%RESET%
)
goto MENU_OR_EXIT

:LOGS
echo %YELLOW%显示最新日志...%RESET%
if exist "%NGINX_DIR%logs\error.log" (
    echo %BLUE%=== 错误日志 (最新20行) ===%RESET%
    powershell "Get-Content '%NGINX_DIR%logs\error.log' -Tail 20"
    echo.
)
if exist "%NGINX_DIR%logs\access.log" (
    echo %BLUE%=== 访问日志 (最新10行) ===%RESET%
    powershell "Get-Content '%NGINX_DIR%logs\access.log' -Tail 10"
)
goto MENU_OR_EXIT

:CLEAN_LOGS
echo %YELLOW%正在清理日志文件...%RESET%
if exist "%NGINX_DIR%logs\error.log" (
    echo. > "%NGINX_DIR%logs\error.log"
    echo %GREEN%✓ 错误日志已清理%RESET%
)
if exist "%NGINX_DIR%logs\access.log" (
    echo. > "%NGINX_DIR%logs\access.log"
    echo %GREEN%✓ 访问日志已清理%RESET%
)
goto MENU_OR_EXIT

:STOP_SILENT
if exist "%PID_FILE%" (
    "%NGINX_EXE%" -s quit >nul 2>&1
    timeout /t 2 >nul
    if exist "%PID_FILE%" (
        taskkill /f /im nginx.exe >nul 2>&1
        if exist "%PID_FILE%" del "%PID_FILE%" >nul 2>&1
    )
)
goto :eof

:START_SILENT
if not exist "%PID_FILE%" (
    start /b "" "%NGINX_EXE%" -c "%NGINX_CONF%" >nul 2>&1
    timeout /t 2 >nul
)
goto :eof

:HELP
echo %BLUE%使用方法:%RESET%
echo %YELLOW%  %~nx0 [命令]%RESET%
echo.
echo %BLUE%可用命令:%RESET%
echo   start    - 启动 Nginx
echo   stop     - 停止 Nginx  
echo   restart  - 重启 Nginx
echo   reload   - 重载配置
echo   status   - 检查状态
echo   test     - 测试配置
echo   logs     - 查看日志
echo.
echo %BLUE%示例:%RESET%
echo   %~nx0 start
echo   %~nx0 restart
goto EXIT

:MENU_OR_EXIT
if "%1"=="" (
    echo.
    pause
    goto MENU
)
goto EXIT

:EXIT
echo.
echo %BLUE%感谢使用 Nginx 管理脚本！%RESET%
pause
exit /b 0 
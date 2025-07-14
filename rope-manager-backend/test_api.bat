@echo off
chcp 65001 >nul
setlocal enabledelayedexpansion

echo ==========================================
echo 绳包管理器后端API测试脚本
echo ==========================================

REM 从配置文件读取端口
set PORT=8080
if exist config.toml (
    for /f "tokens=2 delims==" %%i in ('findstr "port" config.toml') do (
        set PORT=%%i
        set PORT=!PORT: =!
    )
)

echo 服务器地址: http://127.0.0.1:%PORT%
echo.

echo 1. 测试健康检查...
curl -X GET http://127.0.0.1:%PORT%/health
if %ERRORLEVEL% EQU 0 (
    echo ✅ 健康检查成功
) else (
    echo ❌ 健康检查失败
)

echo.
echo 2. 测试服务器状态...
curl -X GET http://127.0.0.1:%PORT%/api/v1/packages
if %ERRORLEVEL% EQU 0 (
    echo ✅ 服务器状态正常
) else (
    echo ❌ 服务器状态检查失败
)

echo.
echo 3. 测试用户注册...
curl -X POST http://127.0.0.1:%PORT%/api/v1/auth/register ^
  -H "Content-Type: application/json" ^
  -d "{\"username\":\"testuser\",\"password\":\"testpass123\",\"email\":\"test@example.com\"}"
if %ERRORLEVEL% EQU 0 (
    echo ✅ 用户注册成功
) else (
    echo ❌ 用户注册失败
)

echo.
echo 4. 测试用户登录...
curl -X POST http://127.0.0.1:%PORT%/api/v1/auth/login ^
  -H "Content-Type: application/json" ^
  -d "{\"username\":\"testuser\",\"password\":\"testpass123\"}"
if %ERRORLEVEL% EQU 0 (
    echo ✅ 用户登录成功
) else (
    echo ❌ 用户登录失败
)

echo.
echo 5. 测试获取用户信息...
curl -X GET http://127.0.0.1:%PORT%/api/v1/auth/user-info
if %ERRORLEVEL% EQU 0 (
    echo ✅ 获取用户信息成功
) else (
    echo ❌ 获取用户信息失败
)

echo.
echo ==========================================
echo 🎉 API测试完成！
echo ==========================================
echo.
echo 测试结果说明:
echo - ✅ 表示测试成功
echo - ❌ 表示测试失败
echo.
pause 
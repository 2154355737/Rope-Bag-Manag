@echo off
echo 启动资源社区APP开发环境...
echo.

REM 检查是否安装了npm
where npm >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
  echo 错误：未找到npm，请确保已安装Node.js
  pause
  exit /b 1
)

REM 安装依赖
echo 安装前端依赖...
call npm install
if %ERRORLEVEL% NEQ 0 (
  echo 错误：依赖安装失败
  pause
  exit /b 1
)

REM 启动后端服务
echo 启动后端服务...
cd ..
start cmd /k "cd rope-manager-backend && start.bat"

REM 等待后端启动
echo 等待后端服务启动...
timeout /t 3 /nobreak >nul

REM 启动前端服务
echo 启动前端开发服务器...
cd app
start cmd /k "npm run dev"

echo.
echo 开发环境启动成功！
echo 前端页面: http://localhost:5173
echo 后端接口: http://localhost:15201
echo.

pause 
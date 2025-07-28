@echo off
chcp 65001 >nul
setlocal enabledelayedexpansion

echo ==========================================
echo 绳包管理器全栈启动脚本
echo ==========================================

echo.
echo 1. 检查后端配置...
if not exist "rope-manager-backend\config.toml" (
    echo 创建默认配置文件...
    cd rope-manager-backend
    powershell -ExecutionPolicy Bypass -File config_manager.ps1 new
    cd ..
)

echo.
echo 2. 启动后端服务...
start "后端服务" cmd /k "cd rope-manager-backend && cargo run --bin rope-manager-backend"

echo.
echo 3. 等待后端服务启动...
timeout /t 5 /nobreak >nul

echo.
echo 4. 检查前端依赖...
if not exist "Rust_Vue\node_modules" (
    echo 安装前端依赖...
    cd Rust_Vue
    npm install
    cd ..
)

echo.
echo 5. 启动前端服务...
start "前端服务" cmd /k "cd Rust_Vue && npm run dev"

echo.
echo 6. 等待前端服务启动...
timeout /t 3 /nobreak >nul

echo.
echo ==========================================
echo 🎉 全栈服务启动完成！
echo ==========================================
echo.
echo 后端服务: http://127.0.0.1:15201
echo 前端服务: http://localhost:5173
echo.
echo 测试API: powershell -ExecutionPolicy Bypass -File rope-manager-backend\test_api.ps1
echo.
echo 按任意键退出...
pause 
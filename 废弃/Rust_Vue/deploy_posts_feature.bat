@echo off
echo ===========================================
echo 部署社区帖子功能
echo ===========================================
echo.

echo [1/4] 检查环境...
if not exist "package.json" (
    echo 错误：找不到 package.json，请确保在前端项目根目录执行此脚本
    pause
    exit /b 1
)

if not exist "../rope-manager-backend/Cargo.toml" (
    echo 错误：找不到后端项目，请检查目录结构
    pause
    exit /b 1
)

echo [2/4] 构建前端项目...
call npm run build
if errorlevel 1 (
    echo 前端构建失败，请检查错误信息
    pause
    exit /b 1
)

echo [3/4] 复制前端文件到后端...
xcopy /E /Y /I "dist\*" "..\rope-manager-backend\html\"
if errorlevel 1 (
    echo 文件复制失败
    pause
    exit /b 1
)

echo [4/4] 启动后端服务...
cd ..\rope-manager-backend
echo 正在启动 Rust 后端服务...
cargo run

echo.
echo ===========================================
echo 部署完成！
echo 浏览器访问: http://127.0.0.1:15201
echo 管理员入口: http://127.0.0.1:15201/admin
echo 帖子列表: http://127.0.0.1:15201/posts
echo ===========================================
pause 
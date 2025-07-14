@echo off
echo 启动绳包管理器后端服务...
echo.

REM 设置环境变量
set HOST=127.0.0.1
set PORT=8080
set DATABASE_URL=data/rope_manager.db
set JWT_SECRET=rope-manager-secret-key-2024
set UPLOAD_PATH=uploads
set MAX_FILE_SIZE=10485760

REM 创建必要的目录
if not exist "data" mkdir data
if not exist "uploads" mkdir uploads
if not exist "uploads\packages" mkdir uploads\packages
if not exist "uploads\avatars" mkdir uploads\avatars
if not exist "uploads\covers" mkdir uploads\covers
if not exist "uploads\backups" mkdir uploads\backups

echo 环境变量已设置
echo 数据库路径: %DATABASE_URL%
echo 上传路径: %UPLOAD_PATH%
echo 服务器地址: http://%HOST%:%PORT%
echo.

REM 运行项目
cargo run

pause 
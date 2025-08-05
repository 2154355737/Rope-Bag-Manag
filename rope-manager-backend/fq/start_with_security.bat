@echo off
echo ========================================
echo 启动绳包管理器后端服务（含防刷量系统）
echo ========================================

echo.
echo 设置环境变量...
set DATABASE_PATH=data.db
set UPLOAD_PATH=uploads
set SERVER_ADDRESS=http://127.0.0.1:15201

echo 数据库路径: %DATABASE_PATH%
echo 上传路径: %UPLOAD_PATH%
echo 服务器地址: %SERVER_ADDRESS%

echo.
echo 编译并启动服务...
echo.

REM 检查是否存在数据库文件
if not exist "data.db" (
    echo 警告: 数据库文件不存在，将自动创建
)

REM 创建必要的目录
if not exist "uploads" mkdir uploads
if not exist "logs" mkdir logs

REM 编译并运行
cargo run

echo.
echo 服务已启动！
echo 防刷量系统已启用，包含以下功能：
echo - 用户级别下载频率限制
echo - IP级别下载频率限制  
echo - 资源级别下载频率限制
echo - 可疑模式检测
echo - 统计异常检测
echo - 实时监控和告警
echo.
echo 管理API:
echo - GET /api/v1/download-security/stats     # 获取安全统计
echo - GET /api/v1/download-security/config    # 获取安全配置
echo - PUT /api/v1/download-security/config    # 更新安全配置
echo - GET /api/v1/download-security/anomalies # 获取异常记录
echo.
echo 测试脚本: test_download_security.rs
echo 使用文档: README_DOWNLOAD_SECURITY.md
echo.
pause 
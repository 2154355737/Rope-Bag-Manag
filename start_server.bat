@echo off
chcp 65001 >nul
REM 下面的中文要用 UTF-8 无 BOM 保存，但 cmd 可能还是乱码
echo 启动绳包管理器服务器...
echo.

REM 检查logs目录是否存在
if not exist "logs" (
    echo 创建logs目录...
    mkdir logs
)

REM 检查配置文件是否存在
if not exist "data\config.json" (
    echo 配置文件不存在，将使用默认配置...
)

echo 编译并启动服务器...
cargo run

echo.
echo 服务器已停止。
pause 
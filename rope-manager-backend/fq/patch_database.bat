@echo off
chcp 65001 >nul
title 绳包管理器 - 数据库修补工具

echo.
echo ========================================
echo  🚀 绳包管理器 - 数据库修补工具
echo ========================================
echo.

REM 检查是否存在数据库文件
if not exist "data.db" (
    echo ❌ 错误: 未找到数据库文件 data.db
    echo 请确保在正确的目录下运行此脚本
    pause
    exit /b 1
)

REM 检查是否存在修补脚本
if not exist "sql\patch_database_settings.sql" (
    echo ❌ 错误: 未找到修补脚本 sql\patch_database_settings.sql
    pause
    exit /b 1
)

echo 📋 可用操作:
echo 1. 执行数据库修补 (默认)
echo 2. 查看修补状态
echo 3. 回滚到备份
echo 4. 退出
echo.

set /p choice="请选择操作 (1-4): "

if "%choice%"=="1" goto patch
if "%choice%"=="2" goto status
if "%choice%"=="3" goto rollback
if "%choice%"=="4" goto exit
if "%choice%"=="" goto patch

:patch
echo.
echo 🔧 准备执行数据库修补...
echo ⚠️  注意: 修补前会自动创建数据库备份
echo.
set /p confirm="确认继续? (y/N): "
if /i not "%confirm%"=="y" goto exit

echo.
echo 🏗️  编译修补工具...
cargo build --bin patch_database --release
if errorlevel 1 (
    echo ❌ 编译失败，请检查代码
    pause
    exit /b 1
)

echo.
echo 🚀 执行数据库修补...
target\release\patch_database.exe data.db
if errorlevel 1 (
    echo.
    echo ❌ 修补失败！
    echo 💡 提示: 您可以使用选项3回滚到备份版本
) else (
    echo.
    echo ✅ 修补成功完成！
    echo 🎉 现在您可以使用新的配置管理功能了
)
goto end

:status
echo.
echo 📊 查看修补状态...
cargo build --bin patch_database --release
if errorlevel 1 (
    echo ❌ 编译失败，请检查代码
    pause
    exit /b 1
)
target\release\patch_database.exe data.db status
goto end

:rollback
echo.
echo 🔄 数据库回滚操作
echo.
echo 📁 可用的备份文件:
dir /b data.db.backup_* 2>nul
if errorlevel 1 (
    echo ❌ 未找到备份文件
    goto end
)
echo.
set /p backup_file="请输入要回滚的备份文件名: "
if "%backup_file%"=="" (
    echo ❌ 未指定备份文件
    goto end
)

echo.
echo ⚠️  警告: 回滚操作将覆盖当前数据库
set /p confirm="确认回滚到 %backup_file%? (y/N): "
if /i not "%confirm%"=="y" goto end

cargo build --bin patch_database --release
if errorlevel 1 (
    echo ❌ 编译失败，请检查代码
    pause
    exit /b 1
)
target\release\patch_database.exe data.db rollback "%backup_file%"
goto end

:exit
echo 👋 退出修补工具
exit /b 0

:end
echo.
echo 按任意键继续...
pause >nul 
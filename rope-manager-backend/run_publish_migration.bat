@echo off
echo 🔧 发布页面数据库迁移工具
echo.

echo 📦 编译迁移工具...
rustc --edition 2021 -L target/release/deps apply_publish_migration.rs -o apply_publish_migration.exe --extern anyhow=target/release/deps/libanyhow-*.rlib --extern rusqlite=target/release/deps/librusqlite-*.rlib 2>nul

if not exist "apply_publish_migration.exe" (
    echo ❌ 编译失败，尝试简单编译...
    rustc apply_publish_migration.rs -o apply_publish_migration.exe 2>nul
)

if not exist "apply_publish_migration.exe" (
    echo ❌ 编译失败，使用cargo运行...
    echo.
    echo [dependencies] > temp_migration_cargo.toml
    echo anyhow = "1.0" >> temp_migration_cargo.toml  
    echo rusqlite = "0.29" >> temp_migration_cargo.toml
    echo.
    echo 请手动执行以下命令:
    echo cargo run --bin apply_publish_migration
    pause
    exit /b 1
)

echo ✅ 编译成功！
echo.

echo 🚀 执行数据库迁移...
apply_publish_migration.exe

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ✅ 迁移执行成功！
) else (
    echo.
    echo ❌ 迁移执行失败！
)

echo.
echo 🧹 清理临时文件...
if exist "apply_publish_migration.exe" del apply_publish_migration.exe
if exist "temp_migration_cargo.toml" del temp_migration_cargo.toml

echo.
pause 
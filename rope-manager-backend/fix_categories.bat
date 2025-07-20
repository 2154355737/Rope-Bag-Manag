@echo off
echo 修复分类表结构...

set dbPath=data.db
set sqlPath=sql\fix_categories.sql

echo .read %sqlPath% | sqlite3 %dbPath%

if %ERRORLEVEL% EQU 0 (
    echo 分类表修复成功！
) else (
    echo 分类表修复失败！
    exit /b 1
)

pause 
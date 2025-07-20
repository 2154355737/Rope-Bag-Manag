@echo off
echo 修复系统表结构...

set dbPath=data.db
set sqlPath=sql\fix_system_tables.sql

echo .read %sqlPath% | sqlite3 %dbPath%

if %ERRORLEVEL% EQU 0 (
    echo 系统表修复成功！
) else (
    echo 系统表修复失败！
    exit /b 1
)

pause 
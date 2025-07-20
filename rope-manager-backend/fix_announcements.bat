@echo off
echo 修复公告表结构...

set dbPath=data.db
set sqlPath=sql\fix_announcements.sql

echo .read %sqlPath% | sqlite3 %dbPath%

if %ERRORLEVEL% EQU 0 (
    echo 公告表修复成功！
) else (
    echo 公告表修复失败！
    exit /b 1
)

pause 
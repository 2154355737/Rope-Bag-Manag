@echo off
echo 迁移公告表...

set dbPath=data.db
set sqlPath=sql\migrate_announcements.sql

if not exist %dbPath% (
    echo 数据库文件不存在！请先创建数据库。
    goto :error
)

if not exist %sqlPath% (
    echo SQL迁移文件不存在！
    goto :error
)

:: 执行SQL迁移脚本
sqlite3 %dbPath% < %sqlPath%

if %ERRORLEVEL% EQU 0 (
    echo 公告表迁移成功！
) else (
    echo 公告表迁移失败！
    goto :error
)

echo 按任意键继续...
pause > nul
goto :eof

:error
pause
exit /b 1 
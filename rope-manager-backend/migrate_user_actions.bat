@echo off
echo 执行用户行为记录表迁移...

REM 执行SQL迁移脚本
type .\sql\migrate_user_actions.sql | sqlite3 data.db

echo 用户行为记录表迁移完成！ 
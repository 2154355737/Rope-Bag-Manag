@echo off
echo 绳包管理器后端服务 - 数据库迁移和启动

echo 正在执行数据库迁移...
sqlite3 data.db ".read sql/migrate_comments.sql"

echo 正在启动服务...
call start.bat 
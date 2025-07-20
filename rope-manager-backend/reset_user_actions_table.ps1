Write-Host "开始重置用户行为记录表..." -ForegroundColor Yellow

# 删除现有的表和索引
$dropTableSQL = @"
DROP TABLE IF EXISTS user_actions;
DROP INDEX IF EXISTS idx_user_actions_user_id;
DROP INDEX IF EXISTS idx_user_actions_action_type;
DROP INDEX IF EXISTS idx_user_actions_created_at;
DROP INDEX IF EXISTS idx_user_actions_timestamp;
DROP INDEX IF EXISTS idx_user_actions_target;
"@

Write-Host "删除现有表和索引..." -ForegroundColor Yellow
sqlite3 data.db $dropTableSQL

Write-Host "执行用户行为记录表迁移..." -ForegroundColor Green

# 执行SQL迁移脚本
$sqlContent = Get-Content -Path "./sql/migrate_user_actions.sql" -Raw
sqlite3 data.db $sqlContent

Write-Host "用户行为记录表迁移完成！" -ForegroundColor Green
Write-Host "请重启服务器应用新的表结构" -ForegroundColor Cyan 
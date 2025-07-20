Write-Host "执行用户行为记录表迁移..." -ForegroundColor Green

# 执行SQL迁移脚本
$sqlContent = Get-Content -Path "./sql/migrate_user_actions.sql" -Raw
sqlite3 data.db $sqlContent

Write-Host "用户行为记录表迁移完成！" -ForegroundColor Green 
# 设置输出编码为UTF-8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$OutputEncoding = [System.Text.Encoding]::UTF8

Write-Host "===== 绳包管理器资源记录 - 数据库迁移和测试数据导入 =====" -ForegroundColor Cyan

# 检查SQLite3是否安装
try {
    $sqliteVersion = & sqlite3 --version
    Write-Host "✓ 检测到SQLite版本: $sqliteVersion" -ForegroundColor Green
}
catch {
    Write-Host "✗ 未检测到SQLite3，请确保sqlite3.exe在PATH环境变量中" -ForegroundColor Red
    exit
}

# 执行数据库表迁移
Write-Host "`n[1/2] 执行数据库迁移..." -ForegroundColor Yellow
try {
    & sqlite3 data.db ".read sql/migrate_resource_records.sql"
    Write-Host "✓ 数据库迁移成功！" -ForegroundColor Green
}
catch {
    Write-Host "✗ 迁移失败: $_" -ForegroundColor Red
    exit
}

# 导入测试数据
Write-Host "`n[2/2] 导入测试数据..." -ForegroundColor Yellow
try {
    & sqlite3 data.db ".read sql/test_data_resource_records.sql"
    $count = & sqlite3 data.db "SELECT COUNT(*) FROM resource_records;"
    Write-Host "✓ 测试数据导入成功，共导入 $count 条记录" -ForegroundColor Green
}
catch {
    Write-Host "✗ 导入失败: $_" -ForegroundColor Red
    exit
}

Write-Host "`n===== 数据库迁移和测试数据导入完成！ =====" -ForegroundColor Cyan
Write-Host "现在您可以启动后端服务并访问资源记录管理页面。" -ForegroundColor White

Write-Host "`n按任意键退出..." -ForegroundColor Gray
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown") 
Write-Host "迁移公告表..." -ForegroundColor Green

$dbPath = "data.db"
$sqlPath = "sql/migrate_announcements.sql"

if (-not (Test-Path $dbPath)) {
    Write-Host "数据库文件不存在！请先创建数据库。" -ForegroundColor Red
    exit 1
}

if (-not (Test-Path $sqlPath)) {
    Write-Host "SQL迁移文件不存在！" -ForegroundColor Red
    exit 1
}

# 运行SQL迁移脚本
Get-Content $sqlPath -Raw | Out-File -FilePath "temp_migrate.sql" -Encoding utf8
sqlite3 $dbPath ".read temp_migrate.sql"

if ($LASTEXITCODE -eq 0) {
    Write-Host "公告表迁移成功！" -ForegroundColor Green
    Remove-Item -Path "temp_migrate.sql" -Force
} else {
    Write-Host "公告表迁移失败！" -ForegroundColor Red
    Remove-Item -Path "temp_migrate.sql" -Force
    exit 1
}

Write-Host "按任意键继续..."
$host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown") | Out-Null 
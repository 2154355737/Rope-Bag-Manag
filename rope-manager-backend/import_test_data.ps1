# 设置输出编码为UTF-8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

Write-Host "开始导入资源记录测试数据..." -ForegroundColor Cyan

# 检查SQLite3是否安装
try {
    $sqliteVersion = & sqlite3 --version
    Write-Host "检测到SQLite版本: $sqliteVersion" -ForegroundColor Green
}
catch {
    Write-Host "未检测到SQLite3，请确保sqlite3.exe在PATH环境变量中" -ForegroundColor Red
    exit
}

# 执行SQL文件
try {
    & sqlite3 data.db ".read sql/test_data_resource_records.sql"
    Write-Host "测试数据导入成功！" -ForegroundColor Green
    
    # 验证数据导入
    $count = & sqlite3 data.db "SELECT COUNT(*) FROM resource_records;"
    Write-Host "已导入 $count 条记录" -ForegroundColor Cyan
}
catch {
    Write-Host "导入失败: $_" -ForegroundColor Red
}

Write-Host "按任意键退出..." -ForegroundColor Yellow
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown") 
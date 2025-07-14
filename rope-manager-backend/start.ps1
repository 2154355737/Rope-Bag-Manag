# 绳包管理器后端启动脚本
Write-Host "启动绳包管理器后端服务..." -ForegroundColor Green
Write-Host ""

# 检查配置文件
if (-not (Test-Path "config.toml")) {
    Write-Host "配置文件不存在，正在创建默认配置..." -ForegroundColor Yellow
    & ".\config_manager.ps1" "new"
    Write-Host ""
}

Write-Host "加载配置..." -ForegroundColor Yellow
& ".\config_manager.ps1" "show"

Write-Host ""
Write-Host "编译并启动服务..." -ForegroundColor Yellow
cargo run

Read-Host "按任意键继续..." 
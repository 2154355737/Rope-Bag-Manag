# 绳包管理器全栈启动脚本 (PowerShell版本)
param(
    [switch]$SkipBackend,
    [switch]$SkipFrontend,
    [switch]$Help
)

if ($Help) {
    Write-Host "用法: .\start_fullstack.ps1 [-SkipBackend] [-SkipFrontend] [-Help]" -ForegroundColor Cyan
    Write-Host "  -SkipBackend: 跳过后端启动" -ForegroundColor Yellow
    Write-Host "  -SkipFrontend: 跳过前端启动" -ForegroundColor Yellow
    Write-Host "  -Help: 显示帮助信息" -ForegroundColor Yellow
    exit 0
}

# 设置控制台编码
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

Write-Host "==========================================" -ForegroundColor Green
Write-Host "绳包管理器全栈启动脚本" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Green

# 检查后端配置
if (-not $SkipBackend) {
    Write-Host "`n1. 检查后端配置..." -ForegroundColor Cyan
    if (-not (Test-Path "rope-manager-backend\config.toml")) {
        Write-Host "创建默认配置文件..." -ForegroundColor Yellow
        Set-Location "rope-manager-backend"
        & .\config_manager.ps1 new
        Set-Location ".."
    }
}

# 启动后端服务
if (-not $SkipBackend) {
    Write-Host "`n2. 启动后端服务..." -ForegroundColor Cyan
    Start-Process -FilePath "cmd" -ArgumentList "/k", "cd rope-manager-backend && cargo run" -WindowStyle Normal
    Write-Host "等待后端服务启动..." -ForegroundColor Yellow
    Start-Sleep -Seconds 5
}

# 检查前端依赖
if (-not $SkipFrontend) {
    Write-Host "`n3. 检查前端依赖..." -ForegroundColor Cyan
    if (-not (Test-Path "Rust_Vue\node_modules")) {
        Write-Host "安装前端依赖..." -ForegroundColor Yellow
        Set-Location "Rust_Vue"
        npm install
        Set-Location ".."
    }
}

# 启动前端服务
if (-not $SkipFrontend) {
    Write-Host "`n4. 启动前端服务..." -ForegroundColor Cyan
    Start-Process -FilePath "cmd" -ArgumentList "/k", "cd Rust_Vue && npm run dev" -WindowStyle Normal
    Write-Host "等待前端服务启动..." -ForegroundColor Yellow
    Start-Sleep -Seconds 3
}

Write-Host "`n==========================================" -ForegroundColor Green
Write-Host "🎉 全栈服务启动完成！" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Green
Write-Host ""
Write-Host "后端服务: http://127.0.0.1:15201" -ForegroundColor Cyan
Write-Host "前端服务: http://localhost:5173" -ForegroundColor Cyan
Write-Host ""
Write-Host "测试API: .\rope-manager-backend\test_api.ps1" -ForegroundColor Yellow
Write-Host ""
Write-Host "按任意键退出..." -ForegroundColor Gray
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown") 
# 绳包管理器后端API测试脚本
# 设置控制台编码为UTF-8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$OutputEncoding = [System.Text.Encoding]::UTF8

# 从配置文件读取端口配置
$configFile = "config.toml"
$port = 8080  # 默认端口

if (Test-Path $configFile) {
    $configContent = Get-Content $configFile -Raw
    if ($configContent -match 'port\s*=\s*(\d+)') {
        $port = $matches[1]
        Write-Host "从配置文件读取端口: $port" -ForegroundColor Cyan
    }
}

$baseUrl = "http://127.0.0.1:$port"

Write-Host "==========================================" -ForegroundColor Green
Write-Host "绳包管理器后端API测试脚本" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Green
Write-Host "服务器地址: $baseUrl" -ForegroundColor Cyan
Write-Host ""

Write-Host "1. 测试健康检查..." -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "$baseUrl/health" -Method GET -TimeoutSec 10
    Write-Host "✅ 健康检查成功" -ForegroundColor Green
    Write-Host "响应内容: $($response | ConvertTo-Json -Depth 3)" -ForegroundColor Gray
} catch {
    Write-Host "❌ 健康检查失败: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host ""
Write-Host "2. 测试服务器状态..." -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "$baseUrl/api/v1/packages" -Method GET -TimeoutSec 10
    Write-Host "✅ 服务器状态正常" -ForegroundColor Green
    Write-Host "响应内容: $($response | ConvertTo-Json -Depth 3)" -ForegroundColor Gray
} catch {
    Write-Host "❌ 服务器状态检查失败: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host ""
Write-Host "3. 测试用户注册..." -ForegroundColor Yellow
$registerData = @{
    username = "testuser"
    password = "testpass123"
    email = "test@example.com"
} | ConvertTo-Json

try {
    $response = Invoke-RestMethod -Uri "$baseUrl/api/v1/auth/register" -Method POST -Body $registerData -ContentType "application/json" -TimeoutSec 10
    Write-Host "✅ 用户注册成功" -ForegroundColor Green
    Write-Host "响应内容: $($response | ConvertTo-Json -Depth 3)" -ForegroundColor Gray
} catch {
    Write-Host "❌ 用户注册失败: $($_.Exception.Message)" -ForegroundColor Red
    if ($_.Exception.Response) {
        $errorResponse = $_.Exception.Response.GetResponseStream()
        $reader = New-Object System.IO.StreamReader($errorResponse)
        $errorContent = $reader.ReadToEnd()
        Write-Host "错误详情: $errorContent" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "4. 测试用户登录..." -ForegroundColor Yellow
$loginData = @{
    username = "testuser"
    password = "testpass123"
} | ConvertTo-Json

try {
    $response = Invoke-RestMethod -Uri "$baseUrl/api/v1/auth/login" -Method POST -Body $loginData -ContentType "application/json" -TimeoutSec 10
    Write-Host "✅ 用户登录成功" -ForegroundColor Green
    Write-Host "响应内容: $($response | ConvertTo-Json -Depth 3)" -ForegroundColor Gray
} catch {
    Write-Host "❌ 用户登录失败: $($_.Exception.Message)" -ForegroundColor Red
    if ($_.Exception.Response) {
        $errorResponse = $_.Exception.Response.GetResponseStream()
        $reader = New-Object System.IO.StreamReader($errorResponse)
        $errorContent = $reader.ReadToEnd()
        Write-Host "错误详情: $errorContent" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "5. 测试获取用户信息..." -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "$baseUrl/api/v1/auth/user-info" -Method GET -TimeoutSec 10
    Write-Host "✅ 获取用户信息成功" -ForegroundColor Green
    Write-Host "响应内容: $($response | ConvertTo-Json -Depth 3)" -ForegroundColor Gray
} catch {
    Write-Host "❌ 获取用户信息失败: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host ""
Write-Host "==========================================" -ForegroundColor Green
Write-Host "🎉 API测试完成！" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Green
Write-Host ""
Write-Host "测试结果说明:" -ForegroundColor Yellow
Write-Host "- ✅ 表示测试成功" -ForegroundColor Green
Write-Host "- ❌ 表示测试失败" -ForegroundColor Red
Write-Host "- 灰色文本显示详细响应内容" -ForegroundColor Gray
Write-Host ""

Read-Host "按任意键继续..." 
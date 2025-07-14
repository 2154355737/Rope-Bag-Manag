# 绳包管理器后端高级API测试脚本
# 设置控制台编码为UTF-8
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$OutputEncoding = [System.Text.Encoding]::UTF8

# 从配置文件读取配置
$configFile = "config.toml"
$port = 8080  # 默认端口
$serverHost = "127.0.0.1"  # 默认主机

if (Test-Path $configFile) {
    $configContent = Get-Content $configFile -Raw
    if ($configContent -match 'port\s*=\s*(\d+)') {
        $port = $matches[1]
    }
    if ($configContent -match 'host\s*=\s*"([^"]+)"') {
        $serverHost = $matches[1]
    }
}

$baseUrl = "http://$serverHost`:$port"

# 颜色函数
function Write-Success { param($text) Write-Host $text -ForegroundColor Green }
function Write-Error { param($text) Write-Host $text -ForegroundColor Red }
function Write-Info { param($text) Write-Host $text -ForegroundColor Cyan }
function Write-Warning { param($text) Write-Host $text -ForegroundColor Yellow }

# 测试结果统计
$testResults = @{
    Total = 0
    Passed = 0
    Failed = 0
}

function Test-API {
    param(
        [string]$Name,
        [string]$Method,
        [string]$Url,
        [string]$Body = $null,
        [hashtable]$Headers = @{},
        [int]$ExpectedStatus = 200
    )
    
    $testResults.Total++
    Write-Host "`n🔍 测试: $Name" -ForegroundColor Yellow
    Write-Host "URL: $Url" -ForegroundColor Gray
    Write-Host "方法: $Method" -ForegroundColor Gray
    
    try {
        $params = @{
            Uri = $Url
            Method = $Method
            TimeoutSec = 10
        }
        
        if ($Body) {
            $params.Body = $Body
            $params.ContentType = "application/json"
            Write-Host "请求体: $Body" -ForegroundColor Gray
        }
        
        if ($Headers.Count -gt 0) {
            $params.Headers = $Headers
        }
        
        $response = Invoke-RestMethod @params
        $statusCode = $response.StatusCode
        
        if ($statusCode -eq $ExpectedStatus) {
            Write-Success "✅ $Name - 成功"
            $testResults.Passed++
            Write-Host "响应: $($response | ConvertTo-Json -Depth 2)" -ForegroundColor Gray
        } else {
            Write-Error "❌ $Name - 失败 (期望状态码: $ExpectedStatus, 实际: $statusCode)"
            $testResults.Failed++
        }
    } catch {
        Write-Error "❌ $Name - 失败: $($_.Exception.Message)"
        $testResults.Failed++
        
        # 显示详细错误信息
        if ($_.Exception.Response) {
            try {
                $errorResponse = $_.Exception.Response.GetResponseStream()
                $reader = New-Object System.IO.StreamReader($errorResponse)
                $errorContent = $reader.ReadToEnd()
                Write-Host "错误详情: $errorContent" -ForegroundColor Red
            } catch {
                Write-Host "无法读取错误详情" -ForegroundColor Red
            }
        }
    }
}

# 主测试流程
Write-Host "==========================================" -ForegroundColor Green
Write-Host "绳包管理器后端高级API测试脚本" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Green
Write-Info "服务器地址: $baseUrl"
Write-Info "配置文件: $configFile"
Write-Host ""

# 1. 健康检查
Test-API -Name "健康检查" -Method "GET" -Url "$baseUrl/health"

# 2. 服务器状态
Test-API -Name "服务器状态" -Method "GET" -Url "$baseUrl/api/v1/packages"

# 3. 用户注册
$registerData = @{
    username = "testuser_$(Get-Date -Format 'yyyyMMdd_HHmmss')"
    password = "testpass123"
    email = "test@example.com"
} | ConvertTo-Json

Test-API -Name "用户注册" -Method "POST" -Url "$baseUrl/api/v1/auth/register" -Body $registerData

# 4. 用户登录
$loginData = @{
    username = "testuser"
    password = "testpass123"
} | ConvertTo-Json

Test-API -Name "用户登录" -Method "POST" -Url "$baseUrl/api/v1/auth/login" -Body $loginData

# 5. 获取用户信息（需要认证）
Test-API -Name "获取用户信息" -Method "GET" -Url "$baseUrl/api/v1/auth/user-info"

# 6. 测试绳包列表
Test-API -Name "获取绳包列表" -Method "GET" -Url "$baseUrl/api/v1/packages"

# 7. 测试用户列表
Test-API -Name "获取用户列表" -Method "GET" -Url "$baseUrl/api/v1/users"

# 8. 测试管理员统计
Test-API -Name "获取管理员统计" -Method "GET" -Url "$baseUrl/api/v1/admin/stats"

# 9. 测试社区评论
Test-API -Name "获取评论列表" -Method "GET" -Url "$baseUrl/api/v1/community/comments"

# 显示测试结果
Write-Host "`n==========================================" -ForegroundColor Green
Write-Host "📊 测试结果统计" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Green
Write-Host "总测试数: $($testResults.Total)" -ForegroundColor White
Write-Success "通过: $($testResults.Passed)"
Write-Error "失败: $($testResults.Failed)"

$successRate = if ($testResults.Total -gt 0) { [math]::Round(($testResults.Passed / $testResults.Total) * 100, 2) } else { 0 }
Write-Host "成功率: $successRate%" -ForegroundColor Cyan

if ($testResults.Failed -eq 0) {
    Write-Host "`n🎉 所有测试通过！" -ForegroundColor Green
} else {
    Write-Host "`n⚠️  有 $($testResults.Failed) 个测试失败" -ForegroundColor Yellow
}

Write-Host "`n==========================================" -ForegroundColor Green
Write-Host "测试完成！" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Green

# 保存测试报告
$reportFile = "test_report_$(Get-Date -Format 'yyyyMMdd_HHmmss').json"
$report = @{
    timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    server_url = $baseUrl
    total_tests = $testResults.Total
    passed_tests = $testResults.Passed
    failed_tests = $testResults.Failed
    success_rate = $successRate
} | ConvertTo-Json -Depth 3

$report | Out-File -FilePath $reportFile -Encoding UTF8
Write-Info "测试报告已保存到: $reportFile"

Read-Host "`n按任意键继续..." 
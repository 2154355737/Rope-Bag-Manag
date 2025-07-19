# 测试登录API
$loginUrl = "http://127.0.0.1:15201/api/v1/auth/login"
$loginData = @{
    username = "admin"
    password = "admin123"
} | ConvertTo-Json

Write-Host "测试登录API..."
try {
    $response = Invoke-RestMethod -Uri $loginUrl -Method POST -Body $loginData -ContentType "application/json"
    Write-Host "登录成功!"
    Write-Host "Token: $($response.data.token)"
    Write-Host "用户信息: $($response.data.user | ConvertTo-Json)"
    
    # 测试获取用户列表
    $usersUrl = "http://127.0.0.1:15201/api/v1/users"
    $headers = @{
        "Authorization" = "Bearer $($response.data.token)"
        "Content-Type" = "application/json"
    }
    
    Write-Host "`n测试获取用户列表..."
    $usersResponse = Invoke-RestMethod -Uri $usersUrl -Method GET -Headers $headers
    Write-Host "获取用户列表成功!"
    Write-Host "用户数量: $($usersResponse.data.total)"
    
} catch {
    Write-Host "错误: $($_.Exception.Message)"
    if ($_.Exception.Response) {
        $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
        $responseBody = $reader.ReadToEnd()
        Write-Host "响应内容: $responseBody"
    }
} 
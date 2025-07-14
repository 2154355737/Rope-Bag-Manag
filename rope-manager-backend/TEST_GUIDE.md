# API测试脚本使用指南

## 📋 测试脚本概述

绳包管理器后端提供了多个测试脚本，用于验证API功能：

| 脚本名称 | 类型 | 特点 | 适用场景 |
|----------|------|------|----------|
| `test_api.bat` | 批处理 | 基础测试，快速验证 | 简单功能测试 |
| `test_api.ps1` | PowerShell | 中文显示，详细输出 | 日常测试 |
| `test_api_advanced.ps1` | PowerShell | 完整测试，生成报告 | 全面测试 |

## 🚀 快速开始

### 1. 基础测试（推荐）

```powershell
# 使用PowerShell脚本（支持中文）
powershell -ExecutionPolicy Bypass -File test_api.ps1
```

### 2. 高级测试

```powershell
# 完整测试，包含详细报告
powershell -ExecutionPolicy Bypass -File test_api_advanced.ps1
```

### 3. 批处理测试

```cmd
# 使用批处理脚本
test_api.bat
```

## 📊 测试内容

### 基础测试包含：

1. **健康检查** - 验证服务是否正常运行
2. **服务器状态** - 检查API端点可访问性
3. **用户注册** - 测试用户注册功能
4. **用户登录** - 测试用户登录功能
5. **获取用户信息** - 测试用户信息获取

### 高级测试额外包含：

6. **绳包列表** - 测试绳包管理功能
7. **用户列表** - 测试用户管理功能
8. **管理员统计** - 测试管理员功能
9. **社区评论** - 测试社区功能

## 🔧 配置说明

### 自动端口检测

测试脚本会自动从 `config.toml` 文件读取端口配置：

```toml
[server]
port = 15201  # 测试脚本会自动使用这个端口
```

### 中文编码支持

PowerShell脚本设置了UTF-8编码，确保中文显示正常：

```powershell
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$OutputEncoding = [System.Text.Encoding]::UTF8
```

## 📈 测试结果

### 成功标志

- ✅ 表示测试通过
- 绿色文本显示成功信息
- 灰色文本显示详细响应内容

### 失败标志

- ❌ 表示测试失败
- 红色文本显示错误信息
- 包含详细的错误描述

### 测试报告

高级测试脚本会生成JSON格式的测试报告：

```json
{
  "timestamp": "2024-01-01 12:00:00",
  "server_url": "http://127.0.0.1:15201",
  "total_tests": 9,
  "passed_tests": 8,
  "failed_tests": 1,
  "success_rate": 88.89
}
```

## 🛠️ 自定义测试

### 修改测试端口

编辑 `config.toml` 文件：

```toml
[server]
port = 你的端口号
```

### 添加新的测试

在 `test_api_advanced.ps1` 中添加新的测试：

```powershell
# 添加新的API测试
Test-API -Name "你的测试名称" -Method "GET" -Url "$baseUrl/your/api/endpoint"
```

### 自定义测试数据

修改测试脚本中的请求数据：

```powershell
$customData = @{
    username = "your_username"
    password = "your_password"
    email = "your_email@example.com"
} | ConvertTo-Json

Test-API -Name "自定义测试" -Method "POST" -Url "$baseUrl/api/v1/auth/register" -Body $customData
```

## 🔍 故障排除

### 常见问题

1. **端口被占用**
   ```
   错误: 无法连接到服务器
   解决: 检查端口是否被其他程序占用
   ```

2. **服务未启动**
   ```
   错误: 连接被拒绝
   解决: 确保后端服务已启动
   ```

3. **配置文件错误**
   ```
   错误: 无法读取配置文件
   解决: 检查 config.toml 文件格式
   ```

4. **权限问题**
   ```
   错误: 无法执行脚本
   解决: 以管理员身份运行PowerShell
   ```

### 调试步骤

1. **检查服务状态**
   ```powershell
   # 手动测试健康检查
   Invoke-RestMethod -Uri "http://127.0.0.1:15201/health"
   ```

2. **检查配置文件**
   ```powershell
   # 查看当前配置
   .\config_manager.ps1 show
   ```

3. **检查端口占用**
   ```cmd
   # 检查端口是否被占用
   netstat -an | findstr 15201
   ```

4. **查看详细错误**
   ```powershell
   # 启用详细错误信息
   $VerbosePreference = "Continue"
   .\test_api.ps1
   ```

## 📝 最佳实践

### 测试前准备

1. **确保服务运行**
   ```powershell
   # 启动服务
   .\start.ps1
   ```

2. **检查配置文件**
   ```powershell
   # 验证配置
   .\config_manager.ps1 test
   ```

3. **清理测试数据**
   ```powershell
   # 删除测试数据库（可选）
   Remove-Item data.db -ErrorAction SilentlyContinue
   ```

### 测试后检查

1. **查看测试报告**
   ```powershell
   # 查看生成的测试报告
   Get-Content test_report_*.json
   ```

2. **检查日志文件**
   ```powershell
   # 查看服务日志
   Get-Content logs/app.log -Tail 20
   ```

3. **验证数据完整性**
   ```powershell
   # 检查数据库状态
   .\config_manager.ps1 show
   ```

## 🎯 测试策略

### 开发环境测试

- 使用基础测试脚本
- 重点关注核心功能
- 快速验证修复

### 集成测试

- 使用高级测试脚本
- 全面测试所有功能
- 生成详细报告

### 生产环境测试

- 使用自定义测试脚本
- 重点测试关键路径
- 监控性能指标

## 📞 技术支持

如果遇到测试问题，请：

1. 检查服务是否正常运行
2. 验证配置文件是否正确
3. 查看详细的错误信息
4. 确认网络连接正常
5. 检查防火墙设置

测试脚本是验证API功能的重要工具，建议在每次部署前都运行完整的测试套件。 
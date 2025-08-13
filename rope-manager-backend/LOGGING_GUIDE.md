# 🚀 绳包管理器后端日志系统指南

## 📋 概述

优化后的日志系统提供了清晰、结构化的日志输出，便于开发调试和生产监控。

## 🎨 日志格式

```
2024-01-15 14:30:25.123 [INFO] [main] 🚀 绳包管理器后端服务启动
2024-01-15 14:30:25.456 [INFO] [api_logger] 🌐 GET /api/v1/packages 200 45ms ⚡ [user:123]
```

### 格式说明
- **时间戳**: 精确到毫秒的时间戳
- **级别**: 彩色编码的日志级别
- **模块**: 简化的模块路径
- **内容**: 带有emoji表情的结构化消息

## 🌈 日志级别

| 级别 | 颜色 | 用途 |
|------|------|------|
| ERROR | 🔴 红色 | 系统错误、API失败 |
| WARN  | 🟡 黄色 | 警告信息、慢请求 |
| INFO  | 🟢 绿色 | 一般信息、API请求 |
| DEBUG | 🔵 青色 | 调试信息、SQL查询 |
| TRACE | ⚪ 白色 | 详细跟踪信息 |

## 🛠️ 环境变量配置

### 设置日志级别
```bash
# 开发环境 - 显示所有信息
export RUST_LOG=debug

# 生产环境 - 只显示重要信息
export RUST_LOG=info

# 特定模块调试
export RUST_LOG=rope_manager_backend::repositories=debug,info
```

### 日志输出控制
```bash
# 输出到文件
cargo run > logs/server.log 2>&1

# 实时查看日志
cargo run | tee logs/server.log

# 只看错误
cargo run 2>&1 | grep ERROR
```

## 🔍 API请求日志

### 日志格式
```
🌐 METHOD /path STATUS TIME_MS PERFORMANCE_ICON [USER_INFO]
```

### 示例
```bash
# 正常请求
🌐 GET /api/v1/packages 200 45ms ⚡ [user:123]

# 慢请求
🌐 POST /api/v1/packages 201 1200ms 🐌 [user:456]

# 错误请求  
🌐 GET /api/v1/invalid 404 12ms ⚡ [192.168.1.100]

# 服务器错误
🌐 POST /api/v1/packages 500 89ms ⚡ [user:789]
```

### 性能指示器
- ⚡ 快速请求 (< 500ms)
- ⚠️ 中等请求 (500ms - 1s)  
- 🐌 慢请求 (> 1s)

## 📊 系统日志示例

### 启动日志
```bash
🚀 绳包管理器后端服务启动
📦 Version: 1.0.0
🌐 Server: http://127.0.0.1:15201
📋 API 文档: http://127.0.0.1:15201/swagger-ui/
✅ 所有服务初始化完成
🌐 API服务启动在: http://127.0.0.1:15201
```

### 邮件日志
```bash
📧 Email sent successfully: user@example.com -> '账户验证'
📧 Email failed: invalid@domain.com -> '密码重置', error: SMTP连接失败
```

### 用户行为日志
```bash
👤 User 123 performed: 资源点赞 on Package:456
👤 User 456 performed: 注册
```

### 安全日志
```bash
🔒 Suspicious activity from 192.168.1.100: 频繁登录尝试
🚫 Blocked access from 192.168.1.200: IP被封禁
🔐 Authentication failed for 'testuser' from 192.168.1.50
```

## 🛡️ 生产环境建议

### 1. 日志轮转
```bash
# 使用logrotate配置
/var/log/rope-manager/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 644 rust rust
}
```

### 2. 监控配置
```bash
# 监控错误日志
tail -f logs/server.log | grep ERROR

# 监控慢请求
tail -f logs/server.log | grep "🐌"

# 监控认证失败
tail -f logs/server.log | grep "🔐"
```

### 3. 性能分析
```bash
# 统计API响应时间
grep "🌐" logs/server.log | awk '{print $6}' | sed 's/ms//' | sort -n

# 找出最慢的API
grep "🐌" logs/server.log | sort
```

## 🔧 开发者工具

### 自定义日志宏
```rust
// 性能日志
log_performance!("数据库查询", duration);

// API请求日志  
log_api_request!("GET", "/api/v1/users", user_id);

// 数据库操作日志
log_db_operation!("INSERT", "users", user_id);

// 邮件日志
log_email!(success, "user@example.com", "验证邮件");

// 用户行为日志
log_user_action!(user_id, "登录", "成功");

// 安全日志
log_security!(auth_failed, "testuser", "192.168.1.100");
```

### 调试技巧
1. **临时提高日志级别**: `RUST_LOG=debug cargo run`
2. **过滤特定模块**: `RUST_LOG=package_service=debug cargo run`
3. **实时监控**: `tail -f logs/server.log | grep "关键词"`
4. **错误追踪**: 所有错误都包含完整的错误链

## 📝 日志最佳实践

### ✅ 好的做法
- 使用结构化的日志消息
- 包含足够的上下文信息
- 使用适当的日志级别
- 避免在循环中打印大量日志

### ❌ 避免的做法
- 不要在生产环境使用DEBUG级别
- 不要记录敏感信息（密码、令牌）
- 不要使用println!进行调试
- 不要记录过于详细的用户数据

## 🎯 故障排查指南

### 常见问题排查
1. **服务启动失败**: 查看ERROR级别日志
2. **API响应慢**: 搜索🐌标识的请求
3. **认证问题**: 查看🔐相关日志
4. **邮件发送失败**: 搜索📧 Email failed

### 日志分析命令
```bash
# 最近的错误
tail -100 logs/server.log | grep ERROR

# 统计各状态码数量
grep "🌐" logs/server.log | awk '{print $4}' | sort | uniq -c

# 最活跃的用户
grep "user:" logs/server.log | sed 's/.*user:\([0-9]*\).*/\1/' | sort | uniq -c | sort -nr

# 最慢的API端点
grep "🐌" logs/server.log | awk '{print $3}' | sort | uniq -c | sort -nr
```

---

💡 **提示**: 合理使用日志可以大大提高开发效率和系统可维护性！ 
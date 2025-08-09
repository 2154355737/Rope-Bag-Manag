# 下载防刷量系统使用说明

## 系统概述

本系统为绳包管理器提供了完善的下载防刷量保护机制，包含多层安全检测和智能异常识别。

## 核心功能

### 1. 多层级频率限制

- **用户级别限制**：每个用户对同一资源的下载频率限制
- **IP级别限制**：同一IP的下载频率限制，防止IP轮换攻击
- **资源级别限制**：基于访问量的智能限制
- **全局限制**：系统整体的下载频率控制

### 2. 智能异常检测

- **可疑模式检测**：识别机器人行为、批量下载等可疑模式
- **统计异常检测**：基于历史数据的异常下载率检测
- **User-Agent检测**：识别可疑的浏览器标识

### 3. 实时监控

- **异常记录**：详细记录所有检测到的异常行为
- **统计报表**：提供异常类型、严重程度等统计信息
- **配置管理**：支持动态调整检测参数

## 数据库表结构

### download_records（下载记录表）
```sql
CREATE TABLE download_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    package_id INTEGER NOT NULL,
    ip_address TEXT NOT NULL,
    user_agent TEXT,
    download_time TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (package_id) REFERENCES packages(id)
);
```

### download_rate_limits（下载限制规则表）
```sql
CREATE TABLE download_rate_limits (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    rule_type TEXT NOT NULL, -- 'user', 'ip', 'resource', 'global'
    target_id INTEGER, -- user_id, package_id, 或 NULL(全局)
    time_window INTEGER NOT NULL, -- 时间窗口(秒)
    max_downloads INTEGER NOT NULL, -- 最大下载次数
    is_active BOOLEAN NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

### download_anomalies（异常检测记录表）
```sql
CREATE TABLE download_anomalies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    anomaly_type TEXT NOT NULL, -- 'rate_limit_exceeded', 'suspicious_pattern', 'statistical_anomaly'
    user_id INTEGER,
    package_id INTEGER,
    ip_address TEXT,
    details TEXT, -- JSON格式的详细信息
    severity TEXT NOT NULL, -- 'low', 'medium', 'high', 'critical'
    is_resolved BOOLEAN NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    resolved_at TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (package_id) REFERENCES packages(id)
);
```

## 配置参数

### DownloadSecurityConfig
```rust
pub struct DownloadSecurityConfig {
    pub enable_rate_limiting: bool,           // 启用频率限制
    pub enable_anomaly_detection: bool,       // 启用异常检测
    pub enable_statistical_analysis: bool,    // 启用统计分析
    pub max_downloads_per_user_per_hour: i32, // 用户每小时最大下载次数
    pub max_downloads_per_ip_per_hour: i32,   // IP每小时最大下载次数
    pub max_downloads_per_resource_per_day: i32, // 资源每天最大下载次数
    pub suspicious_pattern_threshold: f64,    // 可疑模式阈值
    pub statistical_anomaly_threshold: f64,   // 统计异常阈值
}
```

## API接口

### 1. 下载资源（带安全检测）
```
GET /v1/packages/{id}/download
```

**响应示例：**
```json
{
  "code": 0,
  "message": "success",
  "data": "file_url"
}
```

**错误响应：**
```json
{
  "code": 403,
  "message": "下载被阻止: 用户下载频率超限: 10次/小时"
}
```

### 2. 获取安全统计
```
GET /v1/download-security/stats
```

**响应示例：**
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "total_anomalies": 15,
    "by_severity": {
      "critical": 2,
      "high": 5,
      "medium": 6,
      "low": 2
    },
    "by_type": {
      "rate_limit_exceeded": 8,
      "suspicious_pattern": 4,
      "statistical_anomaly": 3
    }
  }
}
```

### 3. 获取安全配置
```
GET /v1/download-security/config
```

### 4. 更新安全配置
```
PUT /v1/download-security/config
```

## 使用建议

### 1. 合理设置限制参数

- **用户限制**：建议设置为5-20次/小时，根据用户类型调整
- **IP限制**：建议设置为10-50次/小时，防止IP轮换攻击
- **资源限制**：建议设置为50-200次/天，根据资源热度调整

### 2. 监控异常行为

- 定期查看异常统计，识别攻击模式
- 关注高严重程度的异常，及时处理
- 根据异常类型调整检测参数

### 3. 动态调整策略

- 根据实际使用情况调整阈值
- 针对不同类型的资源设置差异化策略
- 定期评估和优化检测规则

## 部署说明

### 1. 数据库初始化
```bash
# 执行防刷量系统表创建SQL
sqlite3 data.db < sql/create_download_security_tables.sql
```

### 2. 服务配置
在 `main.rs` 中已经集成了防刷量服务，无需额外配置。

### 3. 前端集成
在管理后台添加了下载安全监控页面，可以实时查看统计信息和调整配置。

## 监控指标

### 1. 异常类型
- **rate_limit_exceeded**：频率限制超限
- **suspicious_pattern**：可疑模式检测
- **statistical_anomaly**：统计异常

### 2. 严重程度
- **critical**：严重异常，需要立即处理
- **high**：高危异常，需要关注
- **medium**：中等异常，需要监控
- **low**：低危异常，记录即可

### 3. 检测维度
- **用户维度**：基于用户ID的下载行为分析
- **IP维度**：基于IP地址的下载行为分析
- **资源维度**：基于资源ID的下载行为分析
- **时间维度**：基于时间窗口的下载行为分析

## 故障排除

### 1. 误报处理
如果发现正常用户被误判为异常，可以：
- 调整可疑模式阈值
- 放宽频率限制参数
- 添加白名单机制

### 2. 漏报处理
如果发现攻击行为未被检测，可以：
- 降低可疑模式阈值
- 收紧频率限制参数
- 增加新的检测规则

### 3. 性能优化
如果系统性能受到影响，可以：
- 优化数据库查询
- 增加缓存机制
- 调整检测频率

## 扩展功能

### 1. 机器学习检测
可以集成机器学习模型，提高异常检测的准确性。

### 2. 实时告警
可以集成邮件、短信等告警机制，及时通知管理员。

### 3. 白名单机制
可以为特定用户或IP添加白名单，避免误判。

### 4. 地理位置检测
可以基于IP地理位置进行异常检测。

## 总结

本防刷量系统提供了多层次的安全保护，既能有效防止恶意刷量，又能避免误判正常用户。通过合理的配置和持续的监控，可以为系统提供可靠的安全保障。 
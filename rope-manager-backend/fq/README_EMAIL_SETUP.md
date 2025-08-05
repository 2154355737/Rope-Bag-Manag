# 邮件系统配置指南

## 快速诊断

如果您遇到 `unexpected EOF during handshake` 错误，请按以下步骤检查：

### QQ邮箱用户（最常见问题）

**🚨 立即检查以下设置：**

1. **SMTP服务是否已开启**
   - 登录 [QQ邮箱网页版](https://mail.qq.com)
   - 进入"设置" → "账户" → "POP3/IMAP/SMTP/Exchange/CardDAV/CalDAV服务"
   - 确保"IMAP/SMTP服务"为开启状态
   - 如果没有开启，按提示发送短信验证开启

2. **授权码获取与使用**
   - 在SMTP服务开启页面点击"生成授权码"
   - 按提示发送短信获取16位授权码（如：abcdefghijklmnop）
   - **重要**：密码字段必须填写授权码，不是QQ密码

3. **配置验证**
   ```toml
   [mail]
   smtp_server = "smtp.qq.com"
   smtp_port = 465              # QQ邮箱标准SSL端口
   username = "你的QQ号@qq.com"  # 必须是完整邮箱地址
   password = "abcdefghijklmnop" # 16位授权码，不是QQ密码
   from_name = "绳包管理器"
   ```

### 如果仍然失败，尝试以下方案：

#### 方案A：使用587端口（推荐）
```toml
[mail]
smtp_server = "smtp.qq.com"
smtp_port = 587              # 尝试STARTTLS模式
username = "你的QQ号@qq.com"
password = "你的授权码"
from_name = "绳包管理器"
```

#### 方案B：检查网络环境
1. **防火墙检查**
   - Windows Defender防火墙可能阻止465/587端口
   - 临时关闭防火墙测试，或添加端口例外

2. **网络连接测试**
   ```powershell
   # PowerShell中测试连接
   Test-NetConnection smtp.qq.com -Port 465
   Test-NetConnection smtp.qq.com -Port 587
   ```

3. **公司/学校网络**
   - 某些企业网络可能封锁SMTP端口
   - 尝试使用手机热点测试
   - 联系网管开放相关端口

### 其他邮箱配置

#### Gmail
```toml
[mail]
smtp_server = "smtp.gmail.com"
smtp_port = 587
username = "your-email@gmail.com"
password = "your-app-password"  # 需要开启两步验证并生成应用密码
from_name = "绳包管理器"
```

#### 163邮箱
```toml
[mail]
smtp_server = "smtp.163.com"
smtp_port = 465
username = "your-email@163.com"
password = "your-smtp-auth-code"  # 需要开启SMTP服务并获取授权码
from_name = "绳包管理器"
```

#### 126邮箱
```toml
[mail]
smtp_server = "smtp.126.com"
smtp_port = 465
username = "your-email@126.com"
password = "your-smtp-auth-code"
from_name = "绳包管理器"
```

## 错误类型与解决方案

### SSL握手错误
**错误信息**: `unexpected EOF during handshake`

**原因**: SSL/TLS连接建立失败

**解决方案**:
1. 尝试不同端口（465 → 587 或 587 → 465）
2. 检查SMTP服务是否开启
3. 确认网络可访问SMTP服务器
4. 验证用户名和授权码正确性

### 认证错误
**错误信息**: `Authentication failed` 或 `535`

**解决方案**:
1. 确认使用授权码而非登录密码
2. 检查用户名格式（必须包含@domain）
3. 重新生成授权码
4. 确认SMTP服务已开启

### 连接超时
**错误信息**: `timeout` 或 `Connection refused`

**解决方案**:
1. 检查网络连接
2. 确认防火墙设置
3. 尝试不同的SMTP端口
4. 检查服务器地址拼写

## 测试步骤

### 1. 基础配置测试
1. 停止后端服务
2. 修改`config.toml`中的邮件配置
3. 重启后端服务
4. 在管理后台点击"发送测试邮件"

### 2. 详细日志分析
后端会输出详细的连接日志，关注以下信息：
```
[INFO] 初始化邮件服务: smtp.qq.com:465
[INFO] 发送方配置: 绳包管理器 <你的邮箱@qq.com>
[INFO] 使用465端口SSL连接模式 (SMTPS)
[INFO] 尝试建立SSL连接到 smtp.qq.com:465
```

### 3. 逐步排除问题
1. 确认SMTP服务开启 ✓
2. 确认授权码正确 ✓
3. 确认网络连通性 ✓
4. 尝试不同端口配置 ✓
5. 检查防火墙设置 ✓

## 常见问题FAQ

**Q: 为什么不能用QQ密码？**
A: QQ邮箱要求使用专门的SMTP授权码，这是为了安全考虑。

**Q: 授权码在哪里获取？**
A: 登录QQ邮箱网页版 → 设置 → 账户 → 开启SMTP服务时会提示获取。

**Q: 465和587端口有什么区别？**
A: 465端口使用SSL加密，587端口使用STARTTLS加密，两种都安全，但某些网络环境下兼容性不同。

**Q: 企业邮箱怎么配置？**
A: 联系您的IT管理员获取SMTP服务器地址、端口和认证方式。

**Q: 可以使用自建邮件服务器吗？**
A: 可以，只需修改`smtp_server`和`smtp_port`为您的服务器配置即可。

## 技术支持

如果按照上述步骤仍无法解决问题，请：

1. **收集日志信息**：复制完整的错误日志
2. **网络测试结果**：提供端口连通性测试结果
3. **配置脱敏信息**：提供配置文件（隐去敏感信息）
4. **环境信息**：操作系统、网络环境（家庭/企业/学校）

联系方式：[在此填写您的支持渠道]

---

**最后更新**: 2025-07-28  
**版本**: v1.1 
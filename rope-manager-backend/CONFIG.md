# 绳包管理器后端配置说明

## 📋 配置系统概述

绳包管理器后端支持多种配置方式，按优先级排序：

1. **配置文件** (`config.toml`) - 最高优先级
2. **环境变量** - 中等优先级  
3. **默认值** - 最低优先级

## 🔧 配置管理工具

### PowerShell 配置管理脚本

```powershell
# 显示当前配置
.\config_manager.ps1 show

# 创建默认配置文件
.\config_manager.ps1 new

# 编辑配置文件
.\config_manager.ps1 edit

# 测试配置文件语法
.\config_manager.ps1 test

# 显示环境变量
.\config_manager.ps1 env

# 设置环境变量
.\config_manager.ps1 set PORT 9090
.\config_manager.ps1 set HOST 0.0.0.0
```

## 📄 配置文件格式

配置文件使用 TOML 格式，结构如下：

```toml
# 服务器配置
[server]
host = "127.0.0.1"      # 服务器监听地址
port = 8080             # 服务器监听端口
workers = 4             # 工作进程数量
timeout = 30            # 请求超时时间（秒）

# 数据库配置
[database]
url = "data.db"         # 数据库文件路径
max_connections = 10    # 最大连接数
timeout = 30           # 数据库操作超时时间

# 认证配置
[auth]
jwt_secret = "your-secret-key-change-in-production"  # JWT密钥
jwt_expiration = 86400  # JWT过期时间（秒）
bcrypt_cost = 12       # bcrypt加密强度

# 文件配置
[file]
upload_path = "uploads"           # 文件上传目录
max_file_size = 10485760         # 最大文件大小（字节）
allowed_extensions = ["zip", "rar", "7z", "tar", "gz"]  # 允许的文件扩展名
temp_path = "temp"               # 临时文件目录

# 日志配置
[logging]
level = "info"                   # 日志级别
file_path = "logs/app.log"       # 日志文件路径
max_files = 5                   # 最大日志文件数量
max_size = 10485760             # 单个日志文件最大大小

# CORS配置
[cors]
allowed_origins = ["*"]          # 允许的源
allowed_methods = ["GET", "POST", "PUT", "DELETE", "OPTIONS"]  # 允许的HTTP方法
allowed_headers = ["Content-Type", "Authorization", "X-Requested-With"]  # 允许的请求头
max_age = 3600                  # 预检请求缓存时间
```

## 🌍 环境变量

### 服务器配置
| 变量名 | 默认值 | 说明 |
|--------|--------|------|
| `HOST` | `127.0.0.1` | 服务器监听地址 |
| `PORT` | `8080` | 服务器监听端口 |
| `WORKERS` | `4` | 工作进程数量 |
| `TIMEOUT` | `30` | 请求超时时间（秒） |

### 数据库配置
| 变量名 | 默认值 | 说明 |
|--------|--------|------|
| `DATABASE_URL` | `data.db` | 数据库文件路径 |
| `DATABASE_MAX_CONNECTIONS` | `10` | 最大连接数 |

### 认证配置
| 变量名 | 默认值 | 说明 |
|--------|--------|------|
| `JWT_SECRET` | `your-secret-key-change-in-production` | JWT密钥 |
| `JWT_EXPIRATION` | `86400` | JWT过期时间（秒） |
| `BCRYPT_COST` | `12` | bcrypt加密强度 |

### 文件配置
| 变量名 | 默认值 | 说明 |
|--------|--------|------|
| `UPLOAD_PATH` | `uploads` | 文件上传目录 |
| `MAX_FILE_SIZE` | `10485760` | 最大文件大小（字节） |
| `TEMP_PATH` | `temp` | 临时文件目录 |

### 日志配置
| 变量名 | 默认值 | 说明 |
|--------|--------|------|
| `LOG_LEVEL` | `info` | 日志级别 |
| `LOG_FILE` | `logs/app.log` | 日志文件路径 |

## 🎯 配置示例

### 开发环境配置

```toml
[server]
host = "127.0.0.1"
port = 8080
workers = 2
timeout = 30

[database]
url = "dev_data.db"
max_connections = 5
timeout = 30

[auth]
jwt_secret = "dev-secret-key"
jwt_expiration = 86400
bcrypt_cost = 10

[file]
upload_path = "uploads/dev"
max_file_size = 5242880  # 5MB
allowed_extensions = ["zip", "rar"]
temp_path = "temp/dev"

[logging]
level = "debug"
file_path = "logs/dev.log"
max_files = 3
max_size = 5242880  # 5MB
```

### 生产环境配置

```toml
[server]
host = "0.0.0.0"
port = 80
workers = 8
timeout = 60

[database]
url = "/var/lib/rope-manager/prod.db"
max_connections = 20
timeout = 60

[auth]
jwt_secret = "production-secret-key-very-long-and-secure"
jwt_expiration = 3600  # 1小时
bcrypt_cost = 14

[file]
upload_path = "/var/lib/rope-manager/uploads"
max_file_size = 52428800  # 50MB
allowed_extensions = ["zip", "rar", "7z", "tar", "gz"]
temp_path = "/tmp/rope-manager"

[logging]
level = "warn"
file_path = "/var/log/rope-manager/app.log"
max_files = 10
max_size = 104857600  # 100MB
```

## 🔒 安全配置建议

### 生产环境安全设置

1. **JWT密钥**：使用强密钥，至少32字符
2. **bcrypt成本**：设置为12-14
3. **文件上传**：限制文件大小和类型
4. **CORS**：限制允许的源
5. **日志级别**：生产环境使用warn或error

### 环境变量安全设置

```bash
# 生产环境环境变量
export JWT_SECRET="your-very-long-and-secure-secret-key"
export LOG_LEVEL="warn"
export HOST="0.0.0.0"
export PORT="80"
export MAX_FILE_SIZE="52428800"
```

## 🛠️ 配置验证

### 验证配置文件语法

```powershell
.\config_manager.ps1 test
```

### 验证环境变量

```powershell
.\config_manager.ps1 env
```

### 验证配置加载

启动服务时，日志会显示加载的配置信息。

## 📝 配置最佳实践

1. **开发环境**：使用配置文件，便于版本控制
2. **生产环境**：使用环境变量，提高安全性
3. **敏感信息**：使用环境变量，避免提交到代码库
4. **配置备份**：定期备份配置文件
5. **配置验证**：部署前验证配置正确性

## 🔄 配置热重载

当前版本不支持配置热重载，修改配置需要重启服务。

## 📊 配置监控

可以通过以下方式监控配置：

1. **日志监控**：查看启动日志中的配置信息
2. **健康检查**：通过 `/health` 端点检查服务状态
3. **配置验证**：定期运行配置验证脚本

## 🆘 故障排除

### 常见配置问题

1. **配置文件不存在**：运行 `.\config_manager.ps1 new`
2. **配置文件语法错误**：运行 `.\config_manager.ps1 test`
3. **权限问题**：检查文件和目录权限
4. **端口被占用**：修改 `PORT` 配置
5. **数据库连接失败**：检查 `DATABASE_URL` 路径

### 调试配置

```powershell
# 显示详细配置信息
.\config_manager.ps1 show

# 检查环境变量
.\config_manager.ps1 env

# 测试配置加载
cargo run -- --config-debug
``` 
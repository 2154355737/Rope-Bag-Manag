# 绳包管理器后端服务

这是一个基于 Rust + Actix-web 的绳包管理器后端服务。

## 功能特性

- 🔐 用户认证和授权
- 📦 绳包管理（上传、下载、分类）
- 💬 社区评论系统
- 👥 用户管理
- 🛠️ 管理员后台
- 📊 数据统计

## 技术栈

- **框架**: Actix-web
- **数据库**: SQLite
- **认证**: JWT
- **密码加密**: bcrypt
- **序列化**: serde
- **日志**: log + env_logger

## 快速开始

### 1. 环境要求

- Rust 1.70+
- Windows/Linux/macOS

### 2. 安装依赖

```bash
cargo build
```

### 3. 配置管理

项目支持多种配置方式：

#### 方式一：配置文件（推荐）
```bash
# 创建默认配置文件
powershell -ExecutionPolicy Bypass -File config_manager.ps1 new

# 编辑配置文件
powershell -ExecutionPolicy Bypass -File config_manager.ps1 edit

# 查看当前配置
powershell -ExecutionPolicy Bypass -File config_manager.ps1 show
```

#### 方式二：环境变量
```bash
# 设置环境变量
set HOST=127.0.0.1
set PORT=8080
set DATABASE_URL=data.db
set JWT_SECRET=your-secret-key
set UPLOAD_PATH=uploads
```

#### 方式三：混合模式
配置文件优先级高于环境变量，环境变量优先级高于默认值。

### 4. 启动服务

#### 方法一：使用批处理脚本（推荐）
```bash
# Windows批处理
start.bat

# 或使用PowerShell脚本（支持中文显示）
powershell -ExecutionPolicy Bypass -File start.ps1
```

#### 方法二：直接运行
```bash
# 开发模式
cargo run

# 发布模式
cargo run --release
```

服务将在 `http://127.0.0.1:8080` 启动。

### 5. 测试API

运行测试脚本：

```bash
# 基础测试（批处理）
test_api.bat

# 基础测试（PowerShell，推荐）
powershell -ExecutionPolicy Bypass -File test_api.ps1

# 高级测试（PowerShell，包含详细报告）
powershell -ExecutionPolicy Bypass -File test_api_advanced.ps1
```

测试脚本会自动从配置文件读取端口设置，支持中文显示和详细的测试报告。

## API 端点

### 认证相关

- `POST /api/v1/auth/register` - 用户注册
- `POST /api/v1/auth/login` - 用户登录
- `GET /api/v1/auth/user-info` - 获取用户信息

### 绳包管理

- `GET /api/v1/packages` - 获取绳包列表
- `GET /api/v1/packages/{id}` - 获取绳包详情
- `POST /api/v1/packages` - 创建绳包
- `PUT /api/v1/packages/{id}` - 更新绳包
- `DELETE /api/v1/packages/{id}` - 删除绳包

### 用户管理

- `GET /api/v1/users` - 获取用户列表
- `GET /api/v1/users/{id}` - 获取用户详情
- `PUT /api/v1/users/{id}` - 更新用户信息
- `DELETE /api/v1/users/{id}` - 删除用户

### 评论系统

- `GET /api/v1/comments` - 获取评论列表
- `POST /api/v1/comments` - 创建评论
- `PUT /api/v1/comments/{id}` - 更新评论
- `DELETE /api/v1/comments/{id}` - 删除评论

### 管理员功能

- `GET /api/v1/admin/stats` - 获取统计数据
- `GET /api/v1/admin/logs` - 获取系统日志
- `POST /api/v1/admin/backup` - 创建数据备份

## 项目结构

```
src/
├── main.rs              # 应用入口
├── config.rs            # 配置管理
├── models/              # 数据模型
│   ├── user.rs         # 用户模型
│   ├── package.rs      # 绳包模型
│   └── comment.rs      # 评论模型
├── api/                 # API路由
│   └── v1/             # API版本1
│       ├── auth.rs     # 认证API
│       ├── packages.rs # 绳包API
│       ├── users.rs    # 用户API
│       └── admin.rs    # 管理员API
├── services/            # 业务逻辑层
│   ├── auth_service.rs # 认证服务
│   ├── user_service.rs # 用户服务
│   └── package_service.rs # 绳包服务
├── repositories/        # 数据访问层
│   ├── user_repo.rs    # 用户仓库
│   ├── package_repo.rs # 绳包仓库
│   └── comment_repo.rs # 评论仓库
├── middleware/          # 中间件
│   ├── auth.rs         # 认证中间件
│   ├── cors.rs         # CORS中间件
│   └── logging.rs      # 日志中间件
└── utils/              # 工具函数
    ├── jwt.rs          # JWT工具
    ├── password.rs     # 密码工具
    └── file.rs         # 文件工具
```

## 数据库

项目使用 SQLite 数据库，数据库文件位于 `data.db`。

初始化SQL脚本位于 `sql/init.sql`，包含：

- 用户表
- 绳包表
- 评论表
- 分类表
- 系统配置表

## 开发

### 添加新功能

1. 在 `models/` 中定义数据模型
2. 在 `repositories/` 中实现数据访问
3. 在 `services/` 中实现业务逻辑
4. 在 `api/v1/` 中定义API端点
5. 在 `main.rs` 中注册路由

### 运行测试

```bash
cargo test
```

### 代码格式化

```bash
cargo fmt
```

### 代码检查

```bash
cargo clippy
```

## 部署

### 生产环境

1. 编译发布版本：
   ```bash
   cargo build --release
   ```

2. 配置生产环境变量

3. 使用进程管理器（如 systemd）运行服务

### Docker 部署

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rope-manager-backend /usr/local/bin/
EXPOSE 8080
CMD ["rope-manager-backend"]
```

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！ 
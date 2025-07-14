# 绳包管理器后端

基于 Rust + Actix-web 的绳包资源管理系统后端服务。

## 🚀 功能特性

- **用户认证**: JWT token 认证系统
- **用户管理**: 用户注册、登录、权限管理
- **绳包管理**: 绳包资源的上传、下载、管理
- **社区功能**: 评论、点赞、收藏
- **管理员后台**: 数据统计、用户管理、系统设置
- **文件管理**: 安全的文件上传和下载

## 🛠️ 技术栈

- **后端框架**: Rust + Actix-web
- **数据库**: SQLite
- **认证**: JWT Token
- **密码加密**: bcrypt
- **文件处理**: 本地文件系统

## 📁 项目结构

```
src/
├── main.rs                 # 应用入口
├── config.rs              # 配置管理
├── models/                # 数据模型
│   ├── mod.rs
│   ├── user.rs           # 用户模型
│   ├── package.rs        # 绳包模型
│   ├── comment.rs        # 评论模型
│   └── system.rs         # 系统模型
├── api/                  # API路由
│   ├── mod.rs
│   └── v1/              # API版本
│       ├── mod.rs
│       ├── auth.rs      # 认证API
│       ├── user.rs      # 用户API
│       ├── package.rs   # 绳包API
│       ├── admin.rs     # 管理API
│       └── community.rs # 社区API
├── services/            # 业务逻辑
│   ├── mod.rs
│   ├── auth_service.rs
│   ├── user_service.rs
│   ├── package_service.rs
│   └── admin_service.rs
├── repositories/        # 数据访问
│   ├── mod.rs
│   ├── user_repo.rs
│   ├── package_repo.rs
│   └── system_repo.rs
├── middleware/          # 中间件
│   ├── mod.rs
│   ├── auth.rs         # 认证中间件
│   ├── cors.rs         # CORS中间件
│   └── logging.rs      # 日志中间件
├── utils/              # 工具函数
│   ├── mod.rs
│   ├── jwt.rs          # JWT工具
│   ├── password.rs     # 密码加密
│   └── file.rs         # 文件处理
└── sql/                # SQL文件
    └── init.sql        # 数据库初始化
```

## 🚀 快速开始

### 环境要求

- Rust 1.70+
- Cargo

### 安装和运行

1. **克隆项目**
```bash
git clone <repository-url>
cd rope-manager-backend
```

2. **安装依赖**
```bash
cargo build
```

3. **运行项目**
```bash
cargo run
```

4. **访问API**
- 服务地址: http://localhost:8080
- API文档: http://localhost:8080/api/v1

## 📋 API接口

### 认证相关

- `POST /api/v1/auth/login` - 用户登录
- `POST /api/v1/auth/register` - 用户注册
- `GET /api/v1/auth/user-info` - 获取用户信息

### 用户管理

- `GET /api/v1/users` - 获取用户列表
- `GET /api/v1/users/{id}` - 获取单个用户
- `PUT /api/v1/users/{id}` - 更新用户信息
- `DELETE /api/v1/users/{id}` - 删除用户

### 绳包管理

- `GET /api/v1/packages` - 获取绳包列表
- `GET /api/v1/packages/{id}` - 获取单个绳包
- `POST /api/v1/packages` - 创建绳包
- `PUT /api/v1/packages/{id}` - 更新绳包
- `DELETE /api/v1/packages/{id}` - 删除绳包
- `GET /api/v1/packages/{id}/download` - 下载绳包

### 社区功能

- `GET /api/v1/community/comments/{package_id}` - 获取评论
- `POST /api/v1/community/comments/{package_id}` - 发表评论

### 管理员功能

- `GET /api/v1/admin/stats` - 获取统计数据
- `GET /api/v1/admin/categories` - 获取分类列表
- `GET /api/v1/admin/user-actions` - 获取用户行为记录

## 🔧 配置

通过环境变量配置：

```bash
# 服务器配置
HOST=127.0.0.1
PORT=8080

# 数据库配置
DATABASE_URL=data/rope_manager.db

# JWT配置
JWT_SECRET=your-secret-key

# 文件上传配置
UPLOAD_PATH=uploads
MAX_FILE_SIZE=10485760  # 10MB
```

## 🔐 默认账户

- **管理员账户**
  - 用户名: `admin`
  - 密码: `admin123`

## 📊 数据库

项目使用 SQLite 数据库，数据库文件位于 `data/rope_manager.db`。

### 主要表结构

- `users` - 用户表
- `packages` - 绳包表
- `comments` - 评论表
- `categories` - 分类表
- `user_actions` - 用户行为日志表

## 🛡️ 安全特性

- JWT Token 认证
- bcrypt 密码加密
- CORS 跨域配置
- 文件上传安全检查
- SQL 注入防护

## 📝 开发说明

### 添加新的API

1. 在 `src/api/v1/` 下创建新的路由文件
2. 在 `src/services/` 下创建对应的服务
3. 在 `src/repositories/` 下创建数据访问层
4. 在 `src/models/` 下定义数据模型

### 数据库迁移

修改 `src/sql/init.sql` 文件，然后重新运行应用。

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## �� 许可证

MIT License 
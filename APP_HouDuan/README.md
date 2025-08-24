# 绳包管理器后端服务 v2.0

## 项目概述

这是绳包管理器的全新后端实现，采用现代化的Rust架构设计，提供高性能、高可靠性的API服务。

## 技术栈

- **框架**: Actix-Web 4.x
- **数据库**: SQLite with SQLx
- **认证**: JWT
- **日志**: Tracing
- **配置**: Config crate
- **缓存**: Redis
- **验证**: Validator

## 架构特性

- 🏗️ **清晰的分层架构**: Domain、Infrastructure、API三层分离
- 🔧 **依赖注入**: 统一的服务容器管理
- ⚡ **高性能**: SQLite连接池、Redis缓存
- 🛡️ **类型安全**: 完整的Rust类型系统
- 📝 **完善日志**: 结构化日志记录
- 🔒 **安全设计**: JWT认证、权限控制

## 项目结构

```
src/
├── core/           # 核心业务逻辑
│   ├── domain/     # 领域模型
│   ├── services/   # 业务服务
│   └── ports/      # 接口定义
├── infrastructure/ # 基础设施
│   ├── database/   # 数据库相关
│   ├── cache/      # 缓存实现
│   ├── storage/    # 文件存储
│   └── email/      # 邮件服务
├── api/           # API 层
│   ├── handlers/   # 请求处理器
│   ├── middleware/ # 中间件
│   └── routes/     # 路由定义
└── shared/        # 共享组件
    ├── errors/     # 错误定义
    ├── utils/      # 工具函数
    └── types/      # 通用类型
```

## 快速开始

### 环境要求

- Rust 1.70+
- SQLite 3.35+
- Redis 6.0+ (可选)

### 安装依赖

```bash
# 安装Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 克隆项目
git clone <repository-url>
cd APP_HouDuan
```

### 配置环境

```bash
# 复制环境变量文件
cp .env.example .env

# 编辑配置文件
vim .env
```

### 运行项目

```bash
# 开发模式
cargo run

# 发布模式
cargo run --release
```

### 数据库迁移

数据库会在首次启动时自动初始化和迁移。

## API文档

启动服务后，访问以下端点：

- 健康检查: `GET /health`
- API基础路径: `/api/v1`

### 主要功能模块

1. **用户管理** (`/api/v1/users`)
   - 用户注册、登录
   - 用户资料管理
   - 权限控制

2. **资源包管理** (`/api/v1/packages`)
   - 资源上传、下载
   - 资源搜索、分类
   - 版本管理

3. **评论系统** (`/api/v1/comments`)
   - 评论发布、回复
   - 点赞、举报

4. **通知系统** (`/api/v1/notifications`)
   - 消息推送
   - 邮件通知

## 配置说明

### 环境变量

| 变量名 | 说明 | 默认值 |
|--------|------|--------|
| `RUN_MODE` | 运行模式 | `development` |
| `APP_SERVER__HOST` | 服务器地址 | `127.0.0.1` |
| `APP_SERVER__PORT` | 服务器端口 | `15201` |
| `APP_DATABASE__URL` | 数据库连接 | `sqlite:./data/app.db` |

### 配置文件

- `config/default.toml` - 默认配置
- `config/development.toml` - 开发环境
- `config/production.toml` - 生产环境

## 开发指南

### 代码规范

- 遵循Rust官方代码风格
- 使用`cargo fmt`格式化代码
- 使用`cargo clippy`检查代码质量

### 测试

```bash
# 运行测试
cargo test

# 运行测试并显示输出
cargo test -- --nocapture
```

### 数据库操作

数据库迁移文件位于`migrations/`目录下，按序号命名。

## 部署

### Docker部署

```bash
# 构建镜像
docker build -t jieshen-backend .

# 运行容器
docker run -p 15201:15201 jieshen-backend
```

### 系统服务

创建systemd服务文件以在生产环境中运行。

## 性能优化

- SQLite连接池优化
- Redis缓存策略
- 静态文件CDN
- 数据库索引优化

## 安全特性

- JWT Token认证
- 密码BCrypt加密
- SQL注入防护
- XSS防护
- CORS配置

## 监控和日志

- 结构化日志输出
- 性能指标收集
- 错误追踪
- 健康检查端点

## 贡献指南

1. Fork项目
2. 创建功能分支
3. 提交更改
4. 创建Pull Request

## 许可证

MIT License

## 更新日志

### v2.0.0
- 全新架构设计
- 重构核心业务逻辑
- 优化数据库性能
- 完善错误处理 
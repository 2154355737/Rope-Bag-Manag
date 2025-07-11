# 绳包管理器 (Rope Package Manager)

[![Rust](https://img.shields.io/badge/Rust-1.70+-blue.svg)](https://www.rust-lang.org/)
[![Actix Web](https://img.shields.io/badge/Actix%20Web-4.4-green.svg)](https://actix.rs/)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## 📖 项目介绍

绳包管理器是一个基于 Rust 和 Actix Web 框架开发的现代化包管理系统。该系统提供了完整的用户管理、包管理、权限控制和统计分析功能，适用于各种包分发和管理场景。

### 🎯 主要特性

- **🔐 用户管理系统**: 支持用户注册、登录、权限管理
- **📦 包管理功能**: 绳包的添加、删除、下载统计
- **👑 管理员权限**: 完整的后台管理功能
- **📊 数据统计**: API调用统计、下载量统计
- **🛡️ 安全机制**: 限流保护、日志记录、会话管理
- **📝 日志系统**: 完整的请求日志和性能监控
- **⚙️ 配置管理**: 灵活的配置文件系统，支持热重载

## 🚀 快速开始

### 环境要求

- Rust 1.70+ 
- Windows/Linux/macOS

### 安装步骤

1. **克隆项目**
```bash
git clone <repository-url>
cd rust
```

2. **编译项目**
```bash
cargo build --release
```

3. **启动服务**
```bash
# Windows
start_server.bat

# Linux/macOS
cargo run --release
```

4. **访问服务**
```
服务地址: http://127.0.0.1:15201
```

## 📋 功能模块

### 用户管理
- ✅ 用户注册与登录
- ✅ 用户信息查询
- ✅ 密码和昵称修改
- ✅ 用户签到系统
- ✅ 用户星级管理
- ✅ 用户封禁/解封

### 包管理
- ✅ 绳包添加与删除
- ✅ 包信息查询
- ✅ 下载统计
- ✅ 版本管理
- ✅ 作者信息

### 管理员功能
- ✅ 用户管理
- ✅ 包管理
- ✅ 权限设置
- ✅ 数据统计
- ✅ 日志查看

### 系统功能
- ✅ 请求限流
- ✅ 日志记录
- ✅ 性能监控
- ✅ 配置管理
- ✅ 错误处理

## 🔧 配置说明

项目使用多个配置文件来管理不同功能：

- `config.json` - 主配置文件（服务器、日志、安全设置）
- `users.json` - 用户数据文件
- `data.json` - 绳包数据文件
- `stats.json` - 统计数据文件

详细配置说明请参考 [配置文件说明文档](docs/配置文件说明.md)

## 📚 API 文档

### 基础信息
- **服务地址**: `http://127.0.0.1:15201`
- **协议**: HTTP GET
- **数据格式**: JSON

### API统一返回结构

所有API接口返回如下JSON结构：

```
{
  "code": 0, // 0为成功，1为失败
  "msg": "操作结果描述",
  "data": { ... } // 有数据时返回
}
```
- 错误时`data`为`null`，并有详细错误信息。
- 成功时`data`为实际数据或`null`。

### 主要接口

#### 用户管理
- `GET /api/register` - 用户注册
- `GET /api/login` - 用户登录
- `GET /api/user-info` - 查询用户信息
- `GET /api/sign-in` - 用户签到
- `GET /api/change-password` - 修改密码
- `GET /api/change-nickname` - 修改昵称

#### 包管理
- `GET /api/add-rope-package` - 添加绳包
- `GET /api/download-rope-package` - 下载统计
- `GET /api/get-data-db` - 获取包数据

#### 管理员功能
- `GET /api/admin/user-info` - 管理员查询用户
- `GET /api/admin/set-user` - 设置用户信息
- `GET /api/admin/set-star` - 设置用户星级
- `GET /api/admin/ban-user` - 封禁/解封用户
- `GET /api/admin/add-rope-package` - 管理员添加包
- `GET /api/admin/delete-rope-package` - 管理员删除包

#### 统计功能
- `GET /api/stats/downloads` - 下载统计
- `GET /api/stats/api-counts` - API调用统计

完整API文档请参考 [接口文档](docs/接口文档.md)

## 📁 项目结构

```
rust/
├── src/
│   ├── main.rs              # 主程序入口
│   ├── config.rs            # 配置管理
│   ├── auth.rs              # 认证模块
│   ├── models.rs            # 数据模型
│   ├── storage.rs           # 数据存储
│   ├── logger.rs            # 日志系统
│   ├── utils.rs             # 工具函数
│   ├── cmd.rs               # 命令处理
│   └── handlers/            # 请求处理器
│       ├── mod.rs
│       ├── login.rs         # 登录处理
│       ├── register.rs      # 注册处理
│       ├── user.rs          # 用户管理
│       ├── package.rs       # 包管理
│       ├── admin.rs         # 管理员功能
│       ├── stats.rs         # 统计功能
│       └── logs.rs          # 日志功能
├── data/                    # 数据文件
│   ├── config.json          # 配置文件
│   ├── users.json           # 用户数据
│   ├── data.json            # 包数据
│   └── stats.json           # 统计数据
├── docs/                    # 文档
│   ├── 接口文档.md
│   ├── 配置文件说明.md
│   └── 日志系统说明.md
├── logs/                    # 日志文件
├── Cargo.toml              # 项目配置
└── README.md               # 项目说明
```

## 🛠️ 开发指南

### 添加新功能

1. 在 `handlers/` 目录下创建新的处理器
2. 在 `main.rs` 中注册新的路由
3. 更新相关文档

### 测试

```bash
# 运行测试
cargo test

# 运行特定测试
cargo test test_name
```

### 日志查看

```bash
# 查看实时日志
tail -f logs/app.log
```

## 📊 性能特性

- **高并发**: 基于 Actix Web 的异步处理
- **内存安全**: Rust 语言的内存安全保证
- **零成本抽象**: 编译时优化
- **热重载**: 配置文件支持热重载
- **限流保护**: 防止恶意请求

## 🤝 贡献指南

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 📞 联系方式

如有问题或建议，请通过以下方式联系：
- QQ 2154355737
- 提交 Issue
- 发送邮件 21543557372@qq.com
- 参与讨论 Muteduaxning

---

**绳包管理器** - 让包管理变得简单高效！ 🚀

---

## 📝 更新日志

### v1.0.2
- 数据库结构与API返回结构完全统一为RawDataJson，所有增删改查API与前端结构一致，便于前端索引和数据同步。
- `/api/get-data-db`接口直接返回完整数据库结构，字段与data.json完全一致。
- 修复了参数名、URL编码、数据同步等兼容性问题。

### v1.1.x
- 所有API接口返回结构统一为`ApiResponse<T>`，包含`code`、`msg`、`data`字段，便于前端和第三方系统统一处理。
- 终端管理员指令（add-rope、delete-rope）无需密码，简化本地管理。
- 新增/优化了用户数据库重载、服务安全关闭、服务重启等命令行指令。
- 优化了管理员操作的安全校验。
- 其它细节优化与bug修复。

---

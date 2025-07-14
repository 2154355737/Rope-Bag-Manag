# 前后端对接说明

## 概述

本文档说明如何将Vue前端与Rust后端进行对接，实现完整的绳包管理系统。

## 项目结构

```
rust/
├── rope-manager-backend/     # Rust后端
│   ├── src/
│   ├── config.toml          # 后端配置
│   └── ...
├── Rust_Vue/                # Vue前端
│   ├── src/
│   │   ├── api/             # API服务
│   │   ├── utils/           # 工具函数
│   │   └── views/           # 页面组件
│   └── ...
└── start_fullstack.bat      # 全栈启动脚本
```

## 后端配置

### 1. CORS配置

后端已配置CORS支持前端访问：

```rust
let cors = Cors::default()
    .allowed_origin("http://localhost:5173")  // Vite默认端口
    .allowed_origin("http://127.0.0.1:5173")
    .allowed_origin("http://localhost:3000")  // 备用端口
    .allowed_origin("http://127.0.0.1:3000")
    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
    .allowed_headers(vec![
        http::header::AUTHORIZATION,
        http::header::CONTENT_TYPE,
        http::header::ACCEPT,
    ])
    .max_age(3600);
```

### 2. API端点

后端提供以下API端点：

- `GET /health` - 健康检查
- `POST /api/v1/auth/login` - 用户登录
- `POST /api/v1/auth/register` - 用户注册
- `GET /api/v1/auth/user-info` - 获取用户信息
- `GET /api/v1/users` - 获取用户列表
- `GET /api/v1/packages` - 获取绳包列表
- `GET /api/v1/admin/stats` - 获取统计数据
- `GET /api/v1/admin/logs` - 获取系统日志
- `GET /api/v1/admin/user-actions` - 获取用户行为

## 前端配置

### 1. API客户端

前端使用axios作为HTTP客户端，配置在 `src/utils/apiClient.ts`：

```typescript
const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://127.0.0.1:15201'

const apiClient: AxiosInstance = axios.create({
  baseURL: API_BASE_URL,
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
})
```

### 2. 环境配置

创建 `.env` 文件配置API地址：

```env
VITE_API_BASE_URL=http://127.0.0.1:15201
VITE_APP_TITLE=绳包管理器
VITE_APP_VERSION=1.0.0
```

### 3. API服务模块

前端API服务按功能模块组织：

- `src/api/auth.ts` - 认证相关API
- `src/api/users.ts` - 用户管理API
- `src/api/packages.ts` - 绳包管理API
- `src/api/admin.ts` - 管理员API

## 启动流程

### 1. 使用全栈启动脚本

```bash
# Windows批处理
start_fullstack.bat

# PowerShell
.\start_fullstack.ps1
```

### 2. 手动启动

```bash
# 1. 启动后端
cd rope-manager-backend
cargo run

# 2. 启动前端
cd Rust_Vue
npm run dev
```

## API对接示例

### 1. 用户登录

```typescript
// 前端调用
import { authApi } from '@/api'

const response = await authApi.login({
  username: 'admin',
  password: 'admin123'
})

if (response.code === 0) {
  // 登录成功
  setToken(response.data.token)
  router.push('/admin')
}
```

### 2. 获取用户列表

```typescript
// 前端调用
import { userApi } from '@/api'

const response = await userApi.getUsers({
  page: 1,
  pageSize: 20,
  role: 'admin'
})

if (response.code === 0) {
  // 处理用户列表数据
  const users = response.data.list
}
```

### 3. 获取统计数据

```typescript
// 前端调用
import { adminApi } from '@/api'

const response = await adminApi.getStats()

if (response.code === 0) {
  // 处理统计数据
  const stats = response.data
}
```

## 测试对接

### 1. API测试页面

访问 `http://localhost:5173/admin/api-test` 进行API测试。

### 2. 健康检查

```bash
curl http://127.0.0.1:15201/health
```

### 3. 登录测试

```bash
curl -X POST http://127.0.0.1:15201/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin123"}'
```

## 常见问题

### 1. CORS错误

确保后端CORS配置正确，前端地址在允许列表中。

### 2. 连接失败

- 检查后端服务是否启动
- 确认端口配置正确
- 检查防火墙设置

### 3. 认证失败

- 确认用户名密码正确
- 检查JWT token配置
- 查看后端日志

### 4. 数据格式不匹配

- 检查API响应格式
- 确认TypeScript类型定义
- 查看网络请求详情

## 开发建议

### 1. 开发环境

- 使用 `start_fullstack.bat` 一键启动
- 开启浏览器开发者工具查看网络请求
- 使用Vue DevTools调试前端状态

### 2. 调试技巧

- 后端日志查看：`cargo run` 控制台输出
- 前端网络请求：浏览器Network面板
- API测试：使用内置测试页面

### 3. 部署准备

- 生产环境修改API地址
- 配置HTTPS证书
- 设置环境变量

## 下一步

1. 完善用户界面
2. 添加更多功能模块
3. 优化性能
4. 添加单元测试
5. 部署到生产环境 
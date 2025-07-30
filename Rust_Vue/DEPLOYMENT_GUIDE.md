# 🚀 前端部署配置指南

## 📋 概述

本项目支持两种运行模式，均通过代理方式访问后端API，确保开发和生产环境的一致性。

### 🔄 两种运行模式

| 模式 | 前端服务 | API代理 | 访问地址 |
|------|----------|---------|----------|
| **开发模式** | Vite开发服务器 (5173) | Vite代理 `/api` → `127.0.0.1:15201` | `http://localhost:5173` |
| **生产模式** | nginx静态文件服务 (80) | nginx代理 `/api/` → `127.0.0.1:15201` | `http://服务器IP:80` |

## ⚙️ 环境配置

### 1. 配置文件说明

```bash
# 环境配置文件
├── env.example        # 配置示例和说明
├── env.development    # 开发环境配置模板
└── env.production     # 生产环境配置模板
```

### 2. 快速配置

```bash
# 开发环境
cp env.development .env.development

# 生产环境  
cp env.production .env.production

# 通用配置（可选）
cp env.example .env
```

### 3. 配置说明

#### 推荐配置（使用代理）
```bash
# 不设置或注释掉 VITE_API_BASE_URL
# VITE_API_BASE_URL=/api

# 系统默认使用 /api，通过代理访问后端
```

#### 备选配置（直接访问）
```bash
# 开发环境
VITE_API_BASE_URL=http://127.0.0.1:15201/api

# 生产环境
VITE_API_BASE_URL=http://39.105.113.219:15201/api
```

## 🔧 开发模式配置

### Vite代理配置 (已配置)

```typescript
// vite.config.ts
server: {
  proxy: {
    '/api': {
      target: 'http://127.0.0.1:15201',
      changeOrigin: true,
      rewrite: (path) => path  // 保持 /api 前缀
    },
    '/uploads': {
      target: 'http://127.0.0.1:15201',
      changeOrigin: true,
      rewrite: (path) => path
    }
  }
}
```

### 启动开发服务器

```bash
# 安装依赖
pnpm install

# 启动开发服务器
pnpm dev

# 访问地址
# http://localhost:5173
```

## 🏭 生产模式配置

### 1. nginx配置 (需要添加uploads代理)

```nginx
server {
    listen 80;
    server_name localhost;

    # 前端静态文件
    location / {
        root   C:/Users/Administrator/Desktop/nginx-1.28.0/nginx-1.28.0/html;
        index  index.html;
        try_files $uri $uri/ /index.html;
    }

    # API代理
    location /api/ {
        proxy_pass         http://127.0.0.1:15201;
        proxy_set_header   Host $host;
        proxy_set_header   X-Real-IP $remote_addr;
        proxy_set_header   X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header   X-Forwarded-Proto $scheme;
    }
    
    # 文件上传/下载代理 (重要：需要添加)
    location /uploads/ {
        proxy_pass         http://127.0.0.1:15201;
        proxy_set_header   Host $host;
        proxy_set_header   X-Real-IP $remote_addr;
        proxy_set_header   X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header   X-Forwarded-Proto $scheme;
    }
}
```

### 2. 构建和部署

```bash
# 构建生产版本
pnpm build

# 复制构建文件到nginx目录
cp -r dist/* C:/Users/Administrator/Desktop/nginx-1.28.0/nginx-1.28.0/html/

# 或使用PowerShell
Copy-Item -Recurse -Force "dist\*" "C:\Users\Administrator\Desktop\nginx-1.28.0\nginx-1.28.0\html\"
```

## 🛠️ 后端配置

### CORS配置 (已配置)

确保后端允许前端域名访问：

```rust
// main.rs (已更新)
.allowed_origin("http://localhost:5173")     // 开发环境
.allowed_origin("http://127.0.0.1:5173")
.allowed_origin("http://39.105.113.219")    // 生产环境
.allowed_origin("https://39.105.113.219")
```

### 启动后端服务

```bash
cd rope-manager-backend
cargo run --release
```

## 🔍 调试和验证

### 1. 查看API配置

打开浏览器控制台，应该看到：

```
🔧 API Client Configuration
🔗 Final API Base URL: /api
🌍 Environment Mode: development
🚀 Development Mode: Using Vite proxy
   • /api -> http://127.0.0.1:15201
   • /uploads -> http://127.0.0.1:15201
```

### 2. 验证API调用

```bash
# 开发环境测试
curl http://localhost:5173/api/health

# 生产环境测试  
curl http://39.105.113.219/api/health
```

### 3. 常见问题排查

#### API 404错误
- 检查后端是否启动 (127.0.0.1:15201)
- 检查nginx代理配置
- 检查CORS配置

#### 文件上传失败
- 确保nginx配置了 `/uploads/` 代理
- 检查后端uploads目录权限

#### 开发模式API调用失败
- 检查Vite代理配置
- 确认后端端口15201

## 🎯 最佳实践

1. **推荐使用代理模式**：保持开发和生产环境一致
2. **环境变量管理**：不同环境使用对应的配置文件
3. **调试信息**：开发时启用 `VITE_DEBUG_API=true`
4. **HTTPS部署**：生产环境建议使用HTTPS

## 🚨 注意事项

1. ⚠️ **nginx配置**：确保添加了 `/uploads/` 代理
2. ⚠️ **后端地址**：确保后端绑定 `0.0.0.0:15201` 而不是 `127.0.0.1:15201`
3. ⚠️ **环境变量**：生产构建时确保使用正确的配置文件
4. ⚠️ **CORS配置**：确保后端允许前端域名访问 
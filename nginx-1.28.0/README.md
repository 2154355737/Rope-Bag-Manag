# Nginx 服务管理工具

## 📋 文件说明

### 🔧 管理脚本
- **`nginx_manager.bat`** - 完整的nginx管理工具（推荐使用）
- **`start_nginx.bat`** - 快速启动nginx（包含配置检查）
- **`stop_nginx.bat`** - 快速停止nginx
- **`deploy_frontend.bat`** - 前端应用部署脚本

### ⚙️ 配置文件
- **`conf/nginx.conf`** - nginx主配置文件（已优化Windows兼容性）
- **`html/50x.html`** - 自定义错误页面

## 🚀 快速开始

### 方法一：使用完整管理工具（推荐）
双击运行 `nginx_manager.bat`，选择相应操作：

```
===============================================
           Nginx 服务管理脚本
===============================================

请选择操作:
[1] 启动 Nginx
[2] 停止 Nginx  
[3] 重启 Nginx
[4] 重载配置
[5] 检查状态
[6] 测试配置
[7] 查看日志
[8] 清理日志
[0] 退出
```

### 方法二：使用快捷脚本
- 启动服务：双击 `start_nginx.bat`
- 停止服务：双击 `stop_nginx.bat`
- 部署前端：双击 `deploy_frontend.bat`

### 方法三：命令行操作
```bash
# 启动
nginx_manager.bat start

# 停止  
nginx_manager.bat stop

# 重启
nginx_manager.bat restart

# 检查状态
nginx_manager.bat status

# 重载配置
nginx_manager.bat reload
```

## 🔧 前端应用部署

### 自动部署（推荐）
1. 确保前端应用已构建：
   ```bash
   cd ..\Rust_Vue
   npm run build
   ```

2. 运行部署脚本：
   ```bash
   deploy_frontend.bat
   ```

### 手动部署
将前端构建文件（`../Rust_Vue/dist/*`）复制到 `html/` 目录

## 🔍 功能特性

### ✅ 完整的服务管理
- 启动/停止/重启nginx服务
- 配置文件测试和重载
- 服务状态检查和健康监测
- 日志查看和清理

### ⚡ 性能优化
- 自动检测CPU核心数优化worker进程
- 启用gzip压缩减少传输大小
- 静态资源缓存配置
- 代理缓冲优化

### 🛡️ 安全配置
- 错误页面自定义
- 文件上传大小限制
- 代理超时保护
- 访问日志记录

### 🔄 代理配置
- **前端静态文件**: `http://localhost/` → `html/` 目录
- **API接口代理**: `http://localhost/api/` → `http://127.0.0.1:15201`
- **文件上传下载**: `http://localhost/uploads/` → `http://127.0.0.1:15201`
- **健康检查**: `http://localhost/health`

### 🖥️ Windows兼容性
- ✅ 已移除Linux特有的 `epoll` 事件模型
- ✅ 已移除Windows不支持的 `tcp_nopush` 指令
- ✅ 使用Windows默认的 `select` 事件模型
- ✅ 路径配置适配Windows环境

## 📊 状态检查

### 健康检查
访问 `http://localhost/health` 查看nginx状态

### 端口检查
nginx默认监听80端口，确保没有其他程序占用

### 日志查看
- 访问日志：`logs/access.log`
- 错误日志：`logs/error.log`

## ⚠️ 故障排除

### 启动失败
1. 检查端口80是否被占用
2. 运行配置测试：`nginx_manager.bat test`
3. 查看错误日志：`nginx_manager.bat logs`

### 常见问题
- **端口占用**: 关闭占用80端口的程序（如IIS、Apache等）
- **权限问题**: 以管理员身份运行脚本
- **路径错误**: 确保nginx.exe在当前目录
- **配置错误**: 运行 `start_nginx.bat` 会自动检查配置

### Windows特有问题
- **事件模型错误**: 已修复，不再使用Linux的epoll
- **指令不支持**: 已移除Windows不支持的指令
- **路径格式**: 使用反斜杠或正斜杠都可以

### 配置文件问题
运行 `nginx_manager.bat test` 检查配置文件语法

## 📁 目录结构

```
nginx-1.28.0/
├── nginx.exe                 # nginx主程序
├── nginx_manager.bat         # 完整管理脚本
├── start_nginx.bat          # 快速启动
├── stop_nginx.bat           # 快速停止
├── deploy_frontend.bat      # 前端部署脚本
├── README.md                # 使用说明
├── conf/
│   └── nginx.conf           # 主配置文件（Windows优化）
├── html/
│   ├── index.html           # 前端应用文件（部署后）
│   └── 50x.html            # 错误页面
├── logs/
│   ├── access.log           # 访问日志
│   ├── error.log            # 错误日志
│   └── nginx.pid            # 进程ID文件
├── backup/                  # 部署备份目录
└── temp/                    # 临时文件目录
```

## 🔧 配置说明

### 基本配置
- **worker进程数**: 自动检测CPU核心数
- **连接数**: 1024个并发连接
- **keepalive**: 65秒超时
- **上传限制**: 100MB

### 代理配置
- **连接超时**: 30秒
- **读写超时**: 30秒（API）/ 60秒（文件）
- **缓冲设置**: 启用代理缓冲优化

### 缓存配置
- **静态资源**: 1年缓存期
- **gzip压缩**: 启用多种文件类型压缩

### Windows优化
- **事件模型**: 使用select（Windows默认）
- **文件操作**: 优化sendfile设置
- **路径处理**: 兼容Windows路径格式

## 📞 技术支持

如遇到问题，请：
1. 运行 `nginx_manager.bat status` 检查状态
2. 运行 `nginx_manager.bat test` 测试配置
3. 查看 `nginx_manager.bat logs` 了解详细信息
4. 联系技术支持团队

---

**注意**: 
- 请确保后端服务在 `127.0.0.1:15201` 上运行，否则API代理将无法正常工作
- 配置已优化Windows兼容性，移除了Linux特有的指令
- 使用 `deploy_frontend.bat` 可以自动部署前端应用 
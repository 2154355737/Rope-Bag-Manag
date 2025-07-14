# 绳包管理器后端使用说明

## 🚀 快速启动

### 1. 启动服务

#### 推荐方式：PowerShell脚本（支持中文）
```powershell
# 在项目目录下运行
powershell -ExecutionPolicy Bypass -File start.ps1
```

#### 备选方式：批处理脚本
```cmd
# 在项目目录下运行
start.bat
```

#### 直接运行
```bash
# 设置环境变量
set DATABASE_PATH=data.db
set HOST=127.0.0.1
set PORT=8080
set JWT_SECRET=your-secret-key-change-in-production
set UPLOAD_PATH=uploads

# 启动服务
cargo run
```

### 2. 测试API

#### 推荐方式：PowerShell脚本
```powershell
powershell -ExecutionPolicy Bypass -File test_api.ps1
```

#### 备选方式：批处理脚本
```cmd
test_api.bat
```

#### 手动测试
```bash
# 健康检查
curl -X GET http://127.0.0.1:8080/health

# 用户注册
curl -X POST http://127.0.0.1:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d "{\"username\":\"testuser\",\"password\":\"testpass123\",\"email\":\"test@example.com\"}"

# 用户登录
curl -X POST http://127.0.0.1:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d "{\"username\":\"testuser\",\"password\":\"testpass123\"}"
```

## 📋 环境变量说明

| 变量名 | 默认值 | 说明 |
|--------|--------|------|
| `DATABASE_PATH` | `data.db` | SQLite数据库文件路径 |
| `HOST` | `127.0.0.1` | 服务器监听地址 |
| `PORT` | `8080` | 服务器监听端口 |
| `JWT_SECRET` | `your-secret-key-change-in-production` | JWT密钥（生产环境请修改） |
| `UPLOAD_PATH` | `uploads` | 文件上传目录 |

## 🔧 常见问题

### 1. 中文显示乱码
- **解决方案**：使用PowerShell脚本（`.ps1`文件）
- **原因**：Windows批处理文件编码问题

### 2. 端口被占用
- **解决方案**：修改环境变量`PORT`为其他端口
- **检查**：`netstat -an | findstr 8080`

### 3. 数据库连接失败
- **解决方案**：检查`DATABASE_PATH`路径是否正确
- **检查**：确保有写入权限

### 4. 编译错误
- **解决方案**：确保Rust环境正确安装
- **检查**：`rustc --version`

### 5. 权限问题
- **解决方案**：以管理员身份运行PowerShell
- **命令**：右键PowerShell -> 以管理员身份运行

## 📊 服务状态检查

### 健康检查
```bash
curl -X GET http://127.0.0.1:8080/health
```

### 预期响应
```json
{
  "status": "ok",
  "message": "绳包管理器后端服务运行正常",
  "timestamp": "2024-01-01T00:00:00Z"
}
```

## 🛠️ 开发工具

### 代码格式化
```bash
cargo fmt
```

### 代码检查
```bash
cargo clippy
```

### 运行测试
```bash
cargo test
```

### 清理构建
```bash
cargo clean
```

## 📝 日志查看

服务启动后，日志会显示在控制台。主要信息包括：

- 服务器启动地址
- 数据库连接状态
- API请求日志
- 错误信息

## 🔒 安全注意事项

1. **生产环境**：修改`JWT_SECRET`为强密钥
2. **数据库**：定期备份`data.db`文件
3. **文件上传**：检查`UPLOAD_PATH`目录权限
4. **网络**：配置防火墙规则

## 📞 技术支持

如果遇到问题，请检查：

1. Rust版本：`rustc --version`
2. 依赖安装：`cargo build`
3. 端口占用：`netstat -an | findstr 8080`
4. 文件权限：确保有读写权限

## 🎯 下一步

1. 完善认证中间件
2. 添加文件上传功能
3. 实现更多API端点
4. 添加单元测试
5. 配置生产环境 
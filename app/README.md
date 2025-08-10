# 资源社区APP

基于Vue 3 + Vant 4 + Tauri的跨平台资源社区应用。

## 功能特性

- 📱 移动端优先的响应式设计
- 🔐 用户认证和权限管理
- 📝 资源发布与管理
- 💬 评论互动系统
- 📊 个人中心和数据统计
- 🚀 支持Web、桌面端和移动端

## 技术栈

- **前端框架**: Vue 3 + Composition API
- **UI组件库**: Vant 4
- **状态管理**: Pinia
- **路由管理**: Vue Router 4
- **HTTP客户端**: Axios
- **跨平台**: Tauri 2.x
- **构建工具**: Vite 5

## 环境要求

- Node.js 16+
- Rust 1.77.2+
- Android SDK (Android构建)

## 开发环境设置

### 1. 克隆项目并安装依赖

```bash
# 安装前端依赖
npm install

# 安装Tauri CLI
npm install -g @tauri-apps/cli
```

### 2. 环境变量配置

复制 `env.config.example` 文件并创建 `.env.local` 文件：

```bash
cp env.config.example .env.local
```

根据你的部署环境修改 `.env.local` 中的配置：

```env
# 开发环境 - 使用Vite代理
VITE_API_BASE_URL=/api

# 生产环境/Tauri构建 - 使用完整服务器地址
VITE_API_BASE_URL=http://39.105.113.219:15201/api/v1

# 本地开发环境
# VITE_API_BASE_URL=http://localhost:15201/api/v1
```

**重要说明：**
- Web开发环境可以使用相对路径 `/api`，会通过Vite代理转发
- Tauri构建（包括Android）必须使用完整的服务器地址（包含协议、IP/域名、端口）

## 运行项目

### Web开发环境

```bash
# 启动开发服务器
npm run dev

# 构建生产版本
npm run build
```

### Tauri桌面应用

```bash
# 开发模式
npm run tauri:dev

# 构建桌面应用
npm run tauri:build
```

### Tauri Android应用

```bash
# 首次构建需要初始化Android项目
tauri android init

# 开发模式（在Android设备/模拟器上运行）
npm run tauri:android

# 构建Android APK
npm run tauri:android:build
```

## 配置说明

### API地址配置

项目支持多种环境的API地址配置：

1. **环境变量优先级**：`.env.local` > `package.json scripts` > `vite.config.js` 默认值

2. **不同环境配置**：
   - 开发环境：使用Vite代理，配置为 `/api`
   - Tauri构建：必须使用完整服务器地址
   - 生产部署：根据实际服务器地址配置

3. **自动检测机制**：
   - 系统会自动检测是否为Tauri环境
   - 在Tauri环境中如果没有设置环境变量，会使用默认服务器地址

### 网络权限配置

Tauri Android应用已配置必要的网络权限：

- `INTERNET` 权限：允许网络访问
- `usesCleartextTraffic`：允许HTTP请求
- 网络安全配置：允许明文流量（开发用）

### HTTP客户端配置

- 已添加 `tauri-plugin-http` 插件支持
- 配置了API服务器的访问域名白名单
- 支持HTTP和HTTPS请求

## 故障排除

### Android构建常见问题

1. **请求无法发出**
   - 确认 `VITE_API_BASE_URL` 使用完整服务器地址
   - 检查网络权限配置
   - 查看控制台日志获取详细错误信息

2. **依赖安装失败**
   ```bash
   # 清理缓存重新安装
   npm run tauri android init --force
   ```

3. **调试信息**
   - 设置 `VITE_DEBUG=true` 查看详细日志
   - 使用 `tauri android dev` 查看实时日志

### 网络请求调试

项目已添加详细的请求日志，在开发模式下会输出：
- 请求配置信息
- 完整请求URL
- 错误详情和建议

## 项目结构

```
app/
├── src/
│   ├── api/          # API接口定义
│   ├── components/   # Vue组件
│   ├── utils/        # 工具函数
│   ├── views/        # 页面组件
│   └── router/       # 路由配置
├── src-tauri/        # Tauri配置和Rust代码
├── public/           # 静态资源
└── dist/             # 构建输出
```

## 部署说明

### Web部署
构建后将 `dist` 目录部署到Web服务器即可。

### Android发布
1. 使用 `npm run tauri:android:build` 构建APK
2. APK文件位于 `src-tauri/gen/android/app/build/outputs/apk/`
3. 可以直接安装或上传到应用商店

## 开发指南

- 遵循Vue 3 Composition API最佳实践
- 使用Vant组件保持UI一致性
- 合理使用Pinia进行状态管理
- 移动端优先的响应式设计
- 注意Android平台的兼容性测试

## 许可证

[MIT License](LICENSE) 
# Android 构建指南

## 修复的问题

我们已经修复了导致Android应用崩溃的主要问题：

### 1. HTTP插件权限配置
- 在 `src-tauri/capabilities/default.json` 中添加了HTTP插件权限
- 配置了允许访问的URL域名

### 2. 插件初始化优化
- 改进了 `src-tauri/src/lib.rs` 中的插件初始化逻辑
- 添加了条件编译避免移动端的潜在问题

### 3. 环境变量配置
- 确保构建时使用正确的API地址
- 自动检测Tauri环境并设置默认值

## 构建步骤

### 1. 准备环境
```bash
# 确保安装了必要的依赖
npm install

# 检查Tauri CLI
npx @tauri-apps/cli --version
```

### 2. 初始化Android项目（首次构建）
```bash
npx @tauri-apps/cli android init
```

### 3. 构建APK
```bash
# 构建前端资源
npm run build:tauri

# 构建Android APK
npm run tauri:android:build
```

## 故障排除

### 问题：应用启动时崩溃
**解决方案：**
1. 检查 `capabilities/default.json` 是否包含HTTP权限
2. 确保环境变量 `VITE_API_BASE_URL` 设置正确
3. 查看Android日志：`adb logcat`

### 问题：网络请求失败
**解决方案：**
1. 确认API服务器地址可访问
2. 检查 `tauri.conf.json` 中的HTTP scope配置
3. 验证Android清单文件中的网络权限

### 问题：构建失败
**解决方案：**
1. 清理构建缓存：
   ```bash
   npx @tauri-apps/cli android dev --clean
   ```
2. 检查Rust工具链版本
3. 确保Android SDK正确安装

## 调试技巧

### 1. 查看应用日志
```bash
# 连接设备后查看日志
adb logcat | grep -i "tauri\|rust\|app"
```

### 2. 检查网络请求
在前端代码中检查控制台输出，观察是否有API请求发出

### 3. 验证配置
确保以下文件配置正确：
- `src-tauri/capabilities/default.json` - 权限配置
- `src-tauri/tauri.conf.json` - HTTP scope配置
- `src/utils/request.js` - API基础URL配置

## 已知问题

1. **首次构建较慢** - Rust依赖需要编译，这是正常的
2. **部分Android版本兼容性** - 建议使用Android 7.0+ (API 24+)
3. **网络安全策略** - 某些设备可能阻止HTTP请求，只允许HTTPS

## 成功标志

应用成功运行的标志：
1. ✅ 应用启动不崩溃
2. ✅ 能够发出网络请求
3. ✅ 在控制台看到正确的API地址配置日志
4. ✅ 用户界面正常显示 
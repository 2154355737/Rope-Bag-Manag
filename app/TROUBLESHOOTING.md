# Android 崩溃故障排除指南

## 当前状态

我们已经创建了一个最小化的Tauri应用配置来诊断崩溃问题。

## 分步测试计划

### 第1步：测试最小化应用
当前配置已移除所有插件，只保留核心功能。

**构建命令：**
```bash
npm run build:tauri
npm run tauri:android:build
```

**期望结果：** 应用应该能够启动而不崩溃

### 第2步：如果第1步成功，逐步添加功能

#### 2.1 添加HTTP插件依赖
在 `Cargo.toml` 中添加：
```toml
tauri-plugin-http = "2"
```

#### 2.2 在 `lib.rs` 中添加HTTP插件：
```rust
.plugin(tauri_plugin_http::init())
```

#### 2.3 在 `capabilities/default.json` 中添加HTTP权限

### 第3步：如果第1步失败，检查其他问题

可能的原因：
1. **Tauri版本兼容性问题**
2. **Android SDK配置问题** 
3. **设备兼容性问题**
4. **构建环境问题**

## 详细调试步骤

### 1. 检查构建日志
```bash
npm run tauri:android:build 2>&1 | tee build.log
```

### 2. 检查Android日志
```bash
# 安装APK后立即运行
adb logcat -c  # 清空日志
adb install path/to/your.apk
adb logcat | grep -E "(tauri|rust|FATAL|AndroidRuntime)"
```

### 3. 检查应用信息
```bash
adb shell dumpsys package com.jieshen.shequ
```

### 4. 使用更详细的崩溃信息
```bash
adb logcat -b crash
```

## 当前最小化配置

### lib.rs
```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Cargo.toml
只包含基本依赖：
- tauri = "2.7.0"
- serde_json = "1.0" 
- serde = "1.0"
- log = "0.4"

### capabilities/default.json
只包含 "core:default" 权限

## 如果最小化配置仍然崩溃

这可能表明问题出在：

1. **Tauri版本与Android的兼容性**
2. **设备特定的问题**（小米设备的特殊限制）
3. **Android API级别兼容性**
4. **NDK版本问题**

### 检查设备信息
```bash
adb shell getprop ro.build.version.sdk  # API级别
adb shell getprop ro.product.cpu.abi    # CPU架构
adb shell getprop ro.build.version.release  # Android版本
```

### 尝试不同的构建配置

在 `tauri.conf.json` 中添加Android特定配置：
```json
{
  "bundle": {
    "android": {
      "minSdkVersion": 24,
      "compileSdkVersion": 34,
      "targetSdkVersion": 34
    }
  }
}
```

## 成功标准

- ✅ 应用启动不崩溃
- ✅ 显示基本UI界面
- ✅ 不产生FATAL错误日志 
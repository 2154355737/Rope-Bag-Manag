# 结绳社区 Android 应用打包指南

## 📱 概述
本指南将帮助你将React应用打包成Android APK文件。

## 🛠 前置要求

### 1. 安装Java Development Kit (JDK)
- 下载并安装 **JDK 21** (推荐) 或 JDK 17
- 设置 `JAVA_HOME` 环境变量指向JDK 21
- 验证安装：`java -version` (应显示21.x.x版本)
- **重要**: Capacitor 7.x需要Java 21支持

### 2. 安装Android Studio
- 下载 [Android Studio](https://developer.android.com/studio)
- 安装并启动Android Studio
- 通过SDK Manager安装：
  - Android SDK Platform 33 (或最新版本)
  - Android SDK Build-Tools
  - Android SDK Command-line Tools

### 3. 设置环境变量
添加以下环境变量到系统Path：
```
ANDROID_HOME=C:\Users\[用户名]\AppData\Local\Android\Sdk
ANDROID_SDK_ROOT=C:\Users\[用户名]\AppData\Local\Android\Sdk
```

将以下路径添加到Path：
```
%ANDROID_HOME%\tools
%ANDROID_HOME%\tools\bin
%ANDROID_HOME%\platform-tools
```

## 🚀 打包步骤

### 1. 构建Web应用
```bash
npm run build
```

### 2. 同步到Android
```bash
npx cap sync android
```

### 3. 打开Android Studio
```bash
npx cap open android
```

### 4. 在Android Studio中构建APK

#### 方法一：调试版本（快速测试）
1. 在Android Studio中，点击 `Build` → `Build Bundle(s) / APK(s)` → `Build APK(s)`
2. 等待构建完成
3. APK文件位置：`android/app/build/outputs/apk/debug/app-debug.apk`

#### 方法二：发布版本（正式发布）
1. 创建签名密钥：
   ```bash
   keytool -genkey -v -keystore my-release-key.keystore -keyalg RSA -keysize 2048 -validity 10000 -alias my-key-alias
   ```

2. 在 `android/app/build.gradle` 中配置签名：
   ```gradle
   android {
       ...
       signingConfigs {
           release {
               storeFile file('my-release-key.keystore')
               storePassword 'your-store-password'
               keyAlias 'my-key-alias'
               keyPassword 'your-key-password'
           }
       }
       buildTypes {
           release {
               signingConfig signingConfigs.release
               minifyEnabled false
               proguardFiles getDefaultProguardFile('proguard-android.txt'), 'proguard-rules.pro'
           }
       }
   }
   ```

3. 构建发布版APK：
   ```bash
   cd android
   ./gradlew assembleRelease
   ```

## 📱 快速命令

### 开发调试
```bash
# 构建并在设备上运行
npm run android:dev

# 或者
npx cap run android
```

### 完整构建流程
```bash
# 1. 构建Web应用
npm run build

# 2. 同步并打开Android Studio
npm run build:android
```

## 📋 自定义配置

### 应用信息配置
编辑 `android/app/src/main/AndroidManifest.xml`：

```xml
<application
    android:allowBackup="true"
    android:icon="@mipmap/ic_launcher"
    android:label="结绳社区"
    android:theme="@style/AppTheme">
    
    <activity
        android:name=".MainActivity"
        android:exported="true"
        android:launchMode="singleTask"
        android:theme="@style/AppTheme.NoActionBarLaunch">
        
        <intent-filter>
            <action android:name="android.intent.action.MAIN" />
            <category android:name="android.intent.category.LAUNCHER" />
        </intent-filter>
    </activity>
</application>
```

### 应用版本配置
编辑 `android/app/build.gradle`：

```gradle
android {
    compileSdkVersion 33
    
    defaultConfig {
        applicationId "com.jieshengshequ.app"
        minSdkVersion 22
        targetSdkVersion 33
        versionCode 1
        versionName "1.0.0"
    }
}
```

## 🎨 图标和启动屏

### 1. 应用图标
- 准备不同尺寸的图标文件
- 放置在 `android/app/src/main/res/` 对应的 `mipmap-*` 文件夹中

### 2. 启动屏
- 编辑 `android/app/src/main/res/values/styles.xml`
- 自定义启动屏样式和颜色

## ⚡ 性能优化

### 1. 启用代码混淆
在 `android/app/build.gradle` 中：
```gradle
buildTypes {
    release {
        minifyEnabled true
        proguardFiles getDefaultProguardFile('proguard-android-optimize.txt'), 'proguard-rules.pro'
    }
}
```

### 2. 减小APK大小
```gradle
android {
    buildTypes {
        release {
            shrinkResources true
            minifyEnabled true
        }
    }
}
```

## 🔧 常见问题

### 1. Gradle构建失败
- 检查Java版本和ANDROID_HOME环境变量
- 清理项目：`./gradlew clean`

### 2. 设备连接问题
- 启用USB调试
- 检查ADB连接：`adb devices`

### 3. 权限问题
- 在 `AndroidManifest.xml` 中添加必要权限：
```xml
<uses-permission android:name="android.permission.INTERNET" />
<uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />
```

## 📦 发布到应用商店

### Google Play Store
1. 创建Google Play Console账户
2. 上传发布版APK或AAB文件
3. 填写应用信息和描述
4. 提交审核

### 其他应用商店
- 华为应用市场
- 小米应用商店
- OPPO软件商店
- vivo应用商店

## 🎯 下一步

1. 测试APK在不同设备上的兼容性
2. 优化应用性能和用户体验
3. 添加推送通知功能
4. 集成原生功能（相机、GPS等）

---

**注意**：首次构建可能需要较长时间，请耐心等待。如遇到问题，请检查环境配置和网络连接。 
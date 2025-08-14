@echo off
echo 正在构建Android应用...

echo 1. 清理之前的构建文件...
cd /d "%~dp0"
if exist "src-tauri\target" rmdir /s /q "src-tauri\target"
if exist "src-tauri\gen\android\app\build" rmdir /s /q "src-tauri\gen\android\app\build"

echo 2. 构建前端资源...
call npm run build:tauri
if %errorlevel% neq 0 (
    echo 前端构建失败！
    pause
    exit /b 1
)

echo 3. 构建Android APK...
cd src-tauri
echo 清理Rust构建缓存...
call cargo clean
echo 开始Android构建...
call npx @tauri-apps/cli android build --target aarch64-linux-android --verbose
if %errorlevel% neq 0 (
    echo Android构建失败！
    echo 尝试查看详细错误信息...
    pause
    exit /b 1
)

echo 4. 构建完成！
echo APK文件位置: src-tauri\gen\android\app\build\outputs\apk\universal\release\
echo 或者: src-tauri\gen\android\app\build\outputs\apk\debug\
dir "src-tauri\gen\android\app\build\outputs\apk\" /s *.apk
pause 
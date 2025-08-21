@echo off
echo ====================================
echo      首页性能优化版本测试工具
echo ====================================
echo.

echo 1. 备份原始首页文件...
if exist "src\views\main\Home.vue.backup" (
    echo    原始文件已备份
) else (
    copy "src\views\main\Home.vue" "src\views\main\Home.vue.backup"
    echo    ✓ 备份完成: Home.vue.backup
)

echo.
echo 2. 切换到优化版本...
copy "src\views\main\HomeOptimized.vue" "src\views\main\Home.vue"
echo    ✓ 已切换到优化版本

echo.
echo 3. 启动开发服务器...
echo    正在启动... (可能需要等待几秒)
echo.

start "Vue开发服务器" cmd /k "npm run dev"

echo ====================================
echo    服务器启动中，请稍等...
echo    
echo    测试建议：
echo    1. 打开浏览器访问 http://localhost:5173/home
echo    2. 按F12打开开发者工具
echo    3. 切换到Performance标签页
echo    4. 点击录制按钮，刷新页面，停止录制
echo    5. 查看性能分析结果
echo    
echo    如需恢复原版本，运行: 恢复原版本.bat
echo ====================================

pause 
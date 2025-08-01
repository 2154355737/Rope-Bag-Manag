@echo off
chcp 65001 >nul
setlocal enabledelayedexpansion

echo 正在启动部署脚本...
echo.

REM Create log directory first
set LOG_DIR=%~dp0logs
if not exist "%LOG_DIR%" (
    mkdir "%LOG_DIR%"
    echo 已创建日志目录: %LOG_DIR%
)

REM Create timestamped log file
set "timestamp=%date:~0,4%%date:~5,2%%date:~8,2%_%time:~0,2%%time:~3,2%%time:~6,2%"
set "timestamp=!timestamp: =0!"
set LOG_FILE=%LOG_DIR%\deploy_!timestamp!.log

echo ========== 部署脚本启动 ========== > "%LOG_FILE%"
echo 脚本启动时间: %date% %time% >> "%LOG_FILE%"
echo 工作目录: %CD% >> "%LOG_FILE%"
echo ============================================== >> "%LOG_FILE%"

echo [日志] 已创建日志文件: %LOG_FILE%

REM Basic error handling
if not defined COMSPEC (
    echo 错误: 命令处理器未定义
    echo 错误: 命令处理器未定义 >> "%LOG_FILE%"
    pause
    exit /b 1
)
echo [日志] 命令处理器检查通过 >> "%LOG_FILE%"

REM Configuration
set FRONT_ROOT=D:\vue\zhiyuanshequ\rust\Rust_Vue
set NGINX_HOME=D:\vue\zhiyuanshequ\rust\nginx-1.28.0
set BACKEND_ROOT=D:\vue\zhiyuanshequ\rust\rope-manager-backend

echo [信息] 前端目录: %FRONT_ROOT%
echo [信息] Nginx目录: %NGINX_HOME%
echo [信息] 后端目录: %BACKEND_ROOT%
echo.

echo [日志] 配置设置: >> "%LOG_FILE%"
echo [日志] FRONT_ROOT=%FRONT_ROOT% >> "%LOG_FILE%"
echo [日志] NGINX_HOME=%NGINX_HOME% >> "%LOG_FILE%"
echo [日志] BACKEND_ROOT=%BACKEND_ROOT% >> "%LOG_FILE%"

echo [信息] 日志文件已创建: %LOG_FILE%
echo.

REM Check directories
echo ========== 环境检查 ==========
echo [LOG] 开始环境检查 >> "%LOG_FILE%"

if not exist "%FRONT_ROOT%" (
    echo [错误] 前端目录未找到
    echo [错误] 前端目录未找到: %FRONT_ROOT% >> "%LOG_FILE%"
    goto :ERROR
)
echo [OK] 前端目录存在
echo [LOG] 前端目录存在: %FRONT_ROOT% >> "%LOG_FILE%"

if not exist "%NGINX_HOME%" (
    echo [错误] Nginx目录未找到
    echo [错误] Nginx目录未找到: %NGINX_HOME% >> "%LOG_FILE%"
    goto :ERROR
)
echo [OK] Nginx目录存在
echo [LOG] Nginx目录存在: %NGINX_HOME% >> "%LOG_FILE%"

if not exist "%BACKEND_ROOT%" (
    echo [错误] 后端目录未找到
    echo [错误] 后端目录未找到: %BACKEND_ROOT% >> "%LOG_FILE%"
    goto :ERROR
)
echo [OK] 后端目录存在
echo [LOG] 后端目录存在: %BACKEND_ROOT% >> "%LOG_FILE%"

REM Check npm
echo [LOG] 检查npm可用性 >> "%LOG_FILE%"
where npm >nul 2>&1
if errorlevel 1 (
    echo [错误] npm未找到
    echo [错误] npm未找到在PATH中 >> "%LOG_FILE%"
    goto :ERROR
)
echo [OK] npm可用
echo [LOG] npm可用 >> "%LOG_FILE%"

REM Check cargo
echo [LOG] 检查cargo可用性 >> "%LOG_FILE%"
where cargo >nul 2>&1
if errorlevel 1 (
    echo [错误] cargo未找到
    echo [错误] cargo未找到在PATH中 >> "%LOG_FILE%"
    goto :ERROR
)
echo [OK] cargo可用
echo [LOG] cargo可用 >> "%LOG_FILE%"

echo.
echo ========== 构建前端 ==========
echo [LOG] 开始前端构建过程 >> "%LOG_FILE%"

echo [信息] 切换到前端目录...
echo [LOG] 切换到前端目录: %FRONT_ROOT% >> "%LOG_FILE%"

cd /d "%FRONT_ROOT%"
if errorlevel 1 (
    echo [错误] 无法切换到前端目录
    echo [错误] 无法切换到前端目录: %FRONT_ROOT% >> "%LOG_FILE%"
    goto :ERROR
)

echo [信息] 当前目录: %CD%
echo [LOG] 当前目录: %CD% >> "%LOG_FILE%"

if not exist "package.json" (
    echo [错误] package.json未找到
    echo [错误] package.json未找到: %CD% >> "%LOG_FILE%"
    goto :ERROR
)
echo [OK] package.json已找到
echo [LOG] package.json已找到: %CD% >> "%LOG_FILE%"

REM Skip npm install - assume dependencies are already installed
echo.
echo [信息] 跳过npm安装 (假设依赖已安装)...
echo [LOG] 跳过npm安装 >> "%LOG_FILE%"

REM Check if node_modules exists
if not exist "node_modules" (
    echo [警告] node_modules目录未找到
    echo [警告] 您可能需要手动运行 'npm install' 首先
    echo [警告] node_modules未找到: %CD% >> "%LOG_FILE%"
)

echo.
echo [信息] 构建项目 (这可能需要几分钟)...
echo [调试] 执行: call npm run build
echo [LOG] 开始npm run build: %time% >> "%LOG_FILE%"

REM Use call to prevent terminal from closing
call npm run build
set "build_exit_code=%errorlevel%"

echo [LOG] npm run build完成于: %time% 退出码: %build_exit_code% >> "%LOG_FILE%"

REM Force a clear break after build command to prevent output confusion
echo.
echo.
echo.
echo ========================================
echo [调试] 构建过程完成!
echo [调试] 退出码: %build_exit_code%
echo [调试] 继续部署...
echo ========================================
echo.

if %build_exit_code% neq 0 (
    echo [错误] npm run build失败退出码: %build_exit_code%
    echo [错误] npm run build失败退出码: %build_exit_code% >> "%LOG_FILE%"
    echo [信息] 构建失败, 停止部署
    goto :ERROR
)

echo [OK] 构建成功
echo [LOG] 构建成功 >> "%LOG_FILE%"

echo [信息] 检查构建输出...
set DIST_DIR=%FRONT_ROOT%\dist
echo [调试] 查找dist目录: %DIST_DIR%
echo [LOG] 检查dist目录: %DIST_DIR% >> "%LOG_FILE%"

if not exist "%DIST_DIR%" (
    echo [错误] dist目录未创建: %DIST_DIR%
    echo [错误] dist目录未创建: %DIST_DIR% >> "%LOG_FILE%"
    echo [信息] 列出当前目录内容:
    dir /b
    dir /b >> "%LOG_FILE%"
    goto :ERROR
)
echo [OK] dist目录已找到
echo [LOG] dist目录已找到: %DIST_DIR% >> "%LOG_FILE%"

echo [信息] 检查dist目录内容...
if exist "%DIST_DIR%\*" (
    echo [OK] dist目录包含文件
    echo [LOG] dist目录包含文件 >> "%LOG_FILE%"
    dir "%DIST_DIR%" /b >> "%LOG_FILE%"
) else (
    echo [警告] dist目录似乎为空
    echo [警告] dist目录为空: %DIST_DIR% >> "%LOG_FILE%"
)

echo.
echo ========== 部署到Nginx ==========
echo [LOG] 开始部署到Nginx >> "%LOG_FILE%"

set NGINX_HTML=%NGINX_HOME%\html
set BACKUP_DIR=%NGINX_HOME%\backup_%date:~0,4%%date:~5,2%%date:~8,2%

echo [信息] 创建备份...
echo [LOG] 从%NGINX_HTML%创建备份到%BACKUP_DIR% >> "%LOG_FILE%"
echo [调试] 备份目录将: %BACKUP_DIR%
echo [调试] Nginx HTML目录: %NGINX_HTML%

if exist "%NGINX_HTML%" (
    echo [调试] Nginx HTML目录存在, 检查内容...
    echo [LOG] Nginx HTML目录存在, 检查内容 >> "%LOG_FILE%"
    dir "%NGINX_HTML%" /b >nul 2>&1
    if errorlevel 1 (
        echo [信息] Nginx HTML目录为空, 跳过备份
        echo [LOG] Nginx HTML目录为空, 跳过备份 >> "%LOG_FILE%"
    ) else (
        echo [调试] 创建备份目录...
        echo [LOG] 创建备份目录: %BACKUP_DIR% >> "%LOG_FILE%"
        if not exist "%BACKUP_DIR%" mkdir "%BACKUP_DIR%"
        echo [调试] 开始xcopy操作...
        echo [LOG] 开始xcopy操作 >> "%LOG_FILE%"
        xcopy "%NGINX_HTML%\*" "%BACKUP_DIR%\" /E /I /H /R /Y
        set "backup_exit=!errorlevel!"
        echo [调试] xcopy退出码: !backup_exit!
        echo [LOG] xcopy退出码: !backup_exit! >> "%LOG_FILE%"
        if !backup_exit! neq 0 (
            echo [警告] 备份完成但有警告 (退出码: !backup_exit!)
            echo [警告] 备份完成但有警告 (退出码: !backup_exit!) >> "%LOG_FILE%"
        ) else (
            echo [OK] 备份成功
            echo [LOG] 备份成功 >> "%LOG_FILE%"
        )
        echo [OK] 备份已创建于: %BACKUP_DIR%
    )
) else (
    echo [调试] Nginx HTML目录不存在, 创建它...
    echo [LOG] Nginx HTML目录不存在, 创建它 >> "%LOG_FILE%"
    mkdir "%NGINX_HTML%"
    echo [OK] Nginx html目录已创建
    echo [LOG] Nginx html目录已创建: %NGINX_HTML% >> "%LOG_FILE%"
)

echo [信息] 清理nginx目录...
echo [LOG] 开始nginx目录清理 >> "%LOG_FILE%"
echo [调试] 清理目录: %NGINX_HTML%
if exist "%NGINX_HTML%" (
    echo [调试] 删除文件...
    echo [LOG] 从%NGINX_HTML%删除文件 >> "%LOG_FILE%"
    del /Q "%NGINX_HTML%\*.*" >nul 2>&1
    echo [调试] 检查子目录...
    echo [LOG] 检查子目录 >> "%LOG_FILE%"
for /D %%d in ("%NGINX_HTML%\*") do (
        if not "%%~nxd"=="uploads" if not "%%~nxd"=="data.db" (
            echo [调试] 删除目录: %%d
            echo [LOG] 删除目录: %%d >> "%LOG_FILE%"
            rmdir /S /Q "%%d" >nul 2>&1
        ) else (
            echo [调试] 保留目录: %%d
            echo [LOG] 保留目录: %%d >> "%LOG_FILE%"
    )
)
    echo [调试] 目录清理完成
    echo [LOG] 目录清理完成 >> "%LOG_FILE%"
)
echo [OK] Nginx目录已清理

echo [信息] 将文件从%DIST_DIR%移动到%NGINX_HTML%...
echo [LOG] 开始文件移动操作 >> "%LOG_FILE%"
echo [调试] 源目录: %DIST_DIR%
echo [调试] 目标目录: %NGINX_HTML%
echo [调试] 开始robocopy操作...
echo [LOG] robocopy从%DIST_DIR%到%NGINX_HTML% >> "%LOG_FILE%"
robocopy "%DIST_DIR%" "%NGINX_HTML%" /E /MOVE /R:2 /W:1
set "move_exit_code=%errorlevel%"
echo [调试] Robocopy完成退出码: %move_exit_code%
echo [LOG] Robocopy完成退出码: %move_exit_code% >> "%LOG_FILE%"

REM robocopy退出码: 0-7为成功, 8+为错误
if %move_exit_code% GEQ 8 (
    echo [错误] 文件移动失败退出码: %move_exit_code%
    echo [错误] 文件移动失败退出码: %move_exit_code% >> "%LOG_FILE%"
    goto :ERROR
)
echo [OK] 文件移动成功 (dist目录已清理)
echo [LOG] 文件移动成功 >> "%LOG_FILE%"

echo [信息] 验证部署...
echo [LOG] 验证部署 >> "%LOG_FILE%"
if exist "%NGINX_HTML%\index.html" (
    echo [OK] index.html在nginx目录中找到
    echo [LOG] index.html在nginx目录中找到 >> "%LOG_FILE%"
) else (
    echo [警告] index.html未在nginx目录中找到
    echo [警告] index.html未在nginx目录中找到 >> "%LOG_FILE%"
)

echo.
echo ========== 重启服务 ==========
echo [LOG] 开始服务重启 >> "%LOG_FILE%"

echo [信息] 详细Nginx重启过程...
cd /d "%NGINX_HOME%"
echo [LOG] 切换到nginx目录: %NGINX_HOME% >> "%LOG_FILE%"

REM 详细nginx状态检查
echo [步骤1] 检查当前Nginx进程...
echo [LOG] 步骤1: 检查当前Nginx进程 >> "%LOG_FILE%"
tasklist | findstr /I "nginx.exe" >nul 2>&1
if errorlevel 1 (
    echo [状态] 未找到Nginx进程
    echo [LOG] 未找到Nginx进程 >> "%LOG_FILE%"
) else (
    echo [状态] 当前Nginx进程:
    echo [LOG] 当前Nginx进程已找到 >> "%LOG_FILE%"
    tasklist | findstr /I "nginx.exe"
    tasklist | findstr /I "nginx.exe" >> "%LOG_FILE%"
)

REM 检查端口80是否被占用
echo [步骤2] 检查端口80使用情况...
echo [LOG] 步骤2: 检查端口80使用情况 >> "%LOG_FILE%"
netstat -ano | findstr ":80 " >nul 2>&1
if errorlevel 1 (
    echo [状态] 端口80空闲
    echo [LOG] 端口80空闲 >> "%LOG_FILE%"
) else (
    echo [状态] 端口80被占用:
    echo [LOG] 端口80被占用 >> "%LOG_FILE%"
    netstat -ano | findstr ":80 "
    netstat -ano | findstr ":80 " >> "%LOG_FILE%"
)

  REM 停止nginx
  echo [步骤3] 停止Nginx...
  echo [LOG] 步骤3: 停止Nginx >> "%LOG_FILE%"
  
  REM 首先尝试使用自动停止脚本
  if exist "stop_nginx_auto.bat" (
      echo [调试] 使用自动停止脚本...
      echo [LOG] 使用自动停止脚本 >> "%LOG_FILE%"
      call stop_nginx_auto.bat
      set "nginx_stop_exit=!errorlevel!"
      echo [调试] stop_nginx_auto.bat退出码: !nginx_stop_exit!
      echo [LOG] stop_nginx_auto.bat退出码: !nginx_stop_exit! >> "%LOG_FILE%"
  ) else (
      echo [调试] 未找到自动停止脚本, 使用直接命令...
      echo [LOG] 未找到自动停止脚本, 使用直接命令 >> "%LOG_FILE%"
      
      REM 检查nginx是否正在运行
      if exist "logs\nginx.pid" (
          echo [调试] Nginx正在运行, 优雅停止...
          echo [LOG] Nginx正在运行, 优雅停止 >> "%LOG_FILE%"
          nginx.exe -s quit
          echo [LOG] nginx.exe -s quit命令已执行 >> "%LOG_FILE%"
          
          REM 等待优雅关闭
          echo [调试] 等待优雅关闭...
          echo [LOG] 等待优雅关闭 >> "%LOG_FILE%"
          timeout /t 3 /nobreak >nul
          
          REM 检查是否仍在运行
          if exist "logs\nginx.pid" (
              echo [调试] Nginx仍在运行, 强制杀死...
              echo [LOG] Nginx仍在运行, 强制杀死 >> "%LOG_FILE%"
              taskkill /F /IM "nginx.exe" >nul 2>&1
              if exist "logs\nginx.pid" del "logs\nginx.pid" >nul 2>&1
              echo [LOG] 强制杀死完成 >> "%LOG_FILE%"
          ) else (
              echo [OK] Nginx优雅停止
              echo [LOG] Nginx优雅停止 >> "%LOG_FILE%"
          )
      ) else (
          echo [信息] Nginx未运行
          echo [LOG] Nginx未运行 >> "%LOG_FILE%"
      )
  )

echo [信息] 等待5秒, 确保Nginx完全停止...
echo [LOG] 等待5秒确保Nginx停止 >> "%LOG_FILE%"
timeout /t 5 /nobreak >nul

REM 验证nginx是否已停止
echo [步骤4] 验证Nginx已停止...
echo [LOG] 步骤4: 验证Nginx已停止 >> "%LOG_FILE%"
tasklist | findstr /I "nginx.exe" >nul 2>&1
if errorlevel 1 (
    echo [成功] 所有Nginx进程已停止
    echo [LOG] 所有Nginx进程已停止 >> "%LOG_FILE%"
) else (
    echo [警告] 某些Nginx进程可能仍在运行:
    echo [LOG] 某些Nginx进程仍在运行 >> "%LOG_FILE%"
    tasklist | findstr /I "nginx.exe"
    tasklist | findstr /I "nginx.exe" >> "%LOG_FILE%"
    echo [信息] 强制杀死剩余进程...
    echo [LOG] 强制杀死剩余进程 >> "%LOG_FILE%"
    taskkill /F /IM "nginx.exe" >nul 2>&1
    timeout /t 2 /nobreak >nul
)

  REM 启动nginx
  echo [步骤5] 启动Nginx...
  echo [LOG] 步骤5: 启动Nginx >> "%LOG_FILE%"
  
  REM 首先尝试使用部署特定的启动脚本
  if exist "start_nginx_deploy.bat" (
      echo [调试] 使用部署特定的启动脚本...
      echo [LOG] 使用部署特定的启动脚本 >> "%LOG_FILE%"
      call start_nginx_deploy.bat
      set "nginx_start_exit=!errorlevel!"
      echo [调试] start_nginx_deploy.bat退出码: !nginx_start_exit!
      echo [LOG] start_nginx_deploy.bat退出码: !nginx_start_exit! >> "%LOG_FILE%"
      
      if !nginx_start_exit! neq 0 (
          echo [警告] 部署启动脚本返回错误退出码: !nginx_start_exit!
          echo [LOG] 警告: 部署启动脚本返回错误退出码: !nginx_start_exit! >> "%LOG_FILE%"
      ) else (
          echo [OK] Nginx部署启动命令已执行成功
          echo [LOG] Nginx部署启动命令已执行成功 >> "%LOG_FILE%"
      )
  ) else if exist "start_nginx_auto.bat" (
      echo [调试] 使用自动启动脚本...
      echo [LOG] 使用自动启动脚本 >> "%LOG_FILE%"
      call start_nginx_auto.bat
      set "nginx_start_exit=!errorlevel!"
      echo [调试] start_nginx_auto.bat退出码: !nginx_start_exit!
      echo [LOG] start_nginx_auto.bat退出码: !nginx_start_exit! >> "%LOG_FILE%"
      
      if !nginx_start_exit! neq 0 (
          echo [警告] Nginx启动失败退出码: !nginx_start_exit!
          echo [LOG] 警告: Nginx启动失败退出码: !nginx_start_exit! >> "%LOG_FILE%"
      ) else (
          echo [OK] Nginx启动命令已执行成功
          echo [LOG] Nginx启动命令已执行成功 >> "%LOG_FILE%"
      )
  ) else (
      echo [调试] 未找到自动启动脚本, 使用直接命令...
      echo [LOG] 未找到自动启动脚本, 使用直接命令 >> "%LOG_FILE%"
      
      REM 检查nginx是否已运行
      if exist "logs\nginx.pid" (
          echo [信息] Nginx已运行
          echo [LOG] Nginx已运行 >> "%LOG_FILE%"
      ) else (
          echo [调试] 测试nginx配置...
          echo [LOG] 测试nginx配置 >> "%LOG_FILE%"
          nginx.exe -t
          set "config_test_exit=!errorlevel!"
          echo [LOG] nginx配置测试退出码: !config_test_exit! >> "%LOG_FILE%"
          
          if !config_test_exit! neq 0 (
              echo [错误] Nginx配置测试失败
              echo [LOG] Nginx配置测试失败 >> "%LOG_FILE%"
              goto :ERROR
          )
          echo [OK] Nginx配置有效
          echo [LOG] Nginx配置有效 >> "%LOG_FILE%"
          
          REM 直接启动nginx
          echo [调试] 直接启动nginx...
          echo [LOG] 直接启动nginx >> "%LOG_FILE%"
          start /b "" nginx.exe
          echo [LOG] nginx.exe启动命令已执行 >> "%LOG_FILE%"
      )
  )

  echo [信息] 等待5秒, 确保Nginx启动...
  echo [LOG] 等待5秒确保Nginx启动 >> "%LOG_FILE%"
  timeout /t 5 /nobreak >nul
  
  REM 立即检查nginx.pid文件是否已创建
  echo [调试] 检查nginx.pid文件...
  echo [LOG] 检查nginx.pid文件 >> "%LOG_FILE%"
  if exist "logs\nginx.pid" (
      echo [调试] nginx.pid文件已找到
      echo [LOG] nginx.pid文件已找到 >> "%LOG_FILE%"
      type "logs\nginx.pid" >> "%LOG_FILE%"
  ) else (
      echo [调试] nginx.pid文件未找到
      echo [LOG] nginx.pid文件未找到 >> "%LOG_FILE%"
  )
  
  REM 等待一段时间, 确保进程完全启动
  echo [调试] 等待额外3秒, 确保进程稳定...
  echo [LOG] 等待额外3秒确保进程稳定 >> "%LOG_FILE%"
  timeout /t 3 /nobreak >nul

REM 详细验证nginx启动
echo [步骤6] 验证Nginx启动...
echo [LOG] 步骤6: 验证Nginx启动 >> "%LOG_FILE%"

REM 首先检查nginx.pid是否存在
if exist "logs\nginx.pid" (
    echo [调试] nginx.pid文件存在, 检查进程...
    echo [LOG] nginx.pid文件存在, 检查进程 >> "%LOG_FILE%"
) else (
    echo [警告] nginx.pid文件未找到, 但继续进程检查...
    echo [LOG] nginx.pid文件未找到, 但继续进程检查 >> "%LOG_FILE%"
)

tasklist | findstr /I "nginx.exe" >nul 2>&1
if errorlevel 1 (
    echo [警告] Nginx进程未立即启动
    echo [LOG] 警告: Nginx进程未立即启动 >> "%LOG_FILE%"
    echo [信息] 等待额外5秒并再次检查...
    echo [LOG] 等待额外5秒并再次检查 >> "%LOG_FILE%"
    timeout /t 5 /nobreak >nul
    
    tasklist | findstr /I "nginx.exe" >nul 2>&1
    if errorlevel 1 (
        echo [错误] Nginx进程仍未找到
        echo [LOG] 错误: Nginx进程仍未找到 >> "%LOG_FILE%"
        echo [信息] 检查错误日志...
        if exist "logs\error.log" (
            echo [调试] error.log最后5行:
            echo [LOG] error.log最后5行: >> "%LOG_FILE%"
            powershell -Command "Get-Content 'logs\error.log' -Tail 5"
            powershell -Command "Get-Content 'logs\error.log' -Tail 5" >> "%LOG_FILE%"
        )
        echo [信息] Nginx可能启动失败, 但继续部署...
        echo [LOG] Nginx可能启动失败, 但继续部署 >> "%LOG_FILE%"
    ) else (
        echo [成功] Nginx进程已找到
        echo [LOG] Nginx进程已找到 >> "%LOG_FILE%"
        tasklist | findstr /I "nginx.exe"
        tasklist | findstr /I "nginx.exe" >> "%LOG_FILE%"
    )
) else (
    echo [成功] Nginx进程正在运行:
    echo [LOG] Nginx进程正在运行 >> "%LOG_FILE%"
    tasklist | findstr /I "nginx.exe"
    tasklist | findstr /I "nginx.exe" >> "%LOG_FILE%"
)

REM 检查端口80
echo [步骤7] 验证端口80是否活跃...
echo [LOG] 步骤7: 验证端口80是否活跃 >> "%LOG_FILE%"
netstat -ano | findstr ":80 " >nul 2>&1
if errorlevel 1 (
    echo [警告] 端口80未检测为活跃
    echo [LOG] 警告: 端口80未检测为活跃 >> "%LOG_FILE%"
) else (
    echo [成功] 端口80活跃:
    echo [LOG] 端口80活跃 >> "%LOG_FILE%"
    netstat -ano | findstr ":80 "
    netstat -ano | findstr ":80 " >> "%LOG_FILE%"
)

REM 测试HTTP连接
echo [步骤8] 测试HTTP连接...
echo [LOG] 步骤8: 测试HTTP连接 >> "%LOG_FILE%"
powershell -Command "try { $response = Invoke-WebRequest -Uri 'http://localhost' -TimeoutSec 5; Write-Host '[成功] HTTP连接测试通过' } catch { Write-Host '[警告] HTTP连接测试失败:' $_.Exception.Message }"
powershell -Command "try { $response = Invoke-WebRequest -Uri 'http://localhost' -TimeoutSec 5; '[LOG] HTTP连接测试通过' } catch { '[LOG] HTTP连接测试失败: ' + $_.Exception.Message }" >> "%LOG_FILE%"

echo.
echo ========== 后端启动 ==========
echo [LOG] 开始后端启动过程 >> "%LOG_FILE%"

echo [信息] 管理后端服务...
cd /d "%BACKEND_ROOT%"
echo [LOG] 切换到后端目录: %BACKEND_ROOT% >> "%LOG_FILE%"

REM 检查当前后端进程
echo [步骤1] 检查当前后端进程...
echo [LOG] 步骤1: 检查当前后端进程 >> "%LOG_FILE%"
tasklist | findstr /I "rope-manager-backend.exe" >nul 2>&1
if errorlevel 1 (
    echo [状态] 未找到后端进程
    echo [LOG] 未找到后端进程 >> "%LOG_FILE%"
) else (
    echo [状态] 当前后端进程:
    echo [LOG] 当前后端进程已找到 >> "%LOG_FILE%"
    tasklist | findstr /I "rope-manager-backend.exe"
    tasklist | findstr /I "rope-manager-backend.exe" >> "%LOG_FILE%"
)

REM 检查后端端口 (后端运行在15201)
echo [步骤2] 检查后端端口15201使用情况...
echo [LOG] 步骤2: 检查后端端口15201使用情况 >> "%LOG_FILE%"
netstat -ano | findstr ":15201 " >nul 2>&1
if errorlevel 1 (
    echo [状态] 端口15201空闲
    echo [LOG] 端口15201空闲 >> "%LOG_FILE%"
) else (
    echo [状态] 端口15201被占用:
    echo [LOG] 端口15201被占用 >> "%LOG_FILE%"
    netstat -ano | findstr ":15201 "
    netstat -ano | findstr ":15201 " >> "%LOG_FILE%"
)

REM 终止现有后端进程
echo [步骤3] 终止现有后端进程...
echo [LOG] 步骤3: 终止现有后端进程 >> "%LOG_FILE%"
taskkill /F /IM "rope-manager-backend.exe" >nul 2>&1
taskkill /F /IM "cargo.exe" >nul 2>&1
timeout /t 3 /nobreak >nul

REM 启动新后端进程
echo [步骤4] 启动新后端进程...
echo [LOG] 步骤4: 启动新后端进程 >> "%LOG_FILE%"
echo [调试] 命令: start "Rope Manager Backend" cmd /k "cargo run"
start "Rope Manager Backend" cmd /k "cargo run"
echo [LOG] 后端启动命令已执行 >> "%LOG_FILE%"

echo [信息] 等待10秒, 确保后端初始化...
echo [LOG] 等待10秒确保后端初始化 >> "%LOG_FILE%"
timeout /t 10 /nobreak >nul

REM 验证后端启动
echo [步骤5] 验证后端启动...
echo [LOG] 步骤5: 验证后端启动 >> "%LOG_FILE%"
tasklist | findstr /I "cargo.exe" >nul 2>&1
if errorlevel 1 (
    echo [警告] Cargo进程未检测到
    echo [LOG] 警告: Cargo进程未检测到 >> "%LOG_FILE%"
) else (
    echo [成功] Cargo进程正在运行:
    echo [LOG] Cargo进程正在运行 >> "%LOG_FILE%"
    tasklist | findstr /I "cargo.exe"
    tasklist | findstr /I "cargo.exe" >> "%LOG_FILE%"
)

REM 检查后端端口是否活跃
echo [步骤6] 检查后端端口15201是否活跃...
echo [LOG] 步骤6: 检查后端端口15201是否活跃 >> "%LOG_FILE%"
netstat -ano | findstr ":15201 " >nul 2>&1
if errorlevel 1 (
    echo [警告] 后端端口15201未活跃 (可能仍在启动)
    echo [LOG] 后端端口15201未活跃 >> "%LOG_FILE%"
) else (
    echo [成功] 后端端口15201活跃:
    echo [LOG] 后端端口15201活跃 >> "%LOG_FILE%"
    netstat -ano | findstr ":15201 "
    netstat -ano | findstr ":15201 " >> "%LOG_FILE%"
)

REM 测试后端API连接
echo [步骤7] 测试后端API连接...
echo [LOG] 步骤7: 测试后端API连接 >> "%LOG_FILE%"
powershell -Command "try { $response = Invoke-WebRequest -Uri 'http://localhost:15201' -TimeoutSec 5; Write-Host '[成功] 后端API连接测试通过' } catch { Write-Host '[信息] 后端可能仍在启动或API未准备好:' $_.Exception.Message }"
powershell -Command "try { $response = Invoke-WebRequest -Uri 'http://localhost:15201' -TimeoutSec 5; '[LOG] 后端API连接测试通过' } catch { '[LOG] 后端API连接测试失败: ' + $_.Exception.Message }" >> "%LOG_FILE%"

echo.
echo ========== 最终状态总结 ==========
echo [LOG] 生成最终状态总结 >> "%LOG_FILE%"

echo [信息] 部署总结:
echo ----------------------------------------
echo 前端构建: 已完成
echo 文件部署: 已完成
echo Nginx状态: 
tasklist | findstr /I "nginx.exe" >nul 2>&1
if errorlevel 1 (
    echo   [错误] 未运行
    echo [LOG] 最终状态 - Nginx: 未运行 >> "%LOG_FILE%"
) else (
    echo   [成功] 运行中
    echo [LOG] 最终状态 - Nginx: 运行中 >> "%LOG_FILE%"
)

echo 后端状态:
tasklist | findstr /I "cargo.exe" >nul 2>&1
if errorlevel 1 (
    echo   [警告] 未检测到
    echo [LOG] 最终状态 - 后端: 未检测到 >> "%LOG_FILE%"
) else (
    echo   [成功] 运行中
    echo [LOG] 最终状态 - 后端: 运行中 >> "%LOG_FILE%"
)

echo 端口状态:
netstat -ano | findstr ":80 " >nul 2>&1
if errorlevel 1 (
    echo   Port 80 (Nginx): 未活跃
    echo [LOG] 最终状态 - Port 80: 未活跃 >> "%LOG_FILE%"
) else (
    echo   Port 80 (Nginx): 活跃
    echo [LOG] 最终状态 - Port 80: 活跃 >> "%LOG_FILE%"
)

netstat -ano | findstr ":15201 " >nul 2>&1
if errorlevel 1 (
    echo   Port 15201 (Backend): 未活跃
    echo [LOG] 最终状态 - Port 15201: 未活跃 >> "%LOG_FILE%"
) else (
    echo   Port 15201 (Backend): 活跃
    echo [LOG] 最终状态 - Port 15201: 活跃 >> "%LOG_FILE%"
)

echo ----------------------------------------

echo.
echo ========== 成功 ==========
echo [成功] 部署过程已完成!
echo [信息] 备份位置: %BACKUP_DIR%
echo [信息] 日志文件: %LOG_FILE%
echo [信息] 您可以访问应用程序: http://localhost
echo [信息] 后端API应可在: http://localhost:15201
echo.
echo [LOG] 部署成功完成于: %time% >> "%LOG_FILE%"
echo ========== 部署脚本结束 ========== >> "%LOG_FILE%"
goto :END

:ERROR
echo.
echo ========== 错误 ==========
echo [错误] 部署失败!
echo [信息] 请检查日志文件: %LOG_FILE%
echo.
echo [LOG] 错误: 部署失败于: %time% >> "%LOG_FILE%"
echo [LOG] 脚本因错误终止 >> "%LOG_FILE%"
if exist "%BACKUP_DIR%" (
    echo [信息] 备份可用: %BACKUP_DIR%
    echo [信息] 要恢复备份, 请运行: xcopy "%BACKUP_DIR%\*" "%NGINX_HTML%\" /E /I /Q
    echo [LOG] 备份可用: %BACKUP_DIR% >> "%LOG_FILE%"
)
echo ========== 部署脚本结束 (错误) ========== >> "%LOG_FILE%"

:END
echo.
echo [信息] 脚本执行完成
echo [信息] 日志文件已保存到: %LOG_FILE%
echo [信息] 按任意键退出...
echo [LOG] 脚本执行完成于: %time% >> "%LOG_FILE%"
pause
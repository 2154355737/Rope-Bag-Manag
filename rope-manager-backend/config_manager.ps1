# 绳包管理器配置管理脚本
param(
    [string]$Action = "show",
    [string]$ConfigFile = "config.toml"
)

function Show-Config {
    param([string]$ConfigFile)
    
    if (Test-Path $ConfigFile) {
        Write-Host "当前配置文件内容:" -ForegroundColor Green
        Write-Host "==================" -ForegroundColor Green
        Get-Content $ConfigFile | ForEach-Object {
            if ($_ -match "^#") {
                Write-Host $_ -ForegroundColor Yellow
            } elseif ($_ -match "^\[") {
                Write-Host $_ -ForegroundColor Cyan
            } else {
                Write-Host $_
            }
        }
    } else {
        Write-Host "配置文件不存在: $ConfigFile" -ForegroundColor Red
    }
}

function New-Config {
    param([string]$ConfigFile)
    
    $defaultConfig = @"
# 绳包管理器后端配置文件
# 服务器配置
[server]
host = "127.0.0.1"
port = 8080
workers = 4
timeout = 30

# 数据库配置
[database]
url = "data.db"
max_connections = 10
timeout = 30

# 认证配置
[auth]
jwt_secret = "your-secret-key-change-in-production"
jwt_expiration = 86400  # 24小时
bcrypt_cost = 12

# 文件配置
[file]
upload_path = "uploads"
max_file_size = 10485760  # 10MB
allowed_extensions = ["zip", "rar", "7z", "tar", "gz"]
temp_path = "temp"

# 日志配置
[logging]
level = "info"
file_path = "logs/app.log"
max_files = 5
max_size = 10485760  # 10MB

# CORS配置
[cors]
allowed_origins = ["*"]
allowed_methods = ["GET", "POST", "PUT", "DELETE", "OPTIONS"]
allowed_headers = ["Content-Type", "Authorization", "X-Requested-With"]
max_age = 3600
"@

    Set-Content -Path $ConfigFile -Value $defaultConfig -Encoding UTF8
    Write-Host "已创建默认配置文件: $ConfigFile" -ForegroundColor Green
}

function Edit-Config {
    param([string]$ConfigFile)
    
    if (Test-Path $ConfigFile) {
        Write-Host "正在打开配置文件进行编辑..." -ForegroundColor Yellow
        notepad $ConfigFile
    } else {
        Write-Host "配置文件不存在，正在创建..." -ForegroundColor Yellow
        New-Config $ConfigFile
        Start-Sleep 2
        notepad $ConfigFile
    }
}

function Test-Config {
    param([string]$ConfigFile)
    
    if (Test-Path $ConfigFile) {
        try {
            $content = Get-Content $ConfigFile -Raw
            $config = [System.Management.Automation.PSParser]::Tokenize($content, [ref]$null)
            Write-Host "配置文件语法检查通过" -ForegroundColor Green
        } catch {
            Write-Host "配置文件语法错误: $($_.Exception.Message)" -ForegroundColor Red
        }
    } else {
        Write-Host "配置文件不存在: $ConfigFile" -ForegroundColor Red
    }
}

function Show-Environment {
    Write-Host "当前环境变量:" -ForegroundColor Green
    Write-Host "==============" -ForegroundColor Green
    
    $envVars = @(
        "HOST", "PORT", "DATABASE_URL", "JWT_SECRET", 
        "UPLOAD_PATH", "MAX_FILE_SIZE", "LOG_LEVEL"
    )
    
    foreach ($var in $envVars) {
        $value = [Environment]::GetEnvironmentVariable($var)
        if ($value) {
            Write-Host "$var = $value" -ForegroundColor Cyan
        } else {
            Write-Host "$var = (未设置)" -ForegroundColor Gray
        }
    }
}

function Set-Environment {
    param([string]$Key, [string]$Value)
    
    [Environment]::SetEnvironmentVariable($Key, $Value, "Process")
    Write-Host "已设置环境变量: $Key = $Value" -ForegroundColor Green
}

# 主逻辑
switch ($Action.ToLower()) {
    "show" {
        Show-Config $ConfigFile
    }
    "new" {
        New-Config $ConfigFile
    }
    "edit" {
        Edit-Config $ConfigFile
    }
    "test" {
        Test-Config $ConfigFile
    }
    "env" {
        Show-Environment
    }
    "set" {
        if ($args.Count -ge 2) {
            Set-Environment $args[0] $args[1]
        } else {
            Write-Host "用法: .\config_manager.ps1 set <变量名> <值>" -ForegroundColor Yellow
        }
    }
    default {
        Write-Host "绳包管理器配置管理工具" -ForegroundColor Green
        Write-Host "======================" -ForegroundColor Green
        Write-Host ""
        Write-Host "用法:" -ForegroundColor Yellow
        Write-Host "  .\config_manager.ps1 show     - 显示当前配置" -ForegroundColor White
        Write-Host "  .\config_manager.ps1 new      - 创建默认配置文件" -ForegroundColor White
        Write-Host "  .\config_manager.ps1 edit     - 编辑配置文件" -ForegroundColor White
        Write-Host "  .\config_manager.ps1 test     - 测试配置文件语法" -ForegroundColor White
        Write-Host "  .\config_manager.ps1 env      - 显示环境变量" -ForegroundColor White
        Write-Host "  .\config_manager.ps1 set <k> <v> - 设置环境变量" -ForegroundColor White
        Write-Host ""
        Write-Host "示例:" -ForegroundColor Yellow
        Write-Host "  .\config_manager.ps1 set PORT 9090" -ForegroundColor White
        Write-Host "  .\config_manager.ps1 set HOST 0.0.0.0" -ForegroundColor White
    }
} 
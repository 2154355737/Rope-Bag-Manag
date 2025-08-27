# 绳包管理器后端使用指南

## 🚀 快速启动

### 开发环境启动
```bash
# 普通启动
.\tools\scripts\start.bat

# 带日志启动
.\tools\scripts\start_with_logs.bat

# 带安全功能启动
.\tools\scripts\start_with_security.bat

# 生产环境启动
.\tools\scripts\start_release.bat
```

## 🛠️ 工具脚本使用

### 数据库工具 (`tools\database\`)

#### 数据库检查工具
```bash
# 检查数据库架构
cargo run --bin tools\database\check_db_schema.rs

# 检查数据库结构
cargo run --bin tools\database\check_db_structure.rs

# 检查表结构
cargo run --bin tools\database\check_tables.rs
```

#### 数据库修复工具
```bash
# 修复邮件启用状态
cargo run --bin tools\database\fix_mail_enable.rs

# 修复用户状态
cargo run --bin tools\database\fix_user_status.rs

# 修复标签表
cargo run --bin tools\database\fix_tags_table.rs

# 修复结构字段
cargo run --bin tools\database\fix_struct_fields.rs

# 修复向量字段
cargo run --bin tools\database\fix_vec_fields.rs
```

#### 数据库管理工具
```bash
# 数据库迁移
cargo run --bin tools\database\migrate_db.rs

# 应用数据库补丁
cargo run --bin tools\database\patch_database.rs
# 或使用批处理脚本
.\tools\scripts\patch_database.bat

# 初始化安全表
cargo run --bin tools\database\init_security_tables.rs

# 应用发布迁移
cargo run --bin tools\database\apply_publish_migration.rs
# 或使用批处理脚本
.\tools\scripts\run_publish_migration.bat
```

### 管理员工具 (`tools\admin\`)

```bash
# 生成管理员密码
cargo run --bin tools\admin\generate_admin_password.rs

# 验证密码
cargo run --bin tools\admin\verify_password.rs

# 检查用户操作
cargo run --bin tools\admin\check_user_actions.rs

# 调试用户操作
cargo run --bin tools\admin\debug_user_actions.rs

# 更新标签计数
cargo run --bin tools\admin\update_tag_counts.rs
```

### 标签工具 (`tools\tags\`)

```bash
# 检查英文标签 (Python脚本)
python tools\tags\check_english_tags.py

# 完整替换标签 (Python脚本)
python tools\tags\complete_replace_tags.py
```

### 测试工具 (`tools\test\`)

```bash
# 简单测试
cargo run --bin tools\test\simple_test.rs

# 邮件测试
cargo run --bin tools\test\test_163_mail.rs
cargo run --bin tools\test\simple_smtp_test.rs
cargo run --bin tools\test\smtp_test_tool.rs

# 连接测试
cargo run --bin tools\test\test_alist_connection.rs

# 认证测试
cargo run --bin tools\test\test_auth.rs

# API端点测试
cargo run --bin tools\test\test_api_endpoints.rs

# 下载安全测试
cargo run --bin tools\test\test_download_security.rs
cargo run --bin tools\test\test_download_security_api.rs
cargo run --bin tools\test\test_download_security_simple.rs

# IP禁令测试
cargo run --bin tools\test\test_ip_ban.rs
cargo run --bin tools\test\test_ip_bans.rs

# 用户操作API测试
cargo run --bin tools\test\test_user_actions_api.rs
```

## 📂 SQL脚本使用

### 初始化脚本 (`sql\init\`)
```bash
# 创建缺失的表
sqlite3 data.db < sql\init\create_missing_tables.sql

# 创建安全表
sqlite3 data.db < sql\init\create_security_tables.sql

# 创建视图跟踪表
sqlite3 data.db < sql\init\create_view_tracking_tables.sql
```

### 迁移脚本 (`sql\migrations\`)
```bash
# 应用迁移
sqlite3 data.db < sql\migrations\apply_migration.sql
```

### 补丁脚本 (`sql\patches\`)
```bash
# 检查并修复数据库
sqlite3 data.db < sql\patches\check_and_fix_db.sql

# 修复包表
sqlite3 data.db < sql\patches\fix_packages_table.sql

# 修复用户操作表
sqlite3 data.db < sql\patches\fix_user_actions_table.sql
```

### 测试数据 (`sql\test\`)
```bash
# 创建示例数据
sqlite3 data.db < sql\test\create_sample_data.sql

# 创建测试帖子
sqlite3 data.db < sql\test\create_test_posts.sql

# 添加管理员测试数据
sqlite3 data.db < sql\test\add_admin_test_data.sql
```

## 📊 部署和维护

### 自动部署
```bash
.\tools\scripts\自动部署.bat
```

### 日志查看
```bash
# Linux系统
.\tools\scripts\test_logs.sh

# Windows系统
type logs\*.log
```

## ⚠️ 重要提示

1. **数据库备份**: 运行任何数据库工具前，请先备份数据库文件
2. **权限检查**: 确保有足够的文件读写权限
3. **依赖检查**: 运行Python脚本前确保已安装相关依赖
4. **测试环境**: 建议先在测试环境运行工具脚本

## 🔧 故障排除

如果遇到问题，请按以下顺序检查：
1. 检查数据库文件是否存在且可访问
2. 检查配置文件 `config.toml` 是否正确
3. 查看 `logs/` 目录下的日志文件
4. 参考 `docs/` 目录下的相关文档

## 📚 更多文档

详细的功能文档请查看 `docs/` 目录下的相关文件：
- 数据库补丁说明: `docs/DATABASE_PATCH_README.md`
- 下载安全功能: `docs/README_DOWNLOAD_SECURITY.md`
- 邮件设置指南: `docs/README_EMAIL_SETUP.md`
- 标签计数说明: `docs/README_TAG_COUNTS.md`
- 日志记录指南: `docs/LOGGING_GUIDE.md`
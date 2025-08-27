# 绳包管理器后端项目结构说明

## 📁 整理后的目录结构

```
rope-manager-backend/
├── 📁 src/                        # 核心源代码
│   ├── api/                       # API路由
│   ├── middleware/                # 中间件
│   ├── models/                    # 数据模型
│   ├── repositories/              # 数据访问层
│   ├── services/                  # 业务逻辑层
│   ├── utils/                     # 工具函数
│   └── main.rs                    # 程序入口
│
├── 📁 tools/                      # 工具脚本目录
│   ├── 📁 database/              # 数据库相关工具
│   │   ├── check_db_schema.rs    # 数据库架构检查
│   │   ├── check_db_structure.rs # 数据库结构检查
│   │   ├── patch_database.rs     # 数据库补丁工具
│   │   ├── migrate_db.rs         # 数据库迁移工具
│   │   ├── fix_*.rs              # 各种修复工具
│   │   └── init_security_tables.rs # 安全表初始化
│   │
│   ├── 📁 admin/                 # 管理员工具
│   │   ├── generate_admin_password.rs # 生成管理员密码
│   │   ├── verify_password.rs    # 密码验证工具
│   │   ├── check_user_actions.rs # 用户行为检查
│   │   ├── debug_user_actions.rs # 用户行为调试
│   │   └── update_tag_counts.rs  # 更新标签计数
│   │
│   ├── 📁 test/                  # 测试工具
│   │   ├── test_*.rs            # 各种功能测试脚本
│   │   ├── simple_test.rs       # 简单测试
│   │   └── smtp_test_tool.rs    # 邮件测试工具
│   │
│   ├── 📁 tags/                  # 标签管理工具
│   │   ├── check_english_tags.py # 检查英文标签
│   │   └── complete_replace_tags.py # 标签替换工具
│   │
│   └── 📁 scripts/               # 部署和启动脚本
│       ├── start.bat            # 普通启动
│       ├── start_release.bat    # 发布版启动
│       ├── start_with_logs.bat  # 带日志启动
│       ├── start_with_security.bat # 带安全检查启动
│       ├── patch_database.bat   # 数据库补丁脚本
│       ├── run_publish_migration.bat # 发布迁移脚本
│       ├── 自动部署.bat        # 自动部署脚本
│       └── test_logs.sh         # 日志测试脚本
│
├── 📁 sql/                       # 数据库脚本（重新组织）
│   ├── 📁 init/                 # 初始化脚本
│   │   ├── create_missing_tables.sql # 创建缺失表
│   │   ├── create_security_tables.sql # 创建安全表
│   │   └── create_view_tracking_tables.sql # 创建视图跟踪表
│   │
│   ├── 📁 migrations/           # 迁移脚本
│   │   ├── apply_migration.sql  # 应用迁移
│   │   ├── migrate_*.sql        # 各种迁移脚本
│   │   └── ...
│   │
│   ├── 📁 patches/              # 补丁修复脚本
│   │   ├── check_and_fix_db.sql # 检查和修复数据库
│   │   ├── fix_packages_table.sql # 修复包表
│   │   ├── fix_user_actions_table.sql # 修复用户行为表
│   │   └── ...
│   │
│   └── 📁 test/                 # 测试数据脚本
│       ├── create_sample_data.sql # 创建示例数据
│       ├── create_test_posts.sql # 创建测试帖子
│       ├── add_admin_test_data.sql # 添加管理员测试数据
│       └── ...
│
├── 📁 docs/                      # 项目文档
│   ├── DATABASE_PATCH_README.md # 数据库补丁说明
│   ├── README_DOWNLOAD_SECURITY.md # 下载安全说明
│   ├── README_EMAIL_SETUP.md    # 邮件设置说明
│   ├── LOGGING_GUIDE.md         # 日志指南
│   ├── IP封禁系统实现总结.md   # IP封禁系统文档
│   ├── 功能更新总结.md         # 功能更新文档
│   └── ...（其他功能总结文档）
│
├── 📁 data/                      # 数据文件
├── 📁 logs/                      # 日志文件
├── 📁 backups/                   # 备份文件
├── 📁 uploads/                   # 上传文件
├── 📁 sqlite-tools/              # SQLite工具
├── 📁 fq/                        # 特殊数据目录
├── 📁 temp/                      # 临时文件
├── 📁 target/                    # Rust编译输出
├── 📁 target2/                   # 备用编译输出
│
├── 📄 Cargo.toml                 # Rust项目配置
├── 📄 config.toml                # 运行时配置
├── 📄 openapi.yaml              # API文档
├── 📄 data.db                   # 主数据库
├── 📄 data.db.backup            # 数据库备份
├── 📄 headers.json              # 请求头配置
└── 📄 PROJECT_STRUCTURE.md      # 本文档
```

## 🎯 工具使用说明

### 数据库工具 (`tools/database/`)
- **检查类**: `check_db_*.rs` - 用于检查数据库状态
- **修复类**: `fix_*.rs` - 用于修复数据库问题
- **迁移类**: `migrate_db.rs`, `patch_database.rs` - 用于数据库迁移和补丁

### 管理员工具 (`tools/admin/`)
- **密码管理**: `generate_admin_password.rs`, `verify_password.rs`
- **用户管理**: `check_user_actions.rs`, `debug_user_actions.rs`
- **系统维护**: `update_tag_counts.rs`

### 测试工具 (`tools/test/`)
- **功能测试**: `test_*.rs` - 各种功能模块的测试
- **连接测试**: `test_alist_connection.rs`, `test_163_mail.rs`
- **API测试**: `test_api_endpoints.rs`, `test_auth.rs`

### 启动脚本 (`tools/scripts/`)
- **开发环境**: `start.bat`
- **生产环境**: `start_release.bat`
- **调试模式**: `start_with_logs.bat`
- **安全模式**: `start_with_security.bat`

## 🚀 快速启动

### 开发环境
```bash
cd tools/scripts
./start.bat
```

### 生产环境
```bash
cd tools/scripts
./start_release.bat
```

### 数据库维护
```bash
cd tools/scripts
./patch_database.bat
```

## 📋 注意事项

1. **数据库脚本执行顺序**:
   - 首先执行 `sql/init/` 中的初始化脚本
   - 然后执行 `sql/migrations/` 中的迁移脚本
   - 最后根据需要执行 `sql/patches/` 中的补丁脚本

2. **工具脚本使用**:
   - 数据库相关工具在 `tools/database/` 目录
   - 管理员工具在 `tools/admin/` 目录
   - 测试工具在 `tools/test/` 目录

3. **文档参考**:
   - 所有项目文档都在 `docs/` 目录中
   - 具体功能说明请查看对应的README文件

4. **备份重要性**:
   - 执行任何数据库操作前，请确保数据库已备份
   - 备份文件存储在 `backups/` 目录中

## 📚 相关文档

- [数据库补丁说明](docs/DATABASE_PATCH_README.md)
- [下载安全配置](docs/README_DOWNLOAD_SECURITY.md)
- [邮件设置指南](docs/README_EMAIL_SETUP.md)
- [日志使用指南](docs/LOGGING_GUIDE.md)
- [功能更新总结](docs/功能更新总结.md)
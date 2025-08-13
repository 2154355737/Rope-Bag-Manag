#!/bin/bash

# 测试后端日志系统
echo "🧪 测试后端日志系统..."

# 等待服务器启动
sleep 3

# 测试各种API请求来查看日志效果
echo "📊 发送测试请求..."

# 1. 正常请求
curl -s "http://127.0.0.1:15201/api/v1/public/categories" > /dev/null

# 2. 认证请求
curl -s -H "Authorization: Bearer invalid_token" "http://127.0.0.1:15201/api/v1/users/profile" > /dev/null

# 3. 不存在的端点 (404)
curl -s "http://127.0.0.1:15201/api/v1/nonexistent" > /dev/null

# 4. 复杂查询参数
curl -s "http://127.0.0.1:15201/api/v1/packages?page=1&page_size=10&search=test" > /dev/null

echo "✅ 测试完成，请查看服务器日志输出" 
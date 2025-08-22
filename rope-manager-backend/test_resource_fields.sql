-- 为现有资源添加测试数据
-- 用于测试前端显示功能

-- 为资源21添加测试数据
UPDATE packages SET 
  screenshots = '["http://localhost:15201/image/结绳社区/结绳-绳包/21-screenshot1.png", "http://localhost:15201/image/结绳社区/结绳-绳包/21-screenshot2.png"]',
  requirements = '["Windows 10+", "iOS 12+", "Android 8.0+", "至少2GB内存"]'
WHERE id = 21;

-- 为资源23添加测试数据  
UPDATE packages SET 
  screenshots = '["http://localhost:15201/image/结绳社区/默认分类/23-demo.jpg"]',
  requirements = '["支持Web浏览器", "JavaScript ES6+"]'
WHERE id = 23;

-- 为资源28添加测试数据
UPDATE packages SET 
  screenshots = '["http://localhost:15201/image/结绳社区/工具类/28-preview1.png", "http://localhost:15201/image/结绳社区/工具类/28-preview2.png", "http://localhost:15201/image/结绳社区/工具类/28-preview3.png"]',
  requirements = '["React 16.8+", "TypeScript 4.0+", "Node.js 14+"]'
WHERE id = 28;

-- 验证更新结果
SELECT id, name, screenshots, requirements FROM packages WHERE id IN (21, 23, 28); 
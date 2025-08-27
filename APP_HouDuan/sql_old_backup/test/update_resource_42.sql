-- 更新资源42的测试数据
UPDATE packages SET
  screenshots = '["http://localhost:15201/test1.png", "http://localhost:15201/test2.png"]',
  requirements = '["Windows 10+", "4GB内存", "100MB空间"]'
WHERE id = 42;

-- 验证更新结果
SELECT id, name, screenshots, requirements FROM packages WHERE id = 42; 
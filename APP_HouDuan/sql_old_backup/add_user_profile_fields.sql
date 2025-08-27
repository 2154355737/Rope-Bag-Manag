-- 添加用户资料相关缺失字段（如果已存在，执行会报错，可忽略该错误并继续下一条）
ALTER TABLE users ADD COLUMN email TEXT;
ALTER TABLE users ADD COLUMN bio TEXT;
ALTER TABLE users ADD COLUMN location TEXT;
ALTER TABLE users ADD COLUMN website TEXT;
ALTER TABLE users ADD COLUMN skills TEXT; 
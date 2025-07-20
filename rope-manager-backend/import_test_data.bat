@echo off
echo 导入资源记录测试数据...
sqlite3 data.db < sql/test_data_resource_records.sql
echo 测试数据导入完成
pause 
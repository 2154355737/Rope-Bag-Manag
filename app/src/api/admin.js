import { get } from '../utils/request';

export const adminApi = {
  // 获取应用每日启动量统计
  getAppLaunchDailyStats: (days = 30) => get('/admin/app/launch-daily-stats', { days }),
  // 获取DAU统计
  getDauStats: (days = 30) => get('/admin/app/dau-stats', { days })
};

export default { adminApi }; 
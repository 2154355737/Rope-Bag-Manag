import { post } from '../utils/request';

// 应用相关API
export const appApi = {
  // 上报应用启动
  reportLaunch: async () => {
    try {
      // 获取用户ID（如果已登录）
      const storedUserInfo = localStorage.getItem('userInfo');
      let userId = null;
      if (storedUserInfo) {
        try {
          const parsed = JSON.parse(storedUserInfo);
          userId = parsed && typeof parsed.id === 'number' ? parsed.id : null;
        } catch (_) {}
      }

      // 生成/读取持久化设备ID
      let deviceId = localStorage.getItem('device_id');
      if (!deviceId) {
        try {
          deviceId = (crypto && typeof crypto.randomUUID === 'function')
            ? crypto.randomUUID()
            : Math.random().toString(36).slice(2) + Date.now().toString(36);
        } catch (_) {
          deviceId = Math.random().toString(36).slice(2) + Date.now().toString(36);
        }
        localStorage.setItem('device_id', deviceId);
      }

      // 平台识别
      const isTauri = typeof window !== 'undefined' && window.__TAURI__ !== undefined;
      let platform = 'web';
      if (isTauri) {
        platform = /Android/i.test(navigator.userAgent) ? 'android' : 'desktop';
      }

      // 应用版本（优先环境变量，Tauri环境下也可为空）
      let appVersion = null;
      if (import.meta && import.meta.env) {
        appVersion = import.meta.env.VITE_APP_VERSION || null;
      }

      const payload = {
        user_id: userId,
        device_id: deviceId,
        app_version: appVersion,
        platform
      };

      return await post('/public/app/launch', payload);
    } catch (e) {
      console.warn('App launch report failed:', e);
      return null;
    }
  }
};

export default appApi; 
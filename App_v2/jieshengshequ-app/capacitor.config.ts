import { CapacitorConfig } from '@capacitor/cli';

const config: CapacitorConfig = {
  appId: 'com.jieshengshequ.app',
  appName: '结绳社区',
  webDir: 'dist',
  server: {
    androidScheme: 'http',
    // 在生产环境中清除任何服务器配置，确保使用构建后的静态文件
    cleartext: true
  },
  android: {
    allowMixedContent: true,
    webContentsDebuggingEnabled: true,
    appendUserAgent: 'CapacitorApp'
  },
  plugins: {
    Keyboard: {
      resize: 'body',
      resizeOnFullScreen: false,
    },
  },
};

export default config; 
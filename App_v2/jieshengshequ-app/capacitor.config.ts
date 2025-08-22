import { CapacitorConfig } from '@capacitor/cli';

const config: CapacitorConfig = {
  appId: 'com.jieshengshequ.app',
  appName: '结绳社区',
  webDir: 'dist',
  server: {
    androidScheme: 'http'
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
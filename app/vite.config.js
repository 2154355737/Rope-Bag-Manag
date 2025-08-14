import { defineConfig, loadEnv } from 'vite';
import vue from '@vitejs/plugin-vue';
import { fileURLToPath, URL } from 'node:url';
import Components from 'unplugin-vue-components/vite';
import { VantResolver } from '@vant/auto-import-resolver';
import Icons from 'unplugin-icons/vite';
import IconsResolver from 'unplugin-icons/resolver';
import { readFileSync } from 'fs';

// 读取package.json中的版本号
const packageJson = JSON.parse(readFileSync('./package.json', 'utf-8'));
const appVersion = packageJson.version;

// https://vitejs.dev/config/
export default defineConfig(({ command, mode }) => {
  // 加载环境变量
  const env = loadEnv(mode, process.cwd(), '');
  
  // 检测是否为Tauri构建
  const isTauri = mode === 'tauri' || process.env.TAURI_PLATFORM !== undefined;
  
  // 为Tauri构建设置默认的API地址（如果没有在环境变量中指定）
  if (isTauri && !env.VITE_API_BASE_URL) {
    // 在Tauri构建中，如果没有设置环境变量，使用生产服务器地址
    process.env.VITE_API_BASE_URL = 'http://39.105.113.219:15201/api/v1';
  }

  const config = {
    plugins: [
      vue(),
      Components({
        resolvers: [
          VantResolver(),
          IconsResolver({
            prefix: 'i', // 使用 i- 前缀，例如 i-mdi-home
          }),
        ],
      }),
      Icons({
        autoInstall: true,
      }),
    ],
    resolve: {
      alias: {
        '@': fileURLToPath(new URL('./src', import.meta.url))
      }
    },
    // 定义全局常量
    define: {
      __TAURI_BUILD__: isTauri,
      __DEV__: command === 'serve',
      __APP_VERSION__: JSON.stringify(appVersion),
    },
  };

  // 开发环境配置
  if (command === 'serve' && !isTauri) {
    config.server = {
      port: 5173,
      proxy: {
        '/api': {
          target: env.VITE_PROXY_TARGET || 'http://127.0.0.1:15201',
          changeOrigin: true,
          rewrite: (path) => path.replace(/^\/api/, '/api/v1')
        }
      }
    };
  }

  // Tauri构建特殊配置
  if (isTauri) {
    config.build = {
      ...config.build,
      // Tauri构建优化
      target: 'esnext',
      minify: 'esbuild',
      rollupOptions: {
        output: {
          manualChunks: undefined,
        },
      },
    };
  }

  return config;
}); 
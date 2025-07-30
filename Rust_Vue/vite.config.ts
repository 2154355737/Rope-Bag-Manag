import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
  ],
  server: {
    proxy: {
      // API代理 - 匹配 /api 开头的所有请求
      '/api': {
        target: 'http://127.0.0.1:15201',
        changeOrigin: true,
        // 保持 /api 前缀，后端路由需要此前缀
        rewrite: (path) => path
      },
      // 文件上传/下载代理 - 匹配 /uploads 开头的所有请求
      '/uploads': {
        target: 'http://127.0.0.1:15201',
        changeOrigin: true,
        // 保持 /uploads 前缀
        rewrite: (path) => path
      },
      // ESM模块代理（如果需要）
      '/esm': {
        target: 'https://esm.sh/',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/esm/, '')
      }
    },
    // 开发服务器配置
    host: '0.0.0.0', // 允许外部访问
    port: 5173,      // 默认端口
    // 防止网络问题
    hmr: {
      overlay: false
    }
  },
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
  build: {
    // 代码分割配置
    rollupOptions: {
      output: {
        // 手动分割代码块
        manualChunks: {
          // Vue核心库
          'vue-vendor': ['vue', 'vue-router'],
          
          // Element Plus UI库
          'element-plus': ['element-plus'],
          
          // ECharts图表库
          'echarts': ['echarts', 'vue-echarts'],
          
          // 工具库
          'utils': ['axios', 'lucide-vue-next']
        },
        
        // 文件名配置
        chunkFileNames: (chunkInfo) => {
          const facadeModuleId = chunkInfo.facadeModuleId
          if (facadeModuleId) {
            if (facadeModuleId.includes('node_modules')) {
              return 'vendor/[name]-[hash].js'
            }
            if (facadeModuleId.includes('views/')) {
              return 'pages/[name]-[hash].js'
            }
          }
          return 'chunks/[name]-[hash].js'
        },
        
        // 资源文件配置
        assetFileNames: (assetInfo) => {
          const info = assetInfo.name?.split('.') || []
          const ext = info[info.length - 1]
          if (/\.(css)$/.test(assetInfo.name || '')) {
            return 'css/[name]-[hash].[ext]'
          }
          if (/\.(png|jpe?g|gif|svg|webp)$/.test(assetInfo.name || '')) {
            return 'images/[name]-[hash].[ext]'
          }
          return 'assets/[name]-[hash].[ext]'
        }
      }
    },
    
    // 资源大小限制
    chunkSizeWarningLimit: 1000,
    
    // 启用源码映射（开发环境）
    sourcemap: false
  },
  
  // 优化依赖预构建
  optimizeDeps: {
    include: [
      'vue',
      'vue-router',
      'element-plus',
      'echarts',
      'axios',
      'lucide-vue-next'
    ],
    exclude: []
  }
})

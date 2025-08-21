import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  server: {
    port: 3000, // 前端使用3000端口，避免与后端8002冲突
    strictPort: false, // 如果端口被占用，会自动尝试下一个可用端口
    open: true, // 自动打开浏览器
    host: '0.0.0.0', // 允许外部访问
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:15201',
        changeOrigin: true,
        rewrite: (path) => path,
      },
      '/uploads': {
        target: 'http://127.0.0.1:15201',
        changeOrigin: true,
        rewrite: (path) => path,
      },
      '/health': {
        target: 'http://127.0.0.1:15201',
        changeOrigin: true,
        rewrite: (path) => path,
      },
    },
    fs: {
      strict: false // 允许访问工作区外的文件
    }
  },
  // 优化构建
  optimizeDeps: {
    include: ['react', 'react-dom']
  },
})
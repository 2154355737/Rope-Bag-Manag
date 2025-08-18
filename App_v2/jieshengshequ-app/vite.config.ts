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
    port: 8082, // 使用8082端口
    strictPort: false, // 如果端口被占用，会自动尝试下一个可用端口
    open: true, // 自动打开浏览器
    historyApiFallback: true, // 支持 history 路由模式
  },
})
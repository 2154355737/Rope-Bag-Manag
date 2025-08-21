<template>
  <footer class="home-footer">
    <div class="footer-container">
      <div class="footer-content">
        <div class="footer-brand">
          <div class="brand-icon">🏘️</div>
          <div>
            <h3>{{ systemSettings?.hero_title || '社区平台' }}</h3>
            <p>{{ systemSettings?.hero_subtitle || '资源分享与交流' }}</p>
          </div>
        </div>
        
        <div class="footer-links">
          <div class="link-group">
            <h4>社区</h4>
            <a href="/about">关于我们</a>
            <a href="/help">帮助中心</a>
            <a href="/contact">联系我们</a>
          </div>
          <div class="link-group">
            <h4>开发者</h4>
            <a href="/api-docs">API 文档</a>
            <a href="/github" target="_blank">GitHub</a>
          </div>
          <div class="link-group">
            <h4>支持</h4>
            <a href="/feedback">意见反馈</a>
            <a href="/terms">服务条款</a>
            <a href="/privacy">隐私政策</a>
          </div>
        </div>
      </div>
      
      <div class="footer-bottom">
        <p>{{ systemSettings?.copyright_text || '© 2024 社区平台. All rights reserved.' }}</p>
        <div class="footer-social">
          <el-button link size="small">
            <el-icon><Message /></el-icon>
          </el-button>
          <el-button link size="small">
            <el-icon><Share /></el-icon>
          </el-button>
        </div>
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
import { Message, Share } from '@element-plus/icons-vue'

// Props
interface Props {
  systemSettings?: any
}

const props = defineProps<Props>()
</script>

<style scoped>
.home-footer {
  background: var(--bg-elevated);
  border-top: 1px solid var(--border-color);
  margin-top: 64px;
  padding: 48px 0 24px 0;
}

.footer-container {
  max-width: 1440px;
  margin: 0 auto;
  padding: 0 24px;
}

.footer-content {
  display: grid;
  grid-template-columns: 1fr 2fr;
  gap: 48px;
  margin-bottom: 32px;
}

.footer-brand {
  display: flex;
  align-items: flex-start;
  gap: 16px;
}

.brand-icon {
  font-size: 24px;
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--brand-color), var(--brand-color-light));
  border-radius: 12px;
  flex-shrink: 0;
}

.footer-brand h3 {
  font-size: 18px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 8px 0;
}

.footer-brand p {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
  line-height: 1.5;
}

.footer-links {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 32px;
}

.link-group h4 {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 16px 0;
}

.link-group a {
  display: block;
  font-size: 14px;
  color: var(--text-secondary);
  text-decoration: none;
  margin-bottom: 8px;
  padding: 4px 0;
  transition: color 0.2s ease;
}

.link-group a:hover {
  color: var(--brand-color);
}

.footer-bottom {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-top: 24px;
  border-top: 1px solid var(--border-color);
}

.footer-bottom p {
  font-size: 14px;
  color: var(--text-muted);
  margin: 0;
}

.footer-social {
  display: flex;
  align-items: center;
  gap: 8px;
}

.footer-social .el-button {
  width: 36px;
  height: 36px;
  border-radius: 8px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  color: var(--text-secondary);
  transition: all 0.2s ease;
}

.footer-social .el-button:hover {
  background: var(--brand-color);
  border-color: var(--brand-color);
  color: white;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .footer-content {
    grid-template-columns: 1fr;
    gap: 32px;
  }
  
  .footer-links {
    grid-template-columns: 1fr;
    gap: 24px;
  }
  
  .footer-brand {
    text-align: center;
    flex-direction: column;
    gap: 12px;
  }
  
  .footer-bottom {
    flex-direction: column;
    gap: 16px;
    text-align: center;
  }
}

@media (max-width: 480px) {
  .home-footer {
    padding: 32px 0 16px 0;
  }
  
  .footer-container {
    padding: 0 16px;
  }
  
  .footer-links {
    gap: 16px;
  }
}
</style> 
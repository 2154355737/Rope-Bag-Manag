<template>
  <div class="about-page">
    <van-nav-bar title="关于我们" left-arrow @click-left="onBack" fixed />
    <div class="content">
      <div class="card">
        <div class="title">基础信息</div>
        <van-cell-group inset>
          <van-cell title="运行环境" :value="envDisplay" />
          <van-cell title="开发人员" value="Muteduanxing" />
          <van-cell title="官方QQ群" value="616300355" />
          <van-cell title="当前版本" value="bate 1.0.0" />
        </van-cell-group>
      </div>

      <div class="card">
        <div class="title">项目介绍</div>
        <p class="desc">
          本应用用于资源分享与交流，包含资源发布、评论、订阅与通知等功能。感谢每一位参与者与使用者的支持！
        </p>
      </div>

      <div class="card">
        <div class="title">联系我们</div>
        <p class="desc">如需反馈问题或提出建议，欢迎加入 QQ 群交流：<span class="highlight">616300355</span>。</p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const onBack = () => router.back()

// 运行环境展示（来源：构建模式 + 简要UA）
const envDisplay = computed(() => {
  const mode = import.meta.env.MODE || 'production'
  return mode
})
</script>

<style scoped>
.about-page { min-height: 100vh; background: var(--background-color); }
.content { 
  padding: 12px; 
}
.card { background: #fff; border-radius: 12px; padding: 12px; margin-bottom: 12px; }
.title { font-size: 16px; font-weight: 600; color: var(--text-color); margin-bottom: 8px; }
.desc { font-size: 14px; color: var(--text-color); line-height: 1.6; }
.highlight { color: var(--primary-color); font-weight: 600; }
</style> 
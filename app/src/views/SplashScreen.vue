<template>
  <div class="splash-screen" :class="{ 'fade-out': isExiting }" @click="handleSkip" @touchstart="handleSkip">
    <!-- 背景渐变 -->
    <div class="background-gradient"></div>
    
    <!-- 动画粒子背景 -->
    <div class="particles-container">
      <div v-for="i in 25" :key="i" class="particle" :style="getParticleStyle(i)"></div>
    </div>
    
    <!-- 光圈效果 -->
    <div class="light-rings">
      <div class="ring ring-1"></div>
      <div class="ring ring-2"></div>
      <div class="ring ring-3"></div>
    </div>
    
    <!-- Logo容器 -->
    <div class="logo-container" :class="{ 'animate': isAnimating }">
      <!-- 主Logo -->
      <div class="logo-main">
        <div class="logo-icon">
          <!-- 使用SVG创建一个现代化的logo -->
          <svg viewBox="0 0 120 120" class="logo-svg">
            <!-- 外圈 -->
            <circle cx="60" cy="60" r="50" fill="none" stroke="url(#gradient1)" stroke-width="3" class="circle-outer"/>
            <!-- 内圈 -->
            <circle cx="60" cy="60" r="35" fill="none" stroke="url(#gradient2)" stroke-width="2" class="circle-inner"/>
            <!-- 中心图标 -->
            <path d="M40 45 L60 35 L80 45 L80 75 L60 85 L40 75 Z" fill="url(#gradient3)" class="center-shape"/>
            <!-- 渐变定义 -->
            <defs>
              <linearGradient id="gradient1" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style="stop-color:#a8e6cf"/>
                <stop offset="100%" style="stop-color:#dcedc1"/>
              </linearGradient>
              <linearGradient id="gradient2" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style="stop-color:#c8e6c9"/>
                <stop offset="100%" style="stop-color:#e8f5e8"/>
              </linearGradient>
              <linearGradient id="gradient3" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style="stop-color:#b2dfdb"/>
                <stop offset="100%" style="stop-color:#e0f2f1"/>
              </linearGradient>
            </defs>
          </svg>
        </div>
        
        <!-- 应用名称 -->
        <div class="app-name">
          <span class="name-text">绳包社区</span>
          <div class="name-subtitle">发现·分享·创造</div>
        </div>
      </div>
      
      <!-- 加载指示器 -->
      <div class="loading-container">
        <div class="loading-dots">
          <span class="dot"></span>
          <span class="dot"></span>
          <span class="dot"></span>
        </div>
        <div class="loading-text">{{ loadingText }}</div>
      </div>
    </div>
    
    <!-- 版本信息 -->
    <div class="version-info">
      <span>v1.0.0</span>
    </div>
    
    <!-- 跳过提示 -->
    <div class="skip-hint" :class="{ 'show': isAnimating }">
      <span>轻触屏幕跳过</span>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { useRouter } from 'vue-router';
import { useUserStore } from '../store/user';

const router = useRouter();
const userStore = useUserStore();

const isAnimating = ref(false);
const isExiting = ref(false);
const isSkipped = ref(false);
const loadingText = ref('正在初始化...');
let animationTimer = null;
let exitTimer = null;

// 生成粒子样式
const getParticleStyle = (index) => {
  const size = Math.random() * 6 + 2;
  const left = Math.random() * 100;
  const animationDelay = Math.random() * 3;
  const animationDuration = 3 + Math.random() * 4;
  
  return {
    width: `${size}px`,
    height: `${size}px`,
    left: `${left}%`,
    animationDelay: `${animationDelay}s`,
    animationDuration: `${animationDuration}s`
  };
};

// 启动动画序列
const startAnimationSequence = async () => {
  // 延迟启动动画，让用户看到初始状态
  await new Promise(resolve => setTimeout(resolve, 300));
  isAnimating.value = true;
  
  // 获取目标路径
  const redirectPath = sessionStorage.getItem('redirectPath') || '/';
  
  try {
    // 更新加载状态
    loadingText.value = '正在检查登录状态...';
    
    // 1. 自动登录检查
    await userStore.checkAuth().catch(error => {
      console.log('启动页自动登录检查:', error);
    });
    
    // 2. 预加载目标页面
    loadingText.value = '正在预加载页面...';
    if (window.preloadTargetPage) {
      await window.preloadTargetPage(redirectPath);
    }
    
    // 3. 确保最小显示时间
    loadingText.value = '即将完成...';
    await new Promise(resolve => setTimeout(resolve, 800));
    
    // 4. 准备退出
    loadingText.value = '加载完成';
    await new Promise(resolve => setTimeout(resolve, 200));
    
  } catch (error) {
    console.warn('启动页加载过程出错:', error);
    loadingText.value = '加载完成';
  }
  
  // 开始退出动画
  isExiting.value = true;
  
  // 等待退出动画完成后跳转
  setTimeout(() => {
    sessionStorage.removeItem('redirectPath');
    router.replace(redirectPath);
  }, 600);
};

// 处理跳过启动页
const handleSkip = async () => {
  if (isSkipped.value || isExiting.value) return;
  isSkipped.value = true;
  
  // 获取目标路径并立即开始预加载
  const redirectPath = sessionStorage.getItem('redirectPath') || '/';
  
  // 快速预加载目标页面
  if (window.preloadTargetPage) {
    window.preloadTargetPage(redirectPath);
  }
  
  // 立即开始退出动画
  isExiting.value = true;
  
  // 稍微等待预加载完成
  setTimeout(() => {
    sessionStorage.removeItem('redirectPath');
    router.replace(redirectPath);
  }, 300); // 进一步缩短跳过时的等待时间
};

onMounted(() => {
  startAnimationSequence();
});

onUnmounted(() => {
  if (animationTimer) clearTimeout(animationTimer);
  if (exitTimer) clearTimeout(exitTimer);
});
</script>

<style scoped>
.splash-screen {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #a8e6cf 0%, #dcedc1 100%);
  z-index: 9999;
  overflow: hidden;
  transition: opacity 0.8s ease-out, transform 0.8s ease-out;
}

.splash-screen.fade-out {
  opacity: 0;
  transform: scale(1.05) translateY(-10px);
  filter: blur(1px);
}

.background-gradient {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: 
    radial-gradient(circle at 20% 80%, rgba(168, 230, 207, 0.3) 0%, transparent 50%),
    radial-gradient(circle at 80% 20%, rgba(220, 237, 193, 0.2) 0%, transparent 50%),
    radial-gradient(circle at 40% 40%, rgba(240, 248, 255, 0.1) 0%, transparent 50%);
  animation: gradientShift 6s ease-in-out infinite alternate;
}

@keyframes gradientShift {
  0% { transform: scale(1) rotate(0deg); }
  100% { transform: scale(1.1) rotate(5deg); }
}

/* 光圈效果 */
.light-rings {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  pointer-events: none;
}

.ring {
  position: absolute;
  border: 1px solid rgba(168, 230, 207, 0.2);
  border-radius: 50%;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.ring-1 {
  width: 200px;
  height: 200px;
  animation: ringPulse 4s ease-in-out infinite;
}

.ring-2 {
  width: 300px;
  height: 300px;
  animation: ringPulse 4s ease-in-out infinite 1.3s;
}

.ring-3 {
  width: 400px;
  height: 400px;
  animation: ringPulse 4s ease-in-out infinite 2.6s;
}

@keyframes ringPulse {
  0%, 100% {
    transform: translate(-50%, -50%) scale(0.8);
    opacity: 0;
  }
  50% {
    transform: translate(-50%, -50%) scale(1.2);
    opacity: 0.3;
  }
}

/* 粒子动画 */
.particles-container {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
}

.particle {
  position: absolute;
  background: rgba(168, 230, 207, 0.4);
  border-radius: 50%;
  animation: floatUp linear infinite;
}

@keyframes floatUp {
  0% {
    opacity: 0;
    transform: translateY(100vh) scale(0);
  }
  10% {
    opacity: 1;
    transform: translateY(90vh) scale(1);
  }
  90% {
    opacity: 1;
    transform: translateY(10vh) scale(1);
  }
  100% {
    opacity: 0;
    transform: translateY(0vh) scale(0);
  }
}

/* Logo容器 */
.logo-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  transform: translateY(20px);
  opacity: 0;
  transition: all 0.8s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

.logo-container.animate {
  transform: translateY(0);
  opacity: 1;
}

.logo-main {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-bottom: 40px;
}

.logo-icon {
  width: 120px;
  height: 120px;
  margin-bottom: 24px;
  animation: logoFloat 3s ease-in-out infinite;
}

@keyframes logoFloat {
  0%, 100% { transform: translateY(0px); }
  50% { transform: translateY(-10px); }
}

.logo-svg {
  width: 100%;
  height: 100%;
  filter: drop-shadow(0 10px 30px rgba(0, 0, 0, 0.3));
}

.circle-outer {
  stroke-dasharray: 314;
  stroke-dashoffset: 314;
  animation: drawCircle 2s ease-out 0.5s forwards;
}

.circle-inner {
  stroke-dasharray: 220;
  stroke-dashoffset: 220;
  animation: drawCircle 2s ease-out 1s forwards;
}

.center-shape {
  opacity: 0;
  transform: scale(0);
  animation: scaleIn 1s ease-out 1.5s forwards;
}

@keyframes drawCircle {
  to {
    stroke-dashoffset: 0;
  }
}

@keyframes scaleIn {
  to {
    opacity: 1;
    transform: scale(1);
  }
}

/* 应用名称 */
.app-name {
  text-align: center;
  color: white;
}

.name-text {
  font-size: 32px;
  font-weight: 700;
  background: linear-gradient(45deg, #2c3e50, #34495e);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  text-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
  display: block;
  margin-bottom: 8px;
  animation: textGlow 2s ease-in-out infinite alternate;
}

@keyframes textGlow {
  0% { text-shadow: 0 2px 10px rgba(0, 0, 0, 0.3); }
  100% { text-shadow: 0 2px 20px rgba(255, 255, 255, 0.4); }
}

.name-subtitle {
  font-size: 14px;
  font-weight: 400;
  color: rgba(60, 60, 60, 0.7);
  letter-spacing: 2px;
  opacity: 0;
  animation: fadeInUp 1s ease-out 2s forwards;
}

@keyframes fadeInUp {
  0% {
    opacity: 0;
    transform: translateY(10px);
  }
  100% {
    opacity: 1;
    transform: translateY(0);
  }
}

/* 加载指示器 */
.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  opacity: 0;
  animation: fadeIn 1s ease-out 2.5s forwards;
}

@keyframes fadeIn {
  to { opacity: 1; }
}

.loading-dots {
  display: flex;
  gap: 8px;
  margin-bottom: 16px;
}

.dot {
  width: 8px;
  height: 8px;
  background: rgba(60, 60, 60, 0.6);
  border-radius: 50%;
  animation: dotBounce 1.4s ease-in-out infinite both;
}

.dot:nth-child(1) { animation-delay: -0.32s; }
.dot:nth-child(2) { animation-delay: -0.16s; }

@keyframes dotBounce {
  0%, 80%, 100% {
    transform: scale(0.8);
    opacity: 0.5;
  }
  40% {
    transform: scale(1.2);
    opacity: 1;
  }
}

.loading-text {
  color: rgba(60, 60, 60, 0.7);
  font-size: 14px;
  font-weight: 300;
  letter-spacing: 1px;
}

/* 版本信息 */
.version-info {
  position: absolute;
  bottom: 40px;
  color: rgba(60, 60, 60, 0.5);
  font-size: 12px;
  font-weight: 300;
  opacity: 0;
  animation: fadeIn 1s ease-out 3s forwards;
}

/* 跳过提示 */
.skip-hint {
  position: absolute;
  bottom: 80px;
  left: 50%;
  transform: translateX(-50%);
  color: rgba(60, 60, 60, 0.6);
  font-size: 13px;
  font-weight: 300;
  opacity: 0;
  transition: opacity 0.5s ease;
  animation: pulseGlow 2s ease-in-out infinite;
}

.skip-hint.show {
  opacity: 1;
  animation-delay: 2s;
}

@keyframes pulseGlow {
  0%, 100% { 
    opacity: 0.7; 
    transform: translateX(-50%) scale(1);
  }
  50% { 
    opacity: 1; 
    transform: translateX(-50%) scale(1.05);
  }
}

/* 响应式设计 */
@media (max-width: 480px) {
  .logo-icon {
    width: 100px;
    height: 100px;
  }
  
  .name-text {
    font-size: 28px;
  }
  
  .name-subtitle {
    font-size: 12px;
  }
}

/* 深色模式适配 */
@media (prefers-color-scheme: dark) {
  .splash-screen {
    background: linear-gradient(135deg, #2d3748 0%, #4a5568 100%);
  }
  
  .name-text {
    background: linear-gradient(45deg, #fff, #f0f0f0);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }
  
  .name-subtitle,
  .loading-text,
  .version-info,
  .skip-hint {
    color: rgba(255, 255, 255, 0.8) !important;
  }
  
  .dot {
    background: rgba(255, 255, 255, 0.7) !important;
  }
}
</style> 
<template>
  <MobileLayout title="个人中心" :showBack="false">
    <div class="mobile-user-center">
      <!-- 用户基本信息 -->
      <div class="user-info-card">
        <div class="user-avatar">
          <img :src="userInfo.avatar || defaultAvatar" alt="用户头像">
        </div>
        <div class="user-details">
          <h2 class="username">{{ userInfo.username || '未登录' }}</h2>
          <p class="user-role">{{ getUserRoleText }}</p>
        </div>
        <div v-if="!isLoggedIn" class="login-actions">
          <el-button type="primary" @click="goToLogin">立即登录</el-button>
          <el-button @click="goToRegister">注册账号</el-button>
        </div>
      </div>

      <!-- 已登录用户功能菜单 -->
      <div v-if="isLoggedIn" class="user-menu">
        <div class="menu-section">
          <div class="section-title">我的内容</div>
          <div class="menu-list">
            <div class="menu-item" @click="navigateTo('/mobile/user/resources')">
              <el-icon><Folder /></el-icon>
              <span class="item-label">我的资源</span>
              <el-icon class="arrow-icon"><ArrowRight /></el-icon>
            </div>
            <div class="menu-item" @click="navigateTo('/mobile/user/comments')">
              <el-icon><ChatDotRound /></el-icon>
              <span class="item-label">我的评论</span>
              <el-icon class="arrow-icon"><ArrowRight /></el-icon>
            </div>
            <div class="menu-item" @click="navigateTo('/mobile/user/favorites')">
              <el-icon><Star /></el-icon>
              <span class="item-label">我的收藏</span>
              <el-icon class="arrow-icon"><ArrowRight /></el-icon>
            </div>
            <div class="menu-item" @click="navigateTo('/mobile/user/downloads')">
              <el-icon><Download /></el-icon>
              <span class="item-label">下载记录</span>
              <el-icon class="arrow-icon"><ArrowRight /></el-icon>
            </div>
          </div>
        </div>
        
        <div class="menu-section">
          <div class="section-title">账号设置</div>
          <div class="menu-list">
            <div class="menu-item" @click="navigateTo('/mobile/user/profile')">
              <el-icon><User /></el-icon>
              <span class="item-label">个人资料</span>
              <el-icon class="arrow-icon"><ArrowRight /></el-icon>
            </div>
            <div class="menu-item" @click="navigateTo('/mobile/user/security')">
              <el-icon><Lock /></el-icon>
              <span class="item-label">账号安全</span>
              <el-icon class="arrow-icon"><ArrowRight /></el-icon>
            </div>
            <div class="menu-item" @click="showThemeSelector = true">
              <el-icon><MoonNight /></el-icon>
              <span class="item-label">外观设置</span>
              <el-icon class="arrow-icon"><ArrowRight /></el-icon>
            </div>
          </div>
        </div>

        <!-- 管理员入口 -->
        <div v-if="isAdmin" class="menu-section">
          <div class="section-title">管理功能</div>
          <div class="menu-list">
            <div class="menu-item" @click="navigateTo('/admin')">
              <el-icon><Setting /></el-icon>
              <span class="item-label">后台管理</span>
              <el-icon class="arrow-icon"><ArrowRight /></el-icon>
            </div>
          </div>
        </div>

        <!-- 元老入口 -->
        <div v-if="isElder" class="menu-section">
          <div class="section-title">元老功能</div>
          <div class="menu-list">
            <div class="menu-item" @click="navigateTo('/elder')">
              <el-icon><Medal /></el-icon>
              <span class="item-label">元老后台</span>
              <el-icon class="arrow-icon"><ArrowRight /></el-icon>
            </div>
          </div>
        </div>

        <!-- 退出登录按钮 -->
        <div class="logout-section">
          <el-button type="danger" plain @click="handleLogout" class="logout-button">
            退出登录
          </el-button>
        </div>
      </div>

      <!-- 未登录用户展示区 -->
      <div v-else class="guest-section">
        <div class="menu-section">
          <div class="section-title">通用功能</div>
          <div class="menu-list">
            <div class="menu-item" @click="showThemeSelector = true">
              <el-icon><MoonNight /></el-icon>
              <span class="item-label">外观设置</span>
              <el-icon class="arrow-icon"><ArrowRight /></el-icon>
            </div>
            <div class="menu-item" @click="navigateTo('/mobile/about')">
              <el-icon><InfoFilled /></el-icon>
              <span class="item-label">关于我们</span>
              <el-icon class="arrow-icon"><ArrowRight /></el-icon>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 主题选择弹窗 -->
    <el-dialog v-model="showThemeSelector" title="外观设置" width="80%" center>
      <div class="theme-options">
        <div class="theme-option" :class="{active: currentTheme === 'auto'}" @click="setTheme('auto')">
          <div class="theme-icon auto-theme"></div>
          <div class="theme-label">跟随系统</div>
        </div>
        <div class="theme-option" :class="{active: currentTheme === 'light'}" @click="setTheme('light')">
          <div class="theme-icon light-theme"></div>
          <div class="theme-label">浅色模式</div>
        </div>
        <div class="theme-option" :class="{active: currentTheme === 'dark'}" @click="setTheme('dark')">
          <div class="theme-icon dark-theme"></div>
          <div class="theme-label">深色模式</div>
        </div>
      </div>
    </el-dialog>
  </MobileLayout>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { ElMessage, ElMessageBox } from 'element-plus';
import MobileLayout from '@/layouts/mobile/MobileLayout.vue';
import { 
  User, Folder, ChatDotRound, Star, Download, Setting, 
  Lock, MoonNight, InfoFilled, ArrowRight, Medal 
} from '@element-plus/icons-vue';

// 路由
const router = useRouter();

// 用户信息
const isLoggedIn = ref(false);
const isAdmin = ref(false);
const isElder = ref(false);
const defaultAvatar = '/assets/default-avatar.jpg';
const userInfo = ref({
  username: '',
  avatar: '',
  role: '',
  email: '',
});

// 主题选择
const showThemeSelector = ref(false);
const currentTheme = ref('auto');

// 计算属性
const getUserRoleText = computed(() => {
  if (!isLoggedIn.value) return '游客';
  
  const roleMap = {
    'admin': '管理员',
    'elder': '元老',
    'user': '用户',
  };
  
  return roleMap[userInfo.value.role] || '用户';
});

// 初始化
onMounted(async () => {
  // 加载主题
  loadThemeSettings();
  
  // 加载用户信息
  try {
    // 实际项目中使用API
    // const response = await api.getUserInfo();
    // if (response.data) {
    //   userInfo.value = response.data;
    //   isLoggedIn.value = true;
    //   isAdmin.value = response.data.role === 'admin';
    //   isElder.value = response.data.role === 'elder';
    // }
    
    // 模拟数据
    const token = localStorage.getItem('token');
    if (token) {
      isLoggedIn.value = true;
      const userRole = localStorage.getItem('userRole') || 'user';
      userInfo.value = {
        username: localStorage.getItem('username') || '测试用户',
        avatar: '',
        role: userRole,
        email: 'user@example.com',
      };
      isAdmin.value = userRole === 'admin';
      isElder.value = userRole === 'elder';
    }
  } catch (error) {
    console.error('获取用户信息失败:', error);
  }
});

// 页面导航
const navigateTo = (path) => {
  router.push(path);
};

// 登录注册跳转
const goToLogin = () => {
  router.push('/login');
};

const goToRegister = () => {
  router.push('/register');
};

// 退出登录
const handleLogout = () => {
  ElMessageBox.confirm('确定要退出登录吗?', '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(async () => {
    try {
      // 实际项目中使用API
      // await api.logout();
      
      // 清除本地存储
      localStorage.removeItem('token');
      localStorage.removeItem('username');
      localStorage.removeItem('userRole');
      
      isLoggedIn.value = false;
      userInfo.value = {};
      isAdmin.value = false;
      isElder.value = false;
      
      ElMessage.success('退出登录成功');
    } catch (error) {
      console.error('退出登录失败:', error);
      ElMessage.error('退出登录失败');
    }
  }).catch(() => {});
};

// 主题设置
const loadThemeSettings = () => {
  const savedTheme = localStorage.getItem('theme');
  if (savedTheme) {
    currentTheme.value = savedTheme;
  }
};

const setTheme = (theme) => {
  currentTheme.value = theme;
  localStorage.setItem('theme', theme);
  
  // 更新文档类名以应用主题
  document.documentElement.className = '';
  if (theme === 'dark') {
    document.documentElement.classList.add('dark-theme');
  } else if (theme === 'light') {
    document.documentElement.classList.add('light-theme');
  } else {
    // 跟随系统
    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
      document.documentElement.classList.add('dark-theme');
    } else {
      document.documentElement.classList.add('light-theme');
    }
  }
  
  showThemeSelector.value = false;
  ElMessage.success('主题设置已更新');
};
</script>

<style scoped>
.mobile-user-center {
  padding-bottom: 20px;
}

.user-info-card {
  background-color: var(--primary-color, #409eff);
  color: #ffffff;
  padding: 20px;
  margin: -16px -16px 16px;
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
}

.user-avatar {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  overflow: hidden;
  margin-bottom: 15px;
  background-color: #fff;
  border: 3px solid rgba(255, 255, 255, 0.5);
}

.user-avatar img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.user-details {
  text-align: center;
}

.username {
  font-size: 18px;
  font-weight: 600;
  margin: 0 0 5px;
}

.user-role {
  font-size: 14px;
  opacity: 0.8;
  margin: 0;
}

.login-actions {
  margin-top: 15px;
  display: flex;
  gap: 10px;
}

.menu-section {
  margin-bottom: 20px;
  background-color: var(--card-bg-color, #fff);
  border-radius: 10px;
  overflow: hidden;
}

.section-title {
  padding: 12px 16px;
  font-size: 15px;
  font-weight: 500;
  color: var(--text-secondary-color, #909399);
  background-color: var(--bg-color, #f5f5f7);
  border-bottom: 1px solid var(--border-color, #ebedf0);
}

.menu-list {
  background-color: var(--card-bg-color, #fff);
}

.menu-item {
  display: flex;
  align-items: center;
  padding: 16px;
  border-bottom: 1px solid var(--border-color, #ebedf0);
  cursor: pointer;
}

.menu-item:last-child {
  border-bottom: none;
}

.menu-item .el-icon {
  font-size: 20px;
  margin-right: 12px;
  color: var(--primary-color, #409eff);
}

.item-label {
  flex: 1;
  font-size: 15px;
  color: var(--text-primary-color, #303133);
}

.arrow-icon {
  color: var(--text-secondary-color, #909399);
  margin-right: 0;
}

.logout-section {
  margin: 30px 0;
  padding: 0 20px;
}

.logout-button {
  width: 100%;
}

.theme-options {
  display: flex;
  justify-content: space-around;
  padding: 10px 0;
}

.theme-option {
  display: flex;
  flex-direction: column;
  align-items: center;
  cursor: pointer;
  padding: 10px;
  border-radius: 8px;
}

.theme-option.active {
  background-color: rgba(64, 158, 255, 0.1);
}

.theme-icon {
  width: 60px;
  height: 60px;
  border-radius: 50%;
  margin-bottom: 10px;
  border: 1px solid var(--border-color, #ebedf0);
}

.light-theme {
  background-color: #ffffff;
}

.dark-theme {
  background-color: #1a1a1a;
}

.auto-theme {
  background: linear-gradient(to right, #ffffff 0%, #ffffff 50%, #1a1a1a 50%, #1a1a1a 100%);
}

.theme-label {
  font-size: 14px;
}

@media (prefers-color-scheme: dark) {
  .menu-section {
    background-color: var(--card-bg-color, #1a1a1a);
  }
  
  .menu-item,
  .section-title {
    border-color: var(--border-color, #3a3a3a);
  }
  
  .section-title {
    background-color: var(--bg-color, #121212);
  }
}
</style> 
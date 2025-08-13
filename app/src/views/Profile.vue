<template>
  <div class="profile-page">
    <!-- 未登录状态 -->
    <template v-if="!isLoggedIn">
      <div class="login-panel">
        <img :src="defaultAvatar" alt="头像" class="avatar" />
        <h2 class="login-title">登录 / 注册</h2>
        <p class="login-desc">登录后体验更多功能</p>
        <div class="login-actions">
          <van-button type="primary" block round @click="goToLogin">
            立即登录
          </van-button>
          <van-button plain block round @click="goToRegister" class="mt-3">
            注册账号
          </van-button>
        </div>
      </div>
    </template>
    
    <!-- 已登录状态 -->
    <template v-else>
      <!-- 用户信息卡片 -->
      <div class="user-card">
        <div class="user-info">
          <div class="user-avatar">
            <img :src="userAvatar || defaultAvatar" alt="头像" />
          </div>
          <div class="user-meta">
            <div class="user-name">{{ userNickname }}</div>
            <div class="user-role">{{ userRoleText }}</div>
          </div>
        </div>
        <div class="user-stats">
          <div class="stat-item clickable" @click="router.push('/my-resources')">
            <div class="stat-value">{{ resourceCount }}</div>
            <div class="stat-label">资源</div>
          </div>
          <div class="stat-item clickable" @click="router.push('/my-posts')">
            <div class="stat-value">{{ postCount }}</div>
            <div class="stat-label">帖子</div>
          </div>
          <div class="stat-item clickable" @click="router.push('/favorites')">
            <div class="stat-value">{{ likeCount }}</div>
            <div class="stat-label">点赞</div>
          </div>
        </div>
      </div>
      
      <!-- 功能菜单 -->
      <div class="menu-group">
        <div class="menu-title">我的内容</div>
        <van-cell-group inset>
          <van-cell title="我的通知" icon="bell-o" is-link to="/notifications" />
          <van-cell title="我的资源" icon="orders-o" is-link to="/my-resources" />
          <van-cell title="我的帖子" icon="notes-o" is-link to="/my-posts" />
          <van-cell title="我的评论" icon="comment-o" is-link to="/my-comments" />
          <van-cell title="我的点赞" icon="star-o" is-link to="/favorites" />
        </van-cell-group>
      </div>
      
      <div class="menu-group">
        <div class="menu-title">账号管理</div>
        <van-cell-group inset>
          <van-cell title="个人资料" icon="contact" is-link @click="showEditProfilePopup = true" />
          <van-cell title="修改密码" icon="lock" is-link @click="showChangePasswordPopup = true" />
          <van-cell title="消息设置" icon="bell-o" is-link to="/message-settings" />
          <van-cell title="分类订阅" icon="bookmark-o" is-link to="/subscriptions" />
        </van-cell-group>
      </div>
      
      <div class="menu-group">
        <div class="menu-title">其他</div>
        <van-cell-group inset>
          <van-cell title="意见反馈" icon="smile-comment-o" is-link />
          <van-cell title="关于我们" icon="info-o" is-link to="/about" />
          <van-cell title="退出登录" icon="delete" @click="confirmLogout" />
        </van-cell-group>
      </div>
      
      <!-- 编辑资料弹窗 -->
      <van-popup
        v-model:show="showEditProfilePopup"
        position="bottom"
        round
        style="height: 70%"
      >
        <div class="popup-content">
          <div class="popup-header">
            <div class="popup-title">编辑个人资料</div>
            <van-icon name="cross" @click="showEditProfilePopup = false" />
          </div>
          
          <div class="form-content">
            <van-form @submit="updateProfile">
              <van-cell-group inset>
                <van-field
                  v-model="profileForm.nickname"
                  label="昵称"
                  placeholder="请输入昵称"
                  :rules="[{ required: true, message: '请填写昵称' }]"
                />
                <van-field
                  v-model="profileForm.email"
                  label="邮箱"
                  placeholder="请输入邮箱"
                  :rules="[
                    { required: true, message: '请填写邮箱' },
                    { pattern: /.+@.+\..+/, message: '请输入正确的邮箱格式' }
                  ]"
                />
                <van-field
                  v-model="profileForm.qq_number"
                  label="QQ"
                  placeholder="请输入QQ号"
                  type="tel"
                />
                <van-field
                  v-model="profileForm.avatar_url"
                  label="头像直链"
                  placeholder="请输入头像图片的直链 URL"
                />
              </van-cell-group>
              
              <div style="margin: 16px;">
                <van-button round block type="primary" native-type="submit" :loading="submitting">
                  保存
                </van-button>
              </div>
            </van-form>
          </div>
        </div>
      </van-popup>
      
      <!-- 修改密码弹窗 -->
      <van-popup
        v-model:show="showChangePasswordPopup"
        position="bottom"
        round
        style="height: 70%"
      >
        <div class="popup-content">
          <div class="popup-header">
            <div class="popup-title">修改密码</div>
            <van-icon name="cross" @click="showChangePasswordPopup = false" />
          </div>
          
          <div class="form-content">
            <van-form @submit="changePassword">
              <van-cell-group inset>
                <van-field
                  v-model="passwordForm.oldPassword"
                  type="password"
                  label="当前密码"
                  placeholder="请输入当前密码"
                  :rules="[{ required: true, message: '请填写当前密码' }]"
                />
                <van-field
                  v-model="passwordForm.newPassword"
                  type="password"
                  label="新密码"
                  placeholder="请输入新密码"
                  :rules="[
                    { required: true, message: '请填写新密码' },
                    { min: 6, message: '密码长度不能少于6位' }
                  ]"
                />
                <van-field
                  v-model="passwordForm.confirmPassword"
                  type="password"
                  label="确认新密码"
                  placeholder="请再次输入新密码"
                  :rules="[
                    { required: true, message: '请确认新密码' },
                    { validator: validateConfirmPassword, message: '两次输入的密码不一致' }
                  ]"
                />
              </van-cell-group>
              
              <div style="margin: 16px;">
                <van-button round block type="primary" native-type="submit" :loading="submitting">
                  确认修改
                </van-button>
              </div>
            </van-form>
          </div>
        </div>
      </van-popup>
    </template>
    
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { showToast, showDialog } from 'vant';
import { useUserStore } from '../store/user';
import defaultAvatar from '@/assets/default-avatar.svg';
import { get } from '../utils/request';
import { postApi } from '../api/post';

const router = useRouter();
const userStore = useUserStore();

// 登录状态
const isLoggedIn = computed(() => userStore.isLoggedIn);

// 用户信息
const userInfo = computed(() => userStore.userInfo);
const userNickname = computed(() => userStore.nickname);
const userAvatar = computed(() => userStore.userAvatar);
const userRoleText = computed(() => {
  const roleMap = {
    'admin': '管理员',
    'moderator': '版主',
    'elder': '元老',
    'user': '普通用户'
  };
  return roleMap[userStore.userRole] || '普通用户';
});

// 计数
const resourceCount = ref(0);
const postCount = ref(0);
const likeCount = computed(() => userStore.userInfo?.favorite_count || 0);

async function refreshCounters() {
  try {
    // 资源数：调用我的资源接口，读取 total
    const resRes = await get('/users/my-resources', { page: 1, pageSize: 1 });
    resourceCount.value = resRes?.data?.total ?? (userStore.userInfo?.upload_count || 0);
  } catch { resourceCount.value = userStore.userInfo?.upload_count || 0; }

  try {
    // 帖子数：作者维度 Draft+Published 总和
    const base = { page: 1, page_size: 1, author_id: userStore.userId };
    const results = await Promise.allSettled([
      postApi.getPosts({ ...base, status: 'Draft' }),
      postApi.getPosts({ ...base, status: 'Published' })
    ]);
    const draftTotal = results[0].status === 'fulfilled' ? (results[0].value?.data?.total || 0) : 0;
    const pubTotal = results[1].status === 'fulfilled' ? (results[1].value?.data?.total || 0) : 0;
    postCount.value = draftTotal + pubTotal;
  } catch { postCount.value = 0; }
}

// 表单相关
const showEditProfilePopup = ref(false);
const showChangePasswordPopup = ref(false);
const submitting = ref(false);

// 资料表单
const profileForm = ref({
  nickname: '',
  email: '',
  qq_number: '',
  avatar_url: ''
});

// 密码表单
const passwordForm = ref({
  oldPassword: '',
  newPassword: '',
  confirmPassword: ''
});

// 确认密码验证
const validateConfirmPassword = (value) => {
  return value === passwordForm.value.newPassword;
};

// 跳转到登录页
const goToLogin = () => {
  router.push('/login');
};

// 跳转到注册页
const goToRegister = () => {
  router.push('/register');
};

// 退出登录确认
const confirmLogout = () => {
  showDialog({
    title: '提示',
    message: '确定要退出登录吗？',
    showCancelButton: true
  }).then((action) => {
    if (action === 'confirm') {
      logout();
    }
  });
};

// 退出登录
const logout = async () => {
  await userStore.logout();
  showToast('已退出登录');
};

// 初始化表单数据
const initFormData = () => {
  if (isLoggedIn.value) {
    profileForm.value = {
      nickname: userStore.userInfo.nickname || '',
      email: userStore.userInfo.email || '',
      qq_number: userStore.userInfo.qq_number || '',
      avatar_url: userStore.userInfo.avatar_url || ''
    };
  }
};

// 更新个人资料
const updateProfile = async () => {
  if (!isLoggedIn.value) return;
  
  submitting.value = true;
  try {
    const success = await userStore.updateUserProfile(profileForm.value);
    if (success) {
      showToast('资料更新成功');
      showEditProfilePopup.value = false;
    }
  } catch (error) {
    console.error('更新资料失败', error);
    showToast('更新失败，请重试');
  } finally {
    submitting.value = false;
  }
};

// 修改密码
const changePassword = async () => {
  if (!isLoggedIn.value) return;
  
  if (passwordForm.value.newPassword !== passwordForm.value.confirmPassword) {
    showToast('两次输入的密码不一致');
    return;
  }
  
  submitting.value = true;
  try {
    const success = await userStore.changePassword(
      passwordForm.value.oldPassword,
      passwordForm.value.newPassword
    );
    if (success) {
      showToast('密码修改成功');
      showChangePasswordPopup.value = false;
      
      // 清空表单
      passwordForm.value = {
        oldPassword: '',
        newPassword: '',
        confirmPassword: ''
      };
    }
  } catch (error) {
    console.error('修改密码失败', error);
  } finally {
    submitting.value = false;
  }
};

// 页面加载时初始化
onMounted(async () => {
  if (isLoggedIn.value) {
    try { await userStore.checkAuth(); } catch (e) {}
    initFormData();
    await refreshCounters();
  }
});
</script>

<style scoped>
.profile-page {
  min-height: 100vh;
  background-color: var(--background-color);
}

.login-panel {
  background-color: #fff;
  padding: 40px 20px;
  text-align: center;
  margin-bottom: 20px;
}

.avatar {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  margin-bottom: 16px;
}

.login-title {
  font-size: 20px;
  font-weight: 500;
  margin-bottom: 8px;
  color: var(--text-color);
}

.login-desc {
  font-size: 14px;
  color: var(--text-color-light);
  margin-bottom: 24px;
}

.login-actions {
  padding: 0 20px;
}

.mt-3 {
  margin-top: 12px;
}

.user-card {
  background-color: var(--primary-color);
  color: white;
  padding: 20px 16px;
  margin-bottom: 16px;
  border-radius: 0 0 16px 16px;
}

.user-info {
  display: flex;
  align-items: center;
  margin-bottom: 20px;
}

.user-avatar {
  width: 64px;
  height: 64px;
  border-radius: 50%;
  overflow: hidden;
  margin-right: 16px;
  border: 2px solid rgba(255, 255, 255, 0.8);
}

.user-avatar img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.user-name {
  font-size: 18px;
  font-weight: 500;
  margin-bottom: 4px;
}

.user-role {
  font-size: 14px;
  opacity: 0.9;
}

.user-stats {
  display: flex;
  justify-content: space-around;
  text-align: center;
  padding: 12px 0;
  background-color: rgba(255, 255, 255, 0.1);
  border-radius: 8px;
}

.stat-item {
  flex: 1;
  padding: 0 8px;
}

.clickable { cursor: pointer; }

.stat-value {
  font-size: 18px;
  font-weight: 500;
  margin-bottom: 4px;
}

.stat-label {
  font-size: 12px;
  opacity: 0.9;
}

.menu-group {
  margin-bottom: 16px;
}

.menu-title {
  font-size: 14px;
  color: var(--text-color-light);
  padding: 0 16px 8px;
}

.popup-content {
  padding: 16px;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.popup-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 0 16px;
  margin-bottom: 16px;
  border-bottom: 1px solid var(--border-color);
}

.popup-title {
  font-size: 18px;
  font-weight: 500;
}

.form-content {
  flex: 1;
  overflow-y: auto;
}
</style> 
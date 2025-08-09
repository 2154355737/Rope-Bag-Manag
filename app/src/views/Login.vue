<template>
  <div class="login-page">
    <!-- 顶部导航栏 -->
    <van-nav-bar
      title="登录"
      left-arrow
      @click-left="onBack"
      fixed
    />
    
    <div class="login-content" :style="{ paddingTop: '46px' }">
      <!-- 登录表单 -->
      <div class="login-form">
        <div class="login-logo">
          <!-- 使用文字作为logo，移除不存在的图片引用 -->
          <h1 class="text-logo">资源社区</h1>
          <h2 class="app-name">专业资源分享平台</h2>
        </div>
        
        <van-tabs v-model:active="activeTab" animated swipeable>
          <!-- 账号密码登录 -->
          <van-tab title="账号密码登录">
            <van-form @submit="onPasswordLogin" class="form-content">
              <van-cell-group inset>
                <van-field
                  v-model="passwordForm.username"
                  name="username"
                  label="用户名"
                  placeholder="请输入用户名/邮箱"
                  :rules="[{ required: true, message: '请输入用户名/邮箱' }]"
                />
                <van-field
                  v-model="passwordForm.password"
                  type="password"
                  name="password"
                  label="密码"
                  placeholder="请输入密码"
                  :rules="[{ required: true, message: '请输入密码' }]"
                />
              </van-cell-group>
              
              <div class="form-actions">
                <div class="form-links">
                  <span @click="goToForgotPassword">忘记密码？</span>
                  <span @click="goToRegister">注册账号</span>
                </div>
                
                <div class="submit-btn">
                  <van-button 
                    round 
                    block 
                    type="primary" 
                    native-type="submit"
                    :loading="passwordLoading"
                  >
                    登录
                  </van-button>
                </div>
              </div>
            </van-form>
          </van-tab>
          
          <!-- 验证码登录 -->
          <van-tab title="邮箱验证码登录">
            <van-form @submit="onCodeLogin" class="form-content">
              <van-cell-group inset>
                <van-field
                  v-model="codeForm.email"
                  name="email"
                  label="邮箱"
                  placeholder="请输入邮箱"
                  :rules="[
                    { required: true, message: '请输入邮箱' },
                    { pattern: /.+@.+\..+/, message: '请输入有效的邮箱地址' }
                  ]"
                />
                
                <van-field
                  v-model="codeForm.code"
                  name="code"
                  label="验证码"
                  placeholder="请输入验证码"
                  :rules="[{ required: true, message: '请输入验证码' }]"
                >
                  <template #button>
                    <van-button 
                      size="small" 
                      type="primary" 
                      :disabled="sendingCode || countdown > 0"
                      @click="sendCode"
                    >
                      {{ countdown > 0 ? `${countdown}秒后重新获取` : '获取验证码' }}
                    </van-button>
                  </template>
                </van-field>
              </van-cell-group>
              
              <div class="form-actions">
                <div class="form-links">
                  <span></span>
                  <span @click="goToRegister">注册账号</span>
                </div>
                
                <div class="submit-btn">
                  <van-button 
                    round 
                    block 
                    type="primary" 
                    native-type="submit"
                    :loading="codeLoading"
                  >
                    登录
                  </van-button>
                </div>
              </div>
            </van-form>
          </van-tab>
        </van-tabs>
        
        <!-- 其他登录方式 -->
        <div class="other-login">
          <div class="divider-text">其他登录方式</div>
          
          <div class="other-login-icons">
            <van-icon name="wechat" class="other-icon" />
            <van-icon name="qq" class="other-icon" />
          </div>
        </div>
        
        <!-- 登录协议 -->
        <div class="agreement-text">
          登录即代表同意 <span class="agreement-link">《用户协议》</span>和<span class="agreement-link">《隐私政策》</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onBeforeUnmount } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { showToast, showNotify } from 'vant';
import { useUserStore } from '../store/user';

const router = useRouter();
const route = useRoute();
const userStore = useUserStore();

// 表单数据
const activeTab = ref(0);
const passwordForm = ref({
  username: '',
  password: '',
});
const codeForm = ref({
  email: '',
  code: '',
});

// 加载状态
const passwordLoading = ref(false);
const codeLoading = ref(false);
const sendingCode = ref(false);

// 验证码倒计时
const countdown = ref(0);
let countdownTimer = null;

// 返回上一页
const onBack = () => {
  router.back();
};

// 账号密码登录
const onPasswordLogin = async () => {
  passwordLoading.value = true;
  
  try {
    const success = await userStore.login(
      passwordForm.value.username,
      passwordForm.value.password
    );
    
    if (success) {
      // 跳转到目标页面或首页
      const redirectPath = route.query.redirect || '/';
      router.replace(redirectPath);
    }
  } catch (error) {
    console.error('登录失败', error);
  } finally {
    passwordLoading.value = false;
  }
};

// 验证码登录
const onCodeLogin = async () => {
  codeLoading.value = true;
  
  try {
    const success = await userStore.loginByEmail(
      codeForm.value.email,
      codeForm.value.code
    );
    
    if (success) {
      // 跳转到目标页面或首页
      const redirectPath = route.query.redirect || '/';
      router.replace(redirectPath);
    }
  } catch (error) {
    console.error('登录失败', error);
  } finally {
    codeLoading.value = false;
  }
};

// 发送验证码
const sendCode = async () => {
  if (sendingCode.value || countdown.value > 0) return;
  
  if (!codeForm.value.email || !/.+@.+\..+/.test(codeForm.value.email)) {
    showToast('请输入有效的邮箱地址');
    return;
  }
  
  sendingCode.value = true;
  try {
    const success = await userStore.sendLoginCode(codeForm.value.email);
    if (success) {
      showNotify({ type: 'success', message: '验证码已发送' });
      
      // 开始倒计时
      countdown.value = 60;
      countdownTimer = setInterval(() => {
        countdown.value -= 1;
        if (countdown.value <= 0) {
          clearInterval(countdownTimer);
        }
      }, 1000);
    }
  } catch (error) {
    console.error('发送验证码失败', error);
  } finally {
    sendingCode.value = false;
  }
};

// 跳转到忘记密码页面
const goToForgotPassword = () => {
  router.push('/forgot-password');
};

// 跳转到注册页面
const goToRegister = () => {
  router.push('/register');
};

// 组件卸载前清除定时器
onBeforeUnmount(() => {
  if (countdownTimer) {
    clearInterval(countdownTimer);
  }
});
</script>

<style scoped>
.login-page {
  min-height: 100vh;
  background-color: var(--background-color);
}

.login-content {
  padding: 16px;
  display: flex;
  flex-direction: column;
  min-height: calc(100vh - 46px);
}

.login-form {
  flex: 1;
}

.login-logo {
  text-align: center;
  margin: 20px 0 30px;
}

.text-logo {
  font-size: 32px;
  font-weight: bold;
  color: var(--primary-color);
  margin-bottom: 8px;
}

.app-name {
  font-size: 18px;
  font-weight: bold;
  color: var(--text-color);
}

.form-content {
  margin: 20px 0;
}

.form-actions {
  margin: 20px 16px;
}

.form-links {
  display: flex;
  justify-content: space-between;
  margin-bottom: 20px;
  font-size: 14px;
  color: var(--text-color-light);
}

.form-links span {
  cursor: pointer;
}

.submit-btn {
  margin-top: 20px;
}

.other-login {
  margin-top: 30px;
}

.divider-text {
  position: relative;
  text-align: center;
  font-size: 14px;
  color: var(--text-color-light);
  margin-bottom: 20px;
}

.divider-text::before,
.divider-text::after {
  content: '';
  position: absolute;
  top: 50%;
  width: 20%;
  height: 1px;
  background-color: var(--border-color);
}

.divider-text::before {
  left: 20%;
}

.divider-text::after {
  right: 20%;
}

.other-login-icons {
  display: flex;
  justify-content: center;
}

.other-icon {
  font-size: 28px;
  color: var(--text-color-light);
  margin: 0 20px;
  cursor: pointer;
}

.agreement-text {
  margin-top: 30px;
  text-align: center;
  font-size: 12px;
  color: var(--text-color-light);
}

.agreement-link {
  color: var(--primary-color);
}
</style> 
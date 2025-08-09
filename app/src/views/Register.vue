<template>
  <div class="register-page">
    <!-- 顶部导航栏 -->
    <van-nav-bar
      title="注册"
      left-arrow
      @click-left="onBack"
      fixed
    />
    
    <div class="register-content" :style="{ paddingTop: '46px' }">
      <!-- 注册表单 -->
      <div class="register-form">
        <div class="register-logo">
          <img :src="logoUrl" alt="资源社区" class="logo-img" />
          <h2 class="app-name">资源社区</h2>
        </div>
        
        <van-form @submit="onRegister" class="form-content">
          <van-cell-group inset>
            <van-field
              v-model="form.username"
              name="username"
              label="用户名"
              placeholder="请输入用户名"
              :rules="[
                { required: true, message: '请输入用户名' },
                { min: 3, max: 20, message: '用户名长度为3-20个字符' }
              ]"
            />
            
            <van-field
              v-model="form.email"
              name="email"
              label="邮箱"
              placeholder="请输入邮箱"
              :rules="[
                { required: true, message: '请输入邮箱' },
                { pattern: /.+@.+\..+/, message: '请输入有效的邮箱地址' }
              ]"
            />
            
            <van-field
              v-model="form.verification_code"
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
            
            <van-field
              v-model="form.password"
              type="password"
              name="password"
              label="密码"
              placeholder="请输入密码"
              :rules="[
                { required: true, message: '请输入密码' },
                { min: 6, message: '密码长度不能少于6位' }
              ]"
            />
            
            <van-field
              v-model="confirmPassword"
              type="password"
              name="confirmPassword"
              label="确认密码"
              placeholder="请确认密码"
              :rules="[
                { required: true, message: '请确认密码' },
                { validator: validateConfirmPassword, message: '两次输入的密码不一致' }
              ]"
            />
            
            <van-field
              v-model="form.nickname"
              name="nickname"
              label="昵称"
              placeholder="请输入昵称（选填）"
            />
            
            <van-field
              v-model="form.qq_number"
              name="qq"
              label="QQ号"
              placeholder="请输入QQ号（选填）"
              type="tel"
            />
          </van-cell-group>
          
          <div class="agreement-checkbox">
            <van-checkbox v-model="agreedToTerms">
              同意 <span class="agreement-link">《用户协议》</span>和<span class="agreement-link">《隐私政策》</span>
            </van-checkbox>
          </div>
          
          <div class="form-actions">
            <van-button 
              round 
              block 
              type="primary" 
              native-type="submit" 
              :disabled="!agreedToTerms"
              :loading="loading"
            >
              注册
            </van-button>
            
            <div class="login-link">
              已有账号？ <span @click="goToLogin">立即登录</span>
            </div>
          </div>
        </van-form>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onBeforeUnmount } from 'vue';
import { useRouter } from 'vue-router';
import { showToast, showNotify } from 'vant';
import { useUserStore } from '../store/user';
import logoUrl from '../assets/default-avatar.svg';

const router = useRouter();
const userStore = useUserStore();

// 表单数据
const form = ref({
  username: '',
  email: '',
  password: '',
  nickname: '',
  qq_number: '',
  verification_code: ''
});
const confirmPassword = ref('');
const agreedToTerms = ref(false);

// 加载状态
const loading = ref(false);
const sendingCode = ref(false);

// 验证码倒计时
const countdown = ref(0);
let countdownTimer = null;

// 返回上一页
const onBack = () => {
  router.back();
};

// 确认密码验证
const validateConfirmPassword = (value) => {
  return value === form.value.password;
};

// 发送验证码
const sendCode = async () => {
  if (sendingCode.value || countdown.value > 0) return;
  
  if (!form.value.email || !/.+@.+\..+/.test(form.value.email)) {
    showToast('请输入有效的邮箱地址');
    return;
  }
  
  sendingCode.value = true;
  try {
    const success = await userStore.sendRegisterCode(form.value.email);
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

// 注册
const onRegister = async () => {
  if (!agreedToTerms.value) {
    showToast('请同意用户协议和隐私政策');
    return;
  }
  
  loading.value = true;
  
  try {
    const success = await userStore.register(form.value);
    
    if (success) {
      showNotify({
        type: 'success',
        message: '注册成功，欢迎加入资源社区'
      });
      
      // 注册成功自动登录，跳转到首页
      router.replace('/');
    }
  } catch (error) {
    console.error('注册失败', error);
  } finally {
    loading.value = false;
  }
};

// 跳转到登录页
const goToLogin = () => {
  router.push('/login');
};

// 组件卸载前清除定时器
onBeforeUnmount(() => {
  if (countdownTimer) {
    clearInterval(countdownTimer);
  }
});
</script>

<style scoped>
.register-page {
  min-height: 100vh;
  background-color: var(--background-color);
}

.register-content {
  padding: 16px;
  min-height: calc(100vh - 46px);
}

.register-logo {
  text-align: center;
  margin: 10px 0 20px;
}

.logo-img {
  width: 60px;
  height: 60px;
  margin-bottom: 8px;
}

.app-name {
  font-size: 20px;
  font-weight: bold;
  color: var(--text-color);
}

.form-content {
  margin: 16px 0;
}

.agreement-checkbox {
  margin: 16px;
  font-size: 14px;
  color: var(--text-color-light);
}

.agreement-link {
  color: var(--primary-color);
}

.form-actions {
  margin: 20px 16px;
}

.login-link {
  margin-top: 16px;
  text-align: center;
  font-size: 14px;
  color: var(--text-color-light);
}

.login-link span {
  color: var(--primary-color);
  cursor: pointer;
}
</style> 
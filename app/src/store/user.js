import { defineStore } from 'pinia';
import { get, post, put } from '../utils/request';
import { showToast } from 'vant';
import router from '../router';

export const useUserStore = defineStore('user', {
  state: () => ({
    token: localStorage.getItem('token') || '',
    userInfo: JSON.parse(localStorage.getItem('userInfo') || '{}'),
    isLoading: false,
  }),
  
  getters: {
    isLoggedIn: (state) => !!state.token,
    username: (state) => state.userInfo.username || '',
    nickname: (state) => state.userInfo.nickname || state.userInfo.username || '',
    userId: (state) => state.userInfo.id,
    userRole: (state) => state.userInfo.role || '',
    userAvatar: (state) => state.userInfo.avatar_url || '',
  },
  
  actions: {
    // 用户登录
    async login(username, password) {
      try {
        this.isLoading = true;
        const res = await post('/auth/login', { username, password });
        
        // 存储登录信息
        const { user, token } = res.data;
        this.token = token;
        this.userInfo = user;
        
        // 保存到本地存储
        localStorage.setItem('token', token);
        localStorage.setItem('userInfo', JSON.stringify(user));
        
        showToast({
          message: '登录成功',
          type: 'success',
        });
        
        return true;
      } catch (error) {
        console.error('登录失败', error);
        return false;
      } finally {
        this.isLoading = false;
      }
    },
    
    // 用户注册
    async register(userData) {
      try {
        this.isLoading = true;
        const res = await post('/auth/register', userData);
        
        // 注册成功，自动登录
        const { user, token } = res.data;
        this.token = token;
        this.userInfo = user;
        
        // 保存到本地存储
        localStorage.setItem('token', token);
        localStorage.setItem('userInfo', JSON.stringify(user));
        
        showToast({
          message: '注册成功',
          type: 'success',
        });
        
        return true;
      } catch (error) {
        console.error('注册失败', error);
        return false;
      } finally {
        this.isLoading = false;
      }
    },
    
    // 发送注册验证码
    async sendRegisterCode(email) {
      try {
        await post('/auth/send-register-code', { email });
        showToast('验证码已发送');
        return true;
      } catch (error) {
        console.error('验证码发送失败', error);
        return false;
      }
    },
    
    // 验证邮箱验证码
    async verifyCode(email, code) {
      try {
        await post('/auth/verify-code', { email, code });
        return true;
      } catch (error) {
        console.error('验证码验证失败', error);
        return false;
      }
    },
    
    // 邮箱验证码登录
    async loginByEmail(email, code) {
      try {
        this.isLoading = true;
        const res = await post('/auth/login-by-email', { email, code });
        
        // 存储登录信息
        const { user, token } = res.data;
        this.token = token;
        this.userInfo = user;
        
        // 保存到本地存储
        localStorage.setItem('token', token);
        localStorage.setItem('userInfo', JSON.stringify(user));
        
        showToast({
          message: '登录成功',
          type: 'success',
        });
        
        return true;
      } catch (error) {
        console.error('登录失败', error);
        return false;
      } finally {
        this.isLoading = false;
      }
    },
    
    // 发送登录验证码
    async sendLoginCode(email) {
      try {
        await post('/auth/send-login-code', { email });
        showToast('验证码已发送');
        return true;
      } catch (error) {
        console.error('验证码发送失败', error);
        return false;
      }
    },
    
    // 退出登录
    async logout() {
      try {
        // 调用退出登录接口
        await post('/auth/logout');
      } catch (error) {
        console.error('退出登录失败', error);
      } finally {
        // 无论成功失败，都清除本地存储
        this.token = '';
        this.userInfo = {};
        localStorage.removeItem('token');
        localStorage.removeItem('userInfo');
        
        // 跳转到登录页
        router.push('/login');
      }
    },
    
    // 检查认证状态，获取最新用户信息
    async checkAuth() {
      if (!this.token) return false;
      
      try {
        const res = await get('/users/profile');
        // 更新用户信息
        this.userInfo = res.data;
        localStorage.setItem('userInfo', JSON.stringify(res.data));
        return true;
      } catch (error) {
        console.error('获取用户信息失败', error);
        // 如果认证失败，清除本地存储
        this.token = '';
        this.userInfo = {};
        localStorage.removeItem('token');
        localStorage.removeItem('userInfo');
        return false;
      }
    },
    
    // 更新用户资料
    async updateUserProfile(profileData) {
      try {
        this.isLoading = true;
        const res = await put('/users/profile', profileData);
        
        // 更新本地用户信息
        this.userInfo = { ...this.userInfo, ...res.data };
        localStorage.setItem('userInfo', JSON.stringify(this.userInfo));
        
        showToast({
          message: '资料更新成功',
          type: 'success',
        });
        
        return true;
      } catch (error) {
        console.error('更新资料失败', error);
        return false;
      } finally {
        this.isLoading = false;
      }
    },
    
    // 修改密码
    async changePassword(oldPassword, newPassword) {
      try {
        this.isLoading = true;
        await post('/users/change-password', {
          old_password: oldPassword,
          new_password: newPassword,
        });
        
        showToast({
          message: '密码修改成功',
          type: 'success',
        });
        
        return true;
      } catch (error) {
        console.error('修改密码失败', error);
        return false;
      } finally {
        this.isLoading = false;
      }
    },
  }
}); 
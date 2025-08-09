import axios from 'axios';
import { showToast, showDialog } from 'vant';
import router from '../router';

// 创建axios实例
const service = axios.create({
  baseURL: '/api',
  timeout: 15000
});

// 请求拦截器
service.interceptors.request.use(
  config => {
    const token = localStorage.getItem('token');
    if (token) {
      config.headers['Authorization'] = `Bearer ${token}`;
    }
    return config;
  },
  error => {
    console.error('请求错误', error);
    return Promise.reject(error);
  }
);

// 响应拦截器
service.interceptors.response.use(
  response => {
    const res = response.data;

    // 统一处理返回数据格式
    if (res.code !== 0) {
      // 显示错误提示
      showToast({
        message: res.message || '系统错误',
        type: 'fail',
        duration: 2000
      });

      // 处理特定错误码
      if (res.code === 401) {
        // token过期或无效
        showDialog({
          title: '登录提示',
          message: '登录状态已失效，请重新登录',
          confirmButtonText: '确定'
        }).then(() => {
          localStorage.removeItem('token');
          localStorage.removeItem('userInfo');
          router.push('/login');
        });
      }
      
      return Promise.reject(new Error(res.message || '系统错误'));
    } else {
      return res;
    }
  },
  error => {
    console.error('响应错误', error);
    
    // 网络错误处理
    let message = '';
    
    if (error.response) {
      // 服务器响应错误
      switch (error.response.status) {
        case 401:
          message = '未授权，请登录';
          // 清除token并跳转到登录页
          localStorage.removeItem('token');
          localStorage.removeItem('userInfo');
          router.push('/login');
          break;
        case 403:
          message = '拒绝访问';
          break;
        case 404:
          message = '请求的资源不存在';
          break;
        case 500:
          message = '服务器错误';
          break;
        default:
          message = `请求错误 (${error.response.status})`;
      }
    } else if (error.request) {
      // 请求发出但没收到响应
      message = '服务器无响应';
    } else {
      // 请求配置出错
      message = '请求配置错误';
    }
    
    // 显示错误提示
    showToast({
      message: message,
      type: 'fail',
      duration: 2000
    });
    
    return Promise.reject(error);
  }
);

// 封装GET请求
export function get(url, params) {
  return service({
    url,
    method: 'get',
    params
  });
}

// 封装POST请求
export function post(url, data) {
  return service({
    url,
    method: 'post',
    data
  });
}

// 封装PUT请求
export function put(url, data) {
  return service({
    url,
    method: 'put',
    data
  });
}

// 封装DELETE请求
export function del(url, params) {
  return service({
    url,
    method: 'delete',
    params
  });
}

// 封装上传文件请求
export function upload(url, file) {
  const formData = new FormData();
  formData.append('file', file);
  
  return service({
    url,
    method: 'post',
    data: formData,
    headers: {
      'Content-Type': 'multipart/form-data'
    }
  });
}

export default service; 
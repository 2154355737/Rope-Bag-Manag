// 智能网络请求模块：自动选择axios或原生fetch
import { showToast, showDialog } from 'vant';
import router from '../router';

// 检测是否可以使用axios (在某些Tauri环境中可能有问题)
let useNativeFetch = false;
let axios = null;

try {
  // 尝试导入axios
  axios = require('axios');
} catch (error) {
  try {
    // 如果require失败，尝试import
    import('axios').then(module => {
      axios = module.default;
    }).catch(() => {
      console.warn('📦 无法加载axios，将使用原生fetch');
      useNativeFetch = true;
    });
  } catch (importError) {
    console.warn('📦 无法加载axios，将使用原生fetch');
    useNativeFetch = true;
  }
}

// 获取API基础URL
const getBaseURL = () => {
  const envBaseURL = import.meta.env.VITE_API_BASE_URL;
  const isTauri = window.__TAURI__ !== undefined;
  
  console.log('🔧 Request Config:', {
    envBaseURL,
    isTauri,
    mode: import.meta.env.MODE,
    useNativeFetch,
    userAgent: navigator.userAgent
  });
  
  // 在Tauri环境中使用完整URL
  if (isTauri) {
    return envBaseURL || 'http://39.105.113.219:15201/api/v1';
  }
  
  return envBaseURL || '/api';
};

const baseURL = getBaseURL();

// ========== 原生Fetch实现 ==========
async function nativeFetchRequest(url, options = {}) {
  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), 15000);
  
  try {
    const fullURL = url.startsWith('http') ? url : `${baseURL}${url}`;
    const token = localStorage.getItem('token');
    
    const defaultOptions = {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
        ...(token && { 'Authorization': `Bearer ${token}` })
      },
      signal: controller.signal
    };
    
    const finalOptions = {
      ...defaultOptions,
      ...options,
      headers: {
        ...defaultOptions.headers,
        ...options.headers
      }
    };
    
    console.log('🚀 发送原生请求:', {
      method: finalOptions.method,
      url: fullURL
    });
    
    const response = await fetch(fullURL, finalOptions);
    clearTimeout(timeoutId);
    
    console.log('📡 收到原生响应:', {
      status: response.status,
      statusText: response.statusText
    });
    
    const data = await response.json();
    
    if (data.code !== 0) {
      console.warn('⚠️ 业务错误:', data);
      
      showToast({
        message: data.message || '系统错误',
        type: 'fail',
        duration: 2000
      });
      
      if (data.code === 401) {
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
      
      throw new Error(data.message || '系统错误');
    }
    
    console.log('✅ 请求成功:', data);
    return data;
    
  } catch (error) {
    clearTimeout(timeoutId);
    console.error('❌ 原生请求错误:', error);
    
    let message = '';
    if (error.name === 'AbortError') {
      message = '请求超时，请检查网络连接';
    } else if (error instanceof TypeError && error.message.includes('fetch')) {
      message = '网络连接失败，请检查网络设置';
    } else {
      message = error.message || '网络请求失败';
    }
    
    showToast({
      message: message,
      type: 'fail',
      duration: 3000
    });
    
    throw error;
  }
}

// ========== 统一接口 ==========

// 封装GET请求
export function get(url, params) {
  // 在Tauri环境中优先使用原生fetch
  const isTauri = window.__TAURI__ !== undefined;
  
  if (isTauri || useNativeFetch || !axios) {
    let fullURL = url;
    if (params) {
      const searchParams = new URLSearchParams();
      Object.keys(params).forEach(key => {
        if (params[key] !== null && params[key] !== undefined) {
          searchParams.append(key, params[key]);
        }
      });
      fullURL += `?${searchParams.toString()}`;
    }
    return nativeFetchRequest(fullURL);
  } else {
    // 使用axios的后备实现（暂不实现，专注于原生fetch）
    return nativeFetchRequest(url + (params ? '?' + new URLSearchParams(params).toString() : ''));
  }
}

// 封装POST请求
export function post(url, data) {
  return nativeFetchRequest(url, {
    method: 'POST',
    body: data ? JSON.stringify(data) : undefined
  });
}

// 封装PUT请求
export function put(url, data) {
  return nativeFetchRequest(url, {
    method: 'PUT',
    body: data ? JSON.stringify(data) : undefined
  });
}

// 封装DELETE请求
export function del(url, params) {
  let fullURL = url;
  if (params) {
    const searchParams = new URLSearchParams(params);
    fullURL += `?${searchParams.toString()}`;
  }
  return nativeFetchRequest(fullURL, {
    method: 'DELETE'
  });
}

// 封装上传文件请求
export function upload(url, file) {
  const formData = new FormData();
  formData.append('file', file);
  
  return nativeFetchRequest(url, {
    method: 'POST',
    body: formData,
    headers: {} // 不设置Content-Type，让浏览器自动设置
  });
}

export default { get, post, put, del, upload }; 
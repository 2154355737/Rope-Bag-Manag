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

// 将查询参数序列化为查询字符串（数组使用 key[] 表示法）
function buildQueryString(params) {
  const searchParams = new URLSearchParams();
  Object.keys(params || {}).forEach(key => {
    const value = params[key];
    if (value === null || value === undefined) return;
    if (Array.isArray(value)) {
      value.forEach(v => searchParams.append(`${key}[]`, v));
    } else {
      searchParams.append(key, value);
    }
  });
  const qs = searchParams.toString();
  return qs ? `?${qs}` : '';
}

// ========== 原生Fetch实现 ==========
async function nativeFetchRequest(url, options = {}) {
  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), 15000);
  
  try {
    const fullURL = url.startsWith('http') ? url : `${baseURL}${url}`;
    const token = localStorage.getItem('token');
    
    const defaultHeaders = {
      ...(token && { 'Authorization': `Bearer ${token}` })
    };
    
    // 如果不是 FormData，才设置 Content-Type
    if (!(options.body instanceof FormData)) {
      defaultHeaders['Content-Type'] = 'application/json';
    }
    
    const defaultOptions = {
      method: 'GET',
      headers: defaultHeaders,
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
    
    // 检查响应是否为空或者不是JSON格式
    const contentType = response.headers.get('content-type');
    let data;
    
    if (!response.ok) {
      // 对于非200状态码，尝试解析错误响应
      try {
        if (contentType && contentType.includes('application/json')) {
          data = await response.json();
        } else {
          const text = await response.text();
          data = { 
            code: response.status, 
            message: text || `HTTP ${response.status}: ${response.statusText}`,
            data: null 
          };
        }
      } catch (parseError) {
        data = { 
          code: response.status, 
          message: `HTTP ${response.status}: ${response.statusText}`,
          data: null 
        };
      }
      
      // 抛出错误以便被catch块处理
      const error = new Error(data.message || `HTTP ${response.status}`);
      error.response = { status: response.status, data };
      throw error;
    }
    
    // 成功响应的处理
    try {
      if (contentType && contentType.includes('application/json')) {
        data = await response.json();
      } else {
        const text = await response.text();
        data = { code: 0, data: text, message: 'success' };
      }
    } catch (parseError) {
      console.warn('⚠️ JSON解析失败，返回原始文本');
      const text = await response.text();
      data = { code: 0, data: text, message: 'success' };
    }
    
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
  
  const queryString = buildQueryString(params);

  if (isTauri || useNativeFetch || !axios) {
    let fullURL = url;
    if (params) {
      fullURL += queryString;
    }
    return nativeFetchRequest(fullURL);
  } else {
    // 使用axios的后备实现保持与原生序列化一致
    return nativeFetchRequest(url + (params ? queryString : ''));
  }
}

// 封装POST请求
export function post(url, data) {
  const options = {
    method: 'POST'
  };
  
  if (data) {
    if (data instanceof FormData) {
      options.body = data;
    } else {
      options.body = JSON.stringify(data);
    }
  }
  
  return nativeFetchRequest(url, options);
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
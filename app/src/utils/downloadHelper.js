/**
 * Android下载助手
 * 提供多种下载方案以确保在不同Android版本和环境下都能正常工作
 */

// 方案1: 使用Android Intent下载
export const downloadWithIntent = (url, filename) => {
  try {
    // 创建一个特殊的链接，触发Android的Intent系统
    const intentUrl = `intent:${url}#Intent;action=android.intent.action.VIEW;category=android.intent.category.BROWSABLE;end`;
    window.location.href = intentUrl;
    return true;
  } catch (error) {
    console.error('Intent下载失败:', error);
    return false;
  }
};

// 方案2: 使用iframe下载
export const downloadWithIframe = (url, filename) => {
  try {
    const iframe = document.createElement('iframe');
    iframe.style.display = 'none';
    iframe.src = url;
    document.body.appendChild(iframe);
    
    // 5秒后移除iframe
    setTimeout(() => {
      document.body.removeChild(iframe);
    }, 5000);
    
    return true;
  } catch (error) {
    console.error('Iframe下载失败:', error);
    return false;
  }
};

// 方案3: 使用fetch + blob下载（改进版）
export const downloadWithBlob = async (url, filename) => {
  try {
    console.log('开始Blob下载:', url, filename);
    
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    
    const blob = await response.blob();
    console.log('Blob创建成功，大小:', blob.size);
    
    // 检查blob是否有效
    if (blob.size === 0) {
      throw new Error('下载的文件为空');
    }
    
    const blobUrl = URL.createObjectURL(blob);
    console.log('Blob URL创建:', blobUrl);
    
    // 创建下载链接
    const link = document.createElement('a');
    link.href = blobUrl;
    link.download = filename || 'download';
    link.style.display = 'none';
    
    // 添加额外属性以提高兼容性
    link.target = '_blank';
    link.rel = 'noopener noreferrer';
    
    document.body.appendChild(link);
    
    // 触发下载
    const clickEvent = new MouseEvent('click', {
      bubbles: true,
      cancelable: true,
      view: window
    });
    
    link.dispatchEvent(clickEvent);
    
    // 延迟移除元素
    setTimeout(() => {
      document.body.removeChild(link);
      URL.revokeObjectURL(blobUrl);
    }, 2000);
    
    console.log('Blob下载触发成功');
    
    // 在Android WebView中，即使触发成功也可能没有实际下载
    // 所以我们返回false让系统尝试其他方案
    if (detectEnvironment().isWebView && detectEnvironment().isAndroid) {
      console.log('Android WebView环境，Blob下载可能无效，返回false');
      return false;
    }
    
    return true;
  } catch (error) {
    console.error('Blob下载失败:', error);
    return false;
  }
};

// 方案3.1: 调用Tauri命令（用于Android提示原生下载）
export const downloadWithTauriInvoke = async (url, filename) => {
  try {
    if (window.__TAURI__ && window.__TAURI__.invoke) {
      await window.__TAURI__.invoke('trigger_system_download', { url, filename });
      return true;
    }
    return false;
  } catch (error) {
    console.error('Tauri invoke 触发下载失败:', error);
    return false;
  }
};

// 方案4: 使用window.open下载（改进版）
export const downloadWithWindowOpen = (url, filename) => {
  try {
    console.log('使用window.open下载:', url);
    
    // 在Android WebView中，window.open可能会触发下载
    const newWindow = window.open(url, '_blank', 'noopener,noreferrer');
    
    if (!newWindow) {
      console.log('window.open被阻止，尝试其他方式');
      // 如果window.open被阻止，尝试直接赋值
      window.location.assign(url);
      return true;
    }
    
    // 检查窗口是否立即关闭（可能表示下载开始）
    setTimeout(() => {
      try {
        if (newWindow.closed) {
          console.log('新窗口已关闭，下载可能已开始');
        } else {
          // 如果窗口仍然打开，尝试关闭它
          newWindow.close();
        }
      } catch (e) {
        // 忽略跨域错误
        console.log('无法检查窗口状态（跨域限制）');
      }
    }, 2000);
    
    return true;
  } catch (error) {
    console.error('Window.open下载失败:', error);
    return false;
  }
};

// 方案5: 使用location.href下载（改进版）
export const downloadWithLocationHref = (url, filename) => {
  try {
    console.log('使用location.href下载:', url);
    
    // 在Android WebView中，直接跳转到下载URL通常是最可靠的方法
    // 这会触发WebView的下载监听器或系统下载管理器
    
    // 保存当前页面URL，以便可能需要返回
    const currentUrl = window.location.href;
    
    // 直接跳转到下载URL
    window.location.href = url;
    
    // 设置一个定时器，如果下载开始，用户可能会留在当前页面
    // 如果没有下载，页面可能会跳转，这是正常的
    setTimeout(() => {
      // 如果页面没有跳转，说明下载可能已经开始
      if (window.location.href === currentUrl) {
        console.log('页面未跳转，下载可能已开始');
      }
    }, 1000);
    
    return true;
  } catch (error) {
    console.error('Location.href下载失败:', error);
    return false;
  }
};

// 方案6: 使用Android WebView的下载监听
export const downloadWithWebViewListener = (url, filename) => {
  try {
    // 触发WebView的下载监听器
    const link = document.createElement('a');
    link.href = url;
    link.download = filename || '';
    link.rel = 'noopener';
    link.target = '_blank';
    
    // 添加特殊属性让WebView识别这是一个下载请求
    link.setAttribute('data-download', 'true');
    link.setAttribute('data-filename', filename || '');
    
    document.body.appendChild(link);
    
    // 触发点击事件
    const event = new MouseEvent('click', {
      bubbles: true,
      cancelable: true,
      view: window
    });
    
    link.dispatchEvent(event);
    document.body.removeChild(link);
    
    return true;
  } catch (error) {
    console.error('WebView监听器下载失败:', error);
    return false;
  }
};

// 检测Android版本和WebView环境
export const detectEnvironment = () => {
  const userAgent = navigator.userAgent;
  const isAndroid = /Android/i.test(userAgent);
  const isWebView = /wv\)/i.test(userAgent) || typeof window.__TAURI__ !== 'undefined';
  
  // 提取Android版本
  const androidMatch = userAgent.match(/Android (\d+(?:\.\d+)?)/);
  const androidVersion = androidMatch ? parseInt(androidMatch[1]) : 0;
  
  return {
    isAndroid,
    isWebView,
    androidVersion,
    userAgent
  };
};

// 智能下载函数 - 根据环境选择最佳下载方案
export const smartDownload = async (url, filename) => {
  const env = detectEnvironment();
  console.log('检测到的环境:', env);
  
  const methods = [];
  const methodNames = [];
  
  if (env.isAndroid) {
    if (env.isWebView) {
      // Android WebView 专用方案（优先级调整）
      methods.push(
        () => downloadWithTauriInvoke(url, filename),
        () => downloadWithLocationHref(url, filename),
        () => downloadWithWindowOpen(url, filename),
        () => downloadWithWebViewListener(url, filename),
        () => downloadWithIframe(url, filename),
        () => downloadWithIntent(url, filename)
      );
      methodNames.push('Tauri调用', 'Location跳转', 'Window打开', 'WebView监听', 'Iframe下载', 'Intent调用');
    } else if (env.androidVersion >= 10) {
      // Android 10+ 浏览器环境
      methods.push(
        () => downloadWithBlob(url, filename),
        () => downloadWithWebViewListener(url, filename),
        () => downloadWithWindowOpen(url, filename),
        () => downloadWithLocationHref(url, filename)
      );
      methodNames.push('Blob下载', 'WebView监听', 'Window打开', 'Location跳转');
    } else {
      // Android 9及以下版本
      methods.push(
        () => downloadWithWebViewListener(url, filename),
        () => downloadWithWindowOpen(url, filename),
        () => downloadWithLocationHref(url, filename),
        () => downloadWithBlob(url, filename)
      );
      methodNames.push('WebView监听', 'Window打开', 'Location跳转', 'Blob下载');
    }
  } else {
    // 非Android环境
    methods.push(
      () => downloadWithBlob(url, filename),
      () => downloadWithWindowOpen(url, filename),
      () => downloadWithLocationHref(url, filename)
    );
    methodNames.push('Blob下载', 'Window打开', 'Location跳转');
  }
  
  // 依次尝试各种下载方案
  for (let i = 0; i < methods.length; i++) {
    try {
      console.log(`尝试下载方案 ${i + 1}/${methods.length}: ${methodNames[i]}`);
      const success = await methods[i]();
      if (success) {
        console.log(`下载方案 ${i + 1} (${methodNames[i]}) 触发成功`);
        
        // 对于Android WebView，给用户额外的提示
        if (env.isAndroid && env.isWebView) {
          return { 
            success: true, 
            method: i + 1, 
            methodName: methodNames[i],
            message: '下载已触发，请检查通知栏或下载文件夹'
          };
        }
        
        return { 
          success: true, 
          method: i + 1, 
          methodName: methodNames[i] 
        };
      }
    } catch (error) {
      console.warn(`下载方案 ${i + 1} (${methodNames[i]}) 失败:`, error);
    }
    
    // 在方案之间添加短暂延迟
    if (i < methods.length - 1) {
      await new Promise(resolve => setTimeout(resolve, 500));
    }
  }
  
  return { success: false, error: '所有下载方案都失败了' };
};

// 简单的备用下载方案
export const fallbackDownload = (url, filename) => {
  console.log('使用备用下载方案');
  
  try {
    // 方案1: 直接跳转到URL
    window.location.href = url;
    return { success: true, method: 'fallback-redirect' };
  } catch (error) {
    console.error('备用下载方案失败:', error);
    
    try {
      // 方案2: 使用简单的a标签
      const link = document.createElement('a');
      link.href = url;
      link.download = filename || '';
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      return { success: true, method: 'fallback-link' };
    } catch (error2) {
      console.error('所有备用方案都失败:', error2);
      return { success: false, error: '所有下载方案都失败了' };
    }
  }
}; 
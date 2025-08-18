/**
 * 安全区域辅助工具
 * 用于处理安卓和iOS设备的安全区域，确保UI元素不被状态栏和导航栏遮挡
 */

// 存储安全区域尺寸
let safeAreaInsets = {
  top: 0,
  right: 0,
  bottom: 0,
  left: 0
};

// 检测设备类型
const isAndroid = /android/i.test(navigator.userAgent);
const isIOS = /iphone|ipad|ipod/i.test(navigator.userAgent);
const isChrome = /chrome/i.test(navigator.userAgent);
const isSafari = /safari/i.test(navigator.userAgent) && !isChrome;

// 存储全局状态
const state = {
  hasNavigationBar: false,
  hasCheckedNavBar: false,
  isUsingGestureNavigation: false
};

/**
 * 初始化安全区域监听
 */
export function initSafeArea() {
  // 设置设备标识类
  const root = document.documentElement;
  if (isAndroid) {
    root.classList.add('android-device');
    root.classList.remove('ios-device');
  } else if (isIOS) {
    root.classList.add('ios-device');
    root.classList.remove('android-device');
  }

  // 初始化时获取一次安全区域
  updateSafeAreaInsets();
  
  // 监听安全区域变化事件（由原生代码触发）
  window.addEventListener('safe-area-change', handleSafeAreaChange);
  
  // 监听窗口大小变化和方向变化
  window.addEventListener('resize', updateSafeAreaInsets);
  window.addEventListener('orientationchange', () => {
    // 方向变化后延迟更新，等待UI稳定
    setTimeout(updateSafeAreaInsets, 300);
  });
  
  // 初始化完成后应用CSS变量
  applySafeAreaToCSS();

  // 专门处理Android设备的系统导航栏
  if (isAndroid) {
    detectAndroidNavigationBar();
  }
}

/**
 * 更新安全区域尺寸
 */
function updateSafeAreaInsets() {
  // 尝试从CSS环境变量获取安全区域
  if (CSS && CSS.supports && CSS.supports('padding-top: env(safe-area-inset-top)')) {
    // 创建测试元素
    const testEl = document.createElement('div');
    testEl.style.cssText = `
      position: fixed;
      top: 0;
      right: 0;
      bottom: 0;
      left: 0;
      padding-top: env(safe-area-inset-top, 0px);
      padding-right: env(safe-area-inset-right, 0px);
      padding-bottom: env(safe-area-inset-bottom, 0px);
      padding-left: env(safe-area-inset-left, 0px);
      visibility: hidden;
    `;
    document.body.appendChild(testEl);
    
    // 获取计算后的样式
    const computedStyle = window.getComputedStyle(testEl);
    safeAreaInsets = {
      top: parseInt(computedStyle.paddingTop) || 0,
      right: parseInt(computedStyle.paddingRight) || 0,
      bottom: parseInt(computedStyle.paddingBottom) || 0,
      left: parseInt(computedStyle.paddingLeft) || 0
    };
    
    document.body.removeChild(testEl);
  } else {
    // 回退值：如果不支持env()，使用估计值
    safeAreaInsets = {
      top: isIOS ? 44 : 24,
      right: 0,
      // 针对Android设备，根据检测结果确定是否需要底部间距
      bottom: isIOS ? 34 : (isAndroid && state.hasNavigationBar ? 30 : 0),
      left: 0
    };
  }
  
  // 安卓设备底部安全区特殊处理
  if (isAndroid) {
    // 如果检测到使用全面屏手势，则不添加额外间距
    if (state.isUsingGestureNavigation) {
      safeAreaInsets.bottom = 0;
      document.documentElement.classList.remove('has-android-navbar');
    }
    // 如果检测到的底部安全区域小于10px且已确认有导航栏，则使用默认值
    else if (safeAreaInsets.bottom < 10 && state.hasNavigationBar) {
      safeAreaInsets.bottom = 30;
      document.documentElement.classList.add('has-android-navbar');
    }
    // 如果安全区域足够大，确认有导航栏
    else if (safeAreaInsets.bottom >= 10) {
      state.hasNavigationBar = true;
      state.hasCheckedNavBar = true;
      document.documentElement.classList.add('has-android-navbar');
    }
    // 如果没有导航栏，确保不添加多余间距
    else if (!state.hasNavigationBar && state.hasCheckedNavBar) {
      safeAreaInsets.bottom = 0;
      document.documentElement.classList.remove('has-android-navbar');
    }
  }
  
  // 应用到CSS变量
  applySafeAreaToCSS();
  
  // 触发自定义事件
  window.dispatchEvent(new CustomEvent('safe-area-updated', { detail: safeAreaInsets }));
}

/**
 * 处理原生代码触发的安全区域变化事件
 */
function handleSafeAreaChange(event) {
  if (event.detail) {
    // 保存原生传递的安全区域值
    const nativeValues = {
      top: event.detail.top || 0,
      right: event.detail.right || 0,
      bottom: event.detail.bottom || 0,
      left: event.detail.left || 0
    };
    
    // 更新全局状态
    state.hasNavigationBar = nativeValues.bottom > 0;
    state.hasCheckedNavBar = true;
    
    // 更新安全区域值
    safeAreaInsets = { ...nativeValues };
    
    // 应用到CSS变量
    applySafeAreaToCSS();
    
    // 触发自定义事件
    window.dispatchEvent(new CustomEvent('safe-area-updated', { detail: safeAreaInsets }));
    
    console.log('从原生接收到安全区域值:', nativeValues);
  }
}

/**
 * 将安全区域应用到CSS变量
 */
function applySafeAreaToCSS() {
  const root = document.documentElement;
  
  // 设置CSS变量
  root.style.setProperty('--safe-area-inset-top', `${safeAreaInsets.top}px`);
  root.style.setProperty('--safe-area-inset-right', `${safeAreaInsets.right}px`);
  root.style.setProperty('--safe-area-inset-bottom', `${safeAreaInsets.bottom}px`);
  root.style.setProperty('--safe-area-inset-left', `${safeAreaInsets.left}px`);
  
  // 添加设备类型标记
  if (isAndroid) {
    root.classList.add('android-device');
    root.classList.remove('ios-device');
  } else if (isIOS) {
    root.classList.add('ios-device');
    root.classList.remove('android-device');
  }
  
  // 检测是否有系统导航栏
  if (safeAreaInsets.bottom > 0) {
    root.classList.add('has-bottom-inset');
    
    // 如果底部安全区域高度超过阈值，设置为有导航栏
    if (safeAreaInsets.bottom >= 10 && isAndroid) {
      state.hasNavigationBar = true;
      state.hasCheckedNavBar = true;
    }
  } else {
    root.classList.remove('has-bottom-inset');
  }
  
  // 如果确认没有导航栏，移除相关类
  if (isAndroid && !state.hasNavigationBar && state.hasCheckedNavBar) {
    root.classList.remove('has-android-navbar');
    root.classList.remove('android-nav-bar');
  }
}

/**
 * 专门检测Android设备的系统导航栏
 */
function detectAndroidNavigationBar() {
  // 只有安卓设备才需要检测
  if (!isAndroid) return;
  
  // 获取屏幕信息，尝试判断是否使用全面屏手势导航
  try {
    // 检测是否支持safe-area-inset
    const supportsSafeArea = CSS.supports('padding-bottom', 'env(safe-area-inset-bottom)');
    
    // 比较屏幕高度和窗口高度
    const screenHeight = window.screen.height;
    const windowHeight = window.innerHeight;
    
    // 全面屏手势导航的特征：
    // 1. 窗口高度接近或等于屏幕高度
    // 2. 底部安全区域很小或为0
    
    // 计算窗口高度与屏幕高度的比例
    const heightRatio = windowHeight / screenHeight;
    
    // 测量safe-area-inset-bottom
    let safeAreaBottom = 0;
    
    if (supportsSafeArea) {
      const testEl = document.createElement('div');
      testEl.style.cssText = 'position:fixed;bottom:0;padding-bottom:env(safe-area-inset-bottom,0px);visibility:hidden;';
      document.body.appendChild(testEl);
      
      const computedStyle = window.getComputedStyle(testEl);
      safeAreaBottom = parseInt(computedStyle.paddingBottom) || 0;
      
      document.body.removeChild(testEl);
    }
    
    // 如果窗口高度接近屏幕高度（>95%）且底部安全区域很小（<10px），可能使用了全面屏手势导航
    state.isUsingGestureNavigation = heightRatio > 0.95 && safeAreaBottom < 10;
    
    // 根据检测结果设置状态
    if (state.isUsingGestureNavigation) {
      state.hasNavigationBar = false;
      state.hasCheckedNavBar = true;
      document.documentElement.classList.remove('has-android-navbar');
    } 
    // 如果有明显的底部安全区域，认为有导航栏
    else if (safeAreaBottom >= 10) {
      state.hasNavigationBar = true;
      state.hasCheckedNavBar = true;
      document.documentElement.classList.add('has-android-navbar');
      document.documentElement.style.setProperty('--android-navbar-height', `${safeAreaBottom}px`);
    } 
    // 如果窗口明显小于屏幕高度（<95%），可能有导航栏
    else if (heightRatio < 0.95) {
      state.hasNavigationBar = true;
      state.hasCheckedNavBar = true;
      document.documentElement.classList.add('has-android-navbar');
      document.documentElement.style.setProperty('--android-navbar-height', '30px');
    }
    // 其他情况，保持不确定状态，等待更多信息
    
    console.log('安卓导航栏检测结果:', {
      heightRatio,
      safeAreaBottom,
      isUsingGestureNavigation: state.isUsingGestureNavigation,
      hasNavigationBar: state.hasNavigationBar
    });
    
    // 动态添加样式，只有在确认有导航栏时才添加
    if (state.hasNavigationBar && !document.getElementById('android-nav-fix')) {
      const style = document.createElement('style');
      style.id = 'android-nav-fix';
      style.textContent = `
        /* 安卓导航栏适配样式 */
        .has-android-navbar .van-tabbar,
        .has-android-navbar .custom-tabbar {
          padding-bottom: calc(var(--android-navbar-height, 30px)) !important;
          min-height: calc(56px + var(--android-navbar-height, 30px)) !important;
        }
        
        .has-android-navbar .page-content {
          padding-bottom: calc(66px + var(--android-navbar-height, 30px)) !important;
        }
        
        .has-android-navbar .fixed-bottom {
          padding-bottom: calc(var(--android-navbar-height, 30px)) !important;
        }
      `;
      document.head.appendChild(style);
    }
  } catch (e) {
    console.error('检测安卓导航栏出错:', e);
  }
}

/**
 * 获取当前安全区域尺寸
 */
export function getSafeAreaInsets() {
  return { ...safeAreaInsets };
}

/**
 * 清理安全区域监听
 */
export function cleanupSafeArea() {
  window.removeEventListener('safe-area-change', handleSafeAreaChange);
  window.removeEventListener('resize', updateSafeAreaInsets);
  window.removeEventListener('orientationchange', updateSafeAreaInsets);
}

// 导出默认对象
export default {
  initSafeArea,
  getSafeAreaInsets,
  cleanupSafeArea
};
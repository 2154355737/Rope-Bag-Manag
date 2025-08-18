package com.jieshen.shequ

import android.os.Bundle
import android.webkit.WebView
import android.view.View
import android.view.WindowManager
import android.Manifest
import android.os.Build
import android.content.pm.PackageManager
import android.graphics.Color
import androidx.core.content.ContextCompat
import androidx.core.app.ActivityCompat
import android.view.KeyEvent
import android.view.KeyCharacterMap
import android.util.DisplayMetrics
import android.provider.Settings


class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)

    // 请求存储权限（Android 6.0-9.0）
    if (Build.VERSION.SDK_INT in 23..28) {
      val writeGranted = ContextCompat.checkSelfPermission(this, Manifest.permission.WRITE_EXTERNAL_STORAGE) == PackageManager.PERMISSION_GRANTED
      if (!writeGranted) {
        ActivityCompat.requestPermissions(
          this,
          arrayOf(Manifest.permission.WRITE_EXTERNAL_STORAGE, Manifest.permission.READ_EXTERNAL_STORAGE),
          1001
        )
      }
    }
    
    // 启用WebView调试
    WebView.setWebContentsDebuggingEnabled(true)
    
    // 设置非沉浸式状态
    setupNormalMode()
  }

  private fun setupNormalMode() {
    // 确保窗口不会进入全屏模式
    window.clearFlags(WindowManager.LayoutParams.FLAG_FULLSCREEN)
    
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
      // Android 11及以上使用WindowInsetsController
      // 清除沉浸模式标志，恢复正常显示
      @Suppress("DEPRECATION")
      window.decorView.systemUiVisibility = 0
      
      // 设置状态栏和导航栏为不透明
      window.statusBarColor = Color.WHITE
      window.navigationBarColor = Color.WHITE
    } else {
      // Android 10及以下使用旧API，不使用沉浸模式相关标志
      @Suppress("DEPRECATION")
      window.decorView.systemUiVisibility = (View.SYSTEM_UI_FLAG_LAYOUT_STABLE)
      
      // 设置状态栏和导航栏为不透明
      window.statusBarColor = Color.WHITE
      window.navigationBarColor = Color.WHITE
    }
  }

  override fun onWindowFocusChanged(hasFocus: Boolean) {
    super.onWindowFocusChanged(hasFocus)
    if (hasFocus) {
      // 当窗口获得焦点时重新应用非沉浸式模式
      setupNormalMode()
    }
  }
  
  override fun onWebViewCreate(webView: WebView) {
    super.onWebViewCreate(webView)
    // 挂载下载监听，使 a 链接 / location 跳转到可下载资源时由系统 DownloadManager 处理
    DownloadHelper.setupWebViewDownload(this@MainActivity, webView)
    
    // 向WebView注入安全区域信息
    injectSafeAreaToWebView(webView)
  }
  
  private fun injectSafeAreaToWebView(webView: WebView) {
    // 获取更精确的导航栏高度
    val navBarHeight = getNavigationBarHeight()
    val statusBarHeight = getStatusBarHeight()
    
    // 在页面加载完成后注入CSS变量
    webView.evaluateJavascript("""
      (function() {
        // 创建一个样式元素
        var style = document.createElement('style');
        style.textContent = `
          :root {
            --safe-area-inset-top: ${statusBarHeight}px;
            --safe-area-inset-bottom: ${navBarHeight}px;
            --safe-area-inset-left: 0px;
            --safe-area-inset-right: 0px;
            --android-navbar-height: ${navBarHeight}px;
          }
          
          /* 添加全局样式确保内容不被系统栏遮挡 */
          body {
            padding-top: var(--safe-area-inset-top);
            padding-bottom: var(--safe-area-inset-bottom);
          }
        `;
        document.head.appendChild(style);
        
        // 检测设备是否有导航栏
        if(${navBarHeight} > 0) {
          document.documentElement.classList.add('has-android-navbar');
          document.documentElement.classList.add('android-device');
        }
        
        // 通知前端应用安全区域已更新
        window.dispatchEvent(new CustomEvent('safe-area-change', {
          detail: {
            top: ${statusBarHeight},
            bottom: ${navBarHeight},
            left: 0,
            right: 0
          }
        }));
        
        console.log('安卓注入的安全区域值:', {
          top: ${statusBarHeight},
          bottom: ${navBarHeight}
        });
      })();
    """, null)
  }
  
  private fun getStatusBarHeight(): Int {
    val resourceId = resources.getIdentifier("status_bar_height", "dimen", "android")
    return if (resourceId > 0) resources.getDimensionPixelSize(resourceId) else 0
  }
  
  private fun getNavigationBarHeight(): Int {
    // 更精确地检测导航栏高度
    val resourceId = resources.getIdentifier("navigation_bar_height", "dimen", "android")
    val defaultNavHeight = if (resourceId > 0) resources.getDimensionPixelSize(resourceId) else 0
    
    // 首先检查设备是否使用全面屏手势导航 - 这种情况下不需要额外空间
    if (isGestureNavigationEnabled()) {
      return 0
    }
    
    // 判断是否有实际显示的虚拟导航栏
    val hasNavigationBar = try {
      // 通过反射检查系统配置
      val hasNavBarId = resources.getIdentifier("config_showNavigationBar", "bool", "android")
      if (hasNavBarId > 0 && resources.getBoolean(hasNavBarId)) {
        // 检查导航栏是否真正可见 (某些设备可以隐藏导航栏)
        val windowManager = getSystemService(WINDOW_SERVICE) as WindowManager
        val metrics = DisplayMetrics()
        windowManager.defaultDisplay.getRealMetrics(metrics)
        val realHeight = metrics.heightPixels
        
        windowManager.defaultDisplay.getMetrics(metrics)
        val displayHeight = metrics.heightPixels
        
        // 如果实际高度大于显示高度，差值通常是导航栏高度
        return if (realHeight - displayHeight > 0) {
          realHeight - displayHeight
        } else {
          0 // 可能使用手势导航或导航栏被隐藏
        }
      } else {
        // 检查是否有实体按键 (老设备)
        val hasBackKey = KeyCharacterMap.deviceHasKey(KeyEvent.KEYCODE_BACK)
        val hasHomeKey = KeyCharacterMap.deviceHasKey(KeyEvent.KEYCODE_HOME)
        if (!hasBackKey && !hasHomeKey) {
          // 没有实体按键，可能有虚拟导航栏
          defaultNavHeight
        } else {
          // 有实体按键，大概率没有虚拟导航栏
          0
        }
      }
    } catch (e: Exception) {
      // 默认返回0，不添加多余间距
      0
    }
  }

  // 检测是否使用了全面屏手势导航
  private fun isGestureNavigationEnabled(): Boolean {
    return try {
      if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
        // Android 10及以上可以通过系统设置检查
        val contentResolver = applicationContext.contentResolver
        val navigationMode = Settings.Secure.getInt(
          contentResolver,
          "navigation_mode",
          0
        )
        // 2 表示使用全面屏手势
        navigationMode == 2
      } else {
        false // Android 9及以下默认使用按钮导航
      }
    } catch (e: Exception) {
      false // 出错则默认为使用导航栏
    }
  }
}
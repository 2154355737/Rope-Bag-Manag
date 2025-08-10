package com.jieshen.shequ

import android.os.Bundle
import android.webkit.WebView
import android.view.View
import android.view.WindowManager
import androidx.core.view.ViewCompat
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat


class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)
    
    // 启用WebView调试
    WebView.setWebContentsDebuggingEnabled(true)
    
    // 确保窗口不会进入全屏模式
    window.clearFlags(WindowManager.LayoutParams.FLAG_FULLSCREEN)
    
    // 显示状态栏和导航栏
    window.decorView.systemUiVisibility = (
      View.SYSTEM_UI_FLAG_LAYOUT_STABLE or
      View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION or
      View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN
    )
    
    // 让系统管理窗口insets，确保内容不会被系统UI覆盖
    WindowCompat.setDecorFitsSystemWindows(window, false)
    
    // 为根视图设置窗口insets监听器
    val rootView = findViewById<View>(android.R.id.content)
    ViewCompat.setOnApplyWindowInsetsListener(rootView) { view, insets ->
      val systemBars = insets.getInsets(WindowInsetsCompat.Type.systemBars())
      view.setPadding(systemBars.left, systemBars.top, systemBars.right, systemBars.bottom)
      insets
    }
  }
}
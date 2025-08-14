package com.jieshen.shequ

import android.os.Bundle
import android.webkit.WebView
import android.view.View
import android.view.WindowManager
import android.Manifest
import android.os.Build
import android.content.pm.PackageManager


class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)

    // 请求存储权限（Android 6.0-9.0）
    if (Build.VERSION.SDK_INT in 23..28) {
      val writeGranted = checkSelfPermission(Manifest.permission.WRITE_EXTERNAL_STORAGE) == PackageManager.PERMISSION_GRANTED
      if (!writeGranted) {
        requestPermissions(
          arrayOf(Manifest.permission.WRITE_EXTERNAL_STORAGE, Manifest.permission.READ_EXTERNAL_STORAGE),
          1001
        )
      }
    }
    
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
  }

  override fun onWebViewCreate(webView: WebView) {
    super.onWebViewCreate(webView)
    // 挂载下载监听，使 a 链接 / location 跳转到可下载资源时由系统 DownloadManager 处理
    DownloadHelper.setupWebViewDownload(this, webView)
  }
}
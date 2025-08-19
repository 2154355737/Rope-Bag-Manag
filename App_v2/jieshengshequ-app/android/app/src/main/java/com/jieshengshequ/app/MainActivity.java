package com.jieshengshequ.app;

import android.os.Bundle;
import android.view.WindowManager;
import androidx.core.view.WindowCompat;
import com.getcapacitor.BridgeActivity;

public class MainActivity extends BridgeActivity {
  
  @Override
  public void onCreate(Bundle savedInstanceState) {
    // 注册自定义插件
    registerPlugin(NavigationBarPlugin.class);
    
    super.onCreate(savedInstanceState);
    
    // 启用边到边显示
    WindowCompat.setDecorFitsSystemWindows(getWindow(), false);
    
    // 设置状态栏和导航栏透明，但不使用NO_LIMITS避免默认预留空间
    getWindow().setStatusBarColor(android.graphics.Color.TRANSPARENT);
    getWindow().setNavigationBarColor(android.graphics.Color.TRANSPARENT);
    
    // 设置系统UI可见性，允许内容延伸到系统栏区域
    getWindow().getDecorView().setSystemUiVisibility(
        android.view.View.SYSTEM_UI_FLAG_LAYOUT_STABLE |
        android.view.View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION |
        android.view.View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN
    );
  }
}

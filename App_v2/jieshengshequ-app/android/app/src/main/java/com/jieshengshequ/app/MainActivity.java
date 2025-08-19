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
    
    // 初始设置状态栏和导航栏为白色，后续可通过插件动态修改
    getWindow().setStatusBarColor(android.graphics.Color.WHITE);
    getWindow().setNavigationBarColor(android.graphics.Color.WHITE);
    
    // 设置系统UI可见性，允许内容延伸到系统栏区域
    getWindow().getDecorView().setSystemUiVisibility(
        android.view.View.SYSTEM_UI_FLAG_LAYOUT_STABLE |
        android.view.View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION |
        android.view.View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN
    );
    
    // 支持状态栏和导航栏的动态控制
    getWindow().addFlags(WindowManager.LayoutParams.FLAG_DRAWS_SYSTEM_BAR_BACKGROUNDS);
    
    // 设置状态栏和导航栏按钮的样式（浅色内容，适合深色背景）
    if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.M) {
      // 可以根据需要动态调整
      getWindow().getDecorView().setSystemUiVisibility(
          getWindow().getDecorView().getSystemUiVisibility() |
          android.view.View.SYSTEM_UI_FLAG_LIGHT_STATUS_BAR
      );
    }
    
    if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.O) {
      // 设置导航栏按钮颜色
      getWindow().getDecorView().setSystemUiVisibility(
          getWindow().getDecorView().getSystemUiVisibility() |
          android.view.View.SYSTEM_UI_FLAG_LIGHT_NAVIGATION_BAR
      );
    }
  }
}

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
    
    // 设置状态栏和导航栏透明
    getWindow().setFlags(
        WindowManager.LayoutParams.FLAG_LAYOUT_NO_LIMITS,
        WindowManager.LayoutParams.FLAG_LAYOUT_NO_LIMITS
    );
  }
}

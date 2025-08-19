package com.jieshengshequ.app;

import android.os.Bundle;
import android.view.WindowManager;
import androidx.core.view.WindowCompat;
import com.getcapacitor.BridgeActivity;
import android.graphics.Color;
import android.os.Build;
import android.view.View;
import android.widget.FrameLayout;
import android.view.Gravity;
import androidx.core.view.ViewCompat;
import androidx.core.view.WindowInsetsCompat;
import android.view.WindowInsetsController;
import androidx.core.view.WindowInsetsControllerCompat;
import androidx.core.view.WindowInsetsAnimationCompat;
import java.util.List;

public class MainActivity extends BridgeActivity {
  
  // 顶部/底部系统栏着色层
  private View statusBarScrimView;
  private View navigationBarScrimView;
  
  @Override
  public void onCreate(Bundle savedInstanceState) {
    // 注册自定义插件
    registerPlugin(NavigationBarPlugin.class);
    
    super.onCreate(savedInstanceState);
    
    // 设置为标准模式，不启用边到边显示，让样式文件控制
    WindowCompat.setDecorFitsSystemWindows(getWindow(), true);
    
    // 确保绘制系统栏背景
    getWindow().addFlags(WindowManager.LayoutParams.FLAG_DRAWS_SYSTEM_BAR_BACKGROUNDS);

    // 运行时禁用系统对比度强制（部分 ROM 会在键盘/深浅切换时叠加遮罩）
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
      try {
        getWindow().setStatusBarContrastEnforced(false);
        getWindow().setNavigationBarContrastEnforced(false);
      } catch (Throwable ignored) {}
    }
    
    // 设置初始状态栏和导航栏颜色为白色
    getWindow().setStatusBarColor(android.graphics.Color.WHITE);
    getWindow().setNavigationBarColor(android.graphics.Color.WHITE);
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
      WindowInsetsController controller = getWindow().getInsetsController();
      if (controller != null) {
        controller.setSystemBarsAppearance(0, WindowInsetsController.APPEARANCE_LIGHT_STATUS_BARS | WindowInsetsController.APPEARANCE_LIGHT_NAVIGATION_BARS);
      }
    }
    
    // 设置状态栏文字/图标为深色（适合浅色背景）
    if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.M) {
      getWindow().getDecorView().setSystemUiVisibility(
          android.view.View.SYSTEM_UI_FLAG_LIGHT_STATUS_BAR
      );
    }
    
    // 设置导航栏按钮为深色（适合浅色背景）
    if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.O) {
      getWindow().getDecorView().setSystemUiVisibility(
          getWindow().getDecorView().getSystemUiVisibility() |
          android.view.View.SYSTEM_UI_FLAG_LIGHT_NAVIGATION_BAR
      );
    }

    // 创建并添加顶部/底部着色层（解决手势导航下系统栏透明感）
    try {
      FrameLayout content = findViewById(android.R.id.content);
      // 使用 decorView 作为 Insets 监听目标，避免 content 布局阶段延迟带来的残影
      View decor = getWindow().getDecorView();

      statusBarScrimView = new View(this);
      statusBarScrimView.setBackgroundColor(Color.WHITE);
      statusBarScrimView.setClickable(false);

      navigationBarScrimView = new View(this);
      navigationBarScrimView.setBackgroundColor(Color.WHITE);
      navigationBarScrimView.setClickable(false);

      FrameLayout.LayoutParams topLp = new FrameLayout.LayoutParams(
          FrameLayout.LayoutParams.MATCH_PARENT,
          0,
          Gravity.TOP
      );
      FrameLayout.LayoutParams bottomLp = new FrameLayout.LayoutParams(
          FrameLayout.LayoutParams.MATCH_PARENT,
          0,
          Gravity.BOTTOM
      );

      content.addView(statusBarScrimView, topLp);
      content.addView(navigationBarScrimView, bottomLp);

      // 获取可控的 Insets 控制器，并设置行为以便在键盘期隐藏导航栏
      final WindowInsetsControllerCompat controllerCompat = WindowCompat.getInsetsController(getWindow(), decor);
      if (controllerCompat != null) {
        controllerCompat.setSystemBarsBehavior(WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE);
      }

      ViewCompat.setOnApplyWindowInsetsListener(decor, (v, insets) -> {
        int top = insets.getInsets(WindowInsetsCompat.Type.statusBars()).top;
        int bottom = insets.getInsets(WindowInsetsCompat.Type.navigationBars()).bottom;
        boolean imeVisible = insets.isVisible(WindowInsetsCompat.Type.ime());
        boolean navVisible = insets.isVisible(WindowInsetsCompat.Type.navigationBars());

        // 顶部遮罩：跟随状态栏高度
        FrameLayout.LayoutParams tlp = (FrameLayout.LayoutParams) statusBarScrimView.getLayoutParams();
        tlp.height = top;
        statusBarScrimView.setLayoutParams(tlp);
        statusBarScrimView.setVisibility(top > 0 ? View.VISIBLE : View.GONE);

        // 底部遮罩：仅在导航栏可见且键盘不可见时显示
        FrameLayout.LayoutParams blp = (FrameLayout.LayoutParams) navigationBarScrimView.getLayoutParams();
        blp.height = navVisible ? bottom : 0;
        navigationBarScrimView.setLayoutParams(blp);
        boolean shouldShowBottomScrim = navVisible && !imeVisible;
        navigationBarScrimView.setVisibility(shouldShowBottomScrim ? View.VISIBLE : View.GONE);

        // 同步系统导航栏外观：键盘显示时设为透明并可选隐藏导航栏；隐藏时恢复
        if (imeVisible) {
          getWindow().setNavigationBarColor(Color.TRANSPARENT);
          if (controllerCompat != null && navVisible) {
            controllerCompat.hide(WindowInsetsCompat.Type.navigationBars());
          }
          if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            WindowInsetsController c = getWindow().getInsetsController();
            if (c != null) {
              c.setSystemBarsAppearance(0, WindowInsetsController.APPEARANCE_LIGHT_NAVIGATION_BARS);
            }
          } else if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            int vis = getWindow().getDecorView().getSystemUiVisibility();
            vis &= ~View.SYSTEM_UI_FLAG_LIGHT_NAVIGATION_BAR;
            getWindow().getDecorView().setSystemUiVisibility(vis);
          }
        } else {
          if (controllerCompat != null) {
            controllerCompat.show(WindowInsetsCompat.Type.navigationBars());
          }
          getWindow().setNavigationBarColor(Color.WHITE);
          if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            WindowInsetsController c = getWindow().getInsetsController();
            if (c != null) {
              c.setSystemBarsAppearance(
                WindowInsetsController.APPEARANCE_LIGHT_NAVIGATION_BARS,
                WindowInsetsController.APPEARANCE_LIGHT_NAVIGATION_BARS
              );
            }
          } else if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            int vis = getWindow().getDecorView().getSystemUiVisibility();
            vis |= View.SYSTEM_UI_FLAG_LIGHT_NAVIGATION_BAR;
            getWindow().getDecorView().setSystemUiVisibility(vis);
          }
        }

        // 请求刷新 Insets，避免残留一帧错位
        ViewCompat.requestApplyInsets(decor);

        return insets;
      });

      // 动画期间强制隐藏底部遮罩，结束后按可见性规则恢复，避免闪烁/重影
      ViewCompat.setWindowInsetsAnimationCallback(decor,
        new WindowInsetsAnimationCompat.Callback(WindowInsetsAnimationCompat.Callback.DISPATCH_MODE_STOP) {
          @Override
          public WindowInsetsCompat onProgress(WindowInsetsCompat insets, List<WindowInsetsAnimationCompat> runningAnimations) {
            boolean imeVisible = insets.isVisible(WindowInsetsCompat.Type.ime());
            boolean navVisible = insets.isVisible(WindowInsetsCompat.Type.navigationBars());
            int bottom = insets.getInsets(WindowInsetsCompat.Type.navigationBars()).bottom;

            FrameLayout.LayoutParams blp = (FrameLayout.LayoutParams) navigationBarScrimView.getLayoutParams();
            blp.height = navVisible ? bottom : 0;
            navigationBarScrimView.setLayoutParams(blp);
            navigationBarScrimView.setVisibility((navVisible && !imeVisible) ? View.VISIBLE : View.GONE);
            return insets;
          }
        }
      );
    } catch (Exception ignored) {}
  }

  @Override
  public void onResume() {
    super.onResume();

    // 再次确保系统栏颜色与外观生效，避免被后续覆盖
    getWindow().setStatusBarColor(Color.WHITE);
    getWindow().setNavigationBarColor(Color.WHITE);

    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
      WindowInsetsController controller = getWindow().getInsetsController();
      if (controller != null) {
        controller.setSystemBarsAppearance(
            WindowInsetsController.APPEARANCE_LIGHT_STATUS_BARS,
            WindowInsetsController.APPEARANCE_LIGHT_STATUS_BARS
        );
        controller.setSystemBarsAppearance(
            WindowInsetsController.APPEARANCE_LIGHT_NAVIGATION_BARS,
            WindowInsetsController.APPEARANCE_LIGHT_NAVIGATION_BARS
        );
      }
    }

    // 再次禁用对比度强制（部分 ROM 切应用前后台后会还原）
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
      try {
        getWindow().setStatusBarContrastEnforced(false);
        getWindow().setNavigationBarContrastEnforced(false);
      } catch (Throwable ignored) {}
    }
  }
}

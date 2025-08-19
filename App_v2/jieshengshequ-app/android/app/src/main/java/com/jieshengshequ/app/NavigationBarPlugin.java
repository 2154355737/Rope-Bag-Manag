package com.jieshengshequ.app;

import android.content.Context;
import android.content.res.Configuration;
import android.content.res.Resources;
import android.graphics.Color;
import android.graphics.Point;
import android.os.Build;
import android.provider.Settings;
import android.util.DisplayMetrics;
import android.view.Display;
import android.view.WindowManager;
import android.view.WindowInsets;
import android.view.WindowInsetsController;

import com.getcapacitor.JSObject;
import com.getcapacitor.Plugin;
import com.getcapacitor.PluginCall;
import com.getcapacitor.PluginMethod;
import com.getcapacitor.annotation.CapacitorPlugin;

@CapacitorPlugin(name = "NavigationBarPlugin")
public class NavigationBarPlugin extends Plugin {

    @PluginMethod
    public void getNavigationBarInfo(PluginCall call) {
        JSObject result = new JSObject();
        
        try {
            Context context = getContext();
            WindowManager windowManager = (WindowManager) context.getSystemService(Context.WINDOW_SERVICE);
            Display display = windowManager.getDefaultDisplay();
            
            // 获取设备信息
            JSObject deviceInfo = new JSObject();
            deviceInfo.put("brand", Build.BRAND);
            deviceInfo.put("model", Build.MODEL);
            deviceInfo.put("sdkVersion", Build.VERSION.SDK_INT);
            deviceInfo.put("hasNotch", hasNotch(context));
            
            // 获取屏幕尺寸信息
            DisplayMetrics displayMetrics = new DisplayMetrics();
            display.getMetrics(displayMetrics);
            
            Point screenSize = new Point();
            display.getSize(screenSize);
            
            Point realSize = new Point();
            display.getRealSize(realSize);
            
            // 计算导航栏高度
            int navigationBarHeight = 0;
            boolean hasNavigationBar = false;
            
            // 方法1: 通过屏幕尺寸差异检测
            int heightDifference = realSize.y - screenSize.y;
            int widthDifference = realSize.x - screenSize.x;
            
            if (heightDifference > 0 || widthDifference > 0) {
                hasNavigationBar = true;
                navigationBarHeight = Math.max(heightDifference, widthDifference);
            }
            
            // 方法2: 通过资源获取导航栏高度
            int resourceNavBarHeight = getNavigationBarHeightFromResources(context);
            if (resourceNavBarHeight > 0) {
                hasNavigationBar = true;
                navigationBarHeight = Math.max(navigationBarHeight, resourceNavBarHeight);
            }
            
            // 方法3: 检查导航栏设置（Android 10+）
            int navigationType = getNavigationType(context);
            boolean isVisible = isNavigationBarVisible();
            boolean isFullscreen = isFullscreenMode();
            
            // 如果是全屏模式，导航栏可能被隐藏
            if (isFullscreen) {
                isVisible = false;
            }
            
            // 获取屏幕方向
            int orientation = context.getResources().getConfiguration().orientation;
            
            // 构建结果
            result.put("hasNavigationBar", hasNavigationBar);
            result.put("navigationBarHeight", navigationBarHeight);
            result.put("navigationType", navigationType);
            result.put("isVisible", isVisible && hasNavigationBar);
            result.put("isFullscreen", isFullscreen);
            result.put("orientation", orientation);
            result.put("deviceInfo", deviceInfo);
            
            // 调试信息
            JSObject debugInfo = new JSObject();
            debugInfo.put("screenSize", screenSize.x + "x" + screenSize.y);
            debugInfo.put("realSize", realSize.x + "x" + realSize.y);
            debugInfo.put("heightDifference", heightDifference);
            debugInfo.put("widthDifference", widthDifference);
            debugInfo.put("resourceNavBarHeight", resourceNavBarHeight);
            result.put("debugInfo", debugInfo);
            
            call.resolve(result);
            
        } catch (Exception e) {
            call.reject("获取导航栏信息失败: " + e.getMessage());
        }
    }

    // setScrimColors方法已被移除，因为不再需要复杂的系统栏处理

    /**
     * 从系统资源获取导航栏高度
     */
    private int getNavigationBarHeightFromResources(Context context) {
        Resources resources = context.getResources();
        int resourceId = resources.getIdentifier("navigation_bar_height", "dimen", "android");
        if (resourceId > 0) {
            return resources.getDimensionPixelSize(resourceId);
        }
        return 0;
    }
    
    /**
     * 获取导航栏类型
     * 0 = 无导航栏
     * 1 = 传统按键导航
     * 2 = 手势导航
     */
    private int getNavigationType(Context context) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            try {
                int navBarMode = Settings.Secure.getInt(
                    context.getContentResolver(),
                    "navigation_mode",
                    0
                );
                switch (navBarMode) {
                    case 0:
                    case 1:
                        return 1; // 按键导航
                    case 2:
                        return 2; // 手势导航
                    default:
                        return 1;
                }
            } catch (Exception e) {
                return hasPhysicalNavigationBar(context) ? 1 : 2;
            }
        } else {
            return hasPhysicalNavigationBar(context) ? 1 : 1;
        }
    }
    
    private boolean hasPhysicalNavigationBar(Context context) {
        boolean hasMenuKey = android.view.ViewConfiguration.get(context).hasPermanentMenuKey();
        boolean hasBackKey = android.view.KeyCharacterMap.deviceHasKey(android.view.KeyEvent.KEYCODE_BACK);
        return !hasMenuKey && !hasBackKey;
    }
    
    private boolean isNavigationBarVisible() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            try {
                WindowInsets insets = getActivity().getWindow().getDecorView().getRootWindowInsets();
                if (insets != null) {
                    return insets.isVisible(WindowInsets.Type.navigationBars());
                }
            } catch (Exception e) {
            }
        }
        return true;
    }
    
    private boolean isFullscreenMode() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            try {
                WindowInsetsController controller = getActivity().getWindow().getInsetsController();
                if (controller != null) {
                    WindowInsets insets = getActivity().getWindow().getDecorView().getRootWindowInsets();
                    return insets != null && !insets.isVisible(WindowInsets.Type.systemBars());
                }
            } catch (Exception e) {
            }
        }
        int flags = getActivity().getWindow().getDecorView().getSystemUiVisibility();
        return (flags & android.view.View.SYSTEM_UI_FLAG_FULLSCREEN) != 0;
    }
    
    private boolean hasNotch(Context context) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
            try {
                WindowInsets insets = getActivity().getWindow().getDecorView().getRootWindowInsets();
                if (insets != null && insets.getDisplayCutout() != null) {
                    return true;
                }
            } catch (Exception e) {
            }
        }
        return checkVendorNotch(context);
    }

    private boolean checkVendorNotch(Context context) {
        String brand = Build.BRAND.toLowerCase();
        if (brand.contains("xiaomi") || brand.contains("redmi")) {
            try {
                Class<?> clazz = Class.forName("android.os.SystemProperties");
                java.lang.reflect.Method method = clazz.getMethod("getInt", String.class, int.class);
                int hasNotch = (int) method.invoke(null, "ro.miui.notch", 0);
                return hasNotch == 1;
            } catch (Exception e) {
            }
        }
        if (brand.contains("huawei") || brand.contains("honor")) {
            try {
                Class<?> clazz = Class.forName("com.huawei.android.util.HwNotchSizeUtil");
                java.lang.reflect.Method method = clazz.getMethod("hasNotchInScreen");
                return (boolean) method.invoke(null);
            } catch (Exception e) {
            }
        }
        return false;
    }
} 
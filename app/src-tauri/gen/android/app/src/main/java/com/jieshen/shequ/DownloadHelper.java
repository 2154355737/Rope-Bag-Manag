package com.jieshen.shequ;

import android.app.DownloadManager;
import android.content.Context;
import android.net.Uri;
import android.os.Environment;
import android.webkit.DownloadListener;
import android.webkit.URLUtil;
import android.widget.Toast;

public class DownloadHelper {
    
    public static void setupWebViewDownload(Context context, android.webkit.WebView webView) {
        webView.setDownloadListener(new DownloadListener() {
            @Override
            public void onDownloadStart(String url, String userAgent, String contentDisposition, 
                                      String mimeType, long contentLength) {
                
                // 使用Android系统下载管理器
                DownloadManager.Request request = new DownloadManager.Request(Uri.parse(url));
                
                // 设置下载标题
                String fileName = URLUtil.guessFileName(url, contentDisposition, mimeType);
                request.setTitle("下载文件");
                request.setDescription("正在下载: " + fileName);
                
                // 设置下载目录
                request.setDestinationInExternalPublicDir(Environment.DIRECTORY_DOWNLOADS, fileName);
                
                // 设置网络类型
                request.setAllowedNetworkTypes(DownloadManager.Request.NETWORK_WIFI | 
                                             DownloadManager.Request.NETWORK_MOBILE);
                
                // 设置漫游状态下是否可以下载
                request.setAllowedOverRoaming(false);
                
                // 设置文件类型
                request.setMimeType(mimeType);
                
                // 在通知栏显示下载进度
                request.setNotificationVisibility(DownloadManager.Request.VISIBILITY_VISIBLE_NOTIFY_COMPLETED);
                
                // 设置为可被媒体扫描器找到
                request.allowScanningByMediaScanner();
                
                // 设置为可见和可管理
                request.setVisibleInDownloadsUi(true);
                
                try {
                    // 获取下载管理器
                    DownloadManager downloadManager = (DownloadManager) context.getSystemService(Context.DOWNLOAD_SERVICE);
                    
                    // 开始下载
                    long downloadId = downloadManager.enqueue(request);
                    
                    // 显示提示
                    Toast.makeText(context, "开始下载: " + fileName, Toast.LENGTH_SHORT).show();
                    
                } catch (Exception e) {
                    e.printStackTrace();
                    Toast.makeText(context, "下载失败: " + e.getMessage(), Toast.LENGTH_SHORT).show();
                }
            }
        });
    }
} 
// 触发Android系统下载的命令
#[tauri::command]
fn trigger_system_download(url: String, filename: String) -> Result<String, String> {
    log::info!("触发系统下载: {} -> {}", url, filename);
    
    #[cfg(target_os = "android")]
    {
        // 在Android上，我们通过JavaScript触发系统下载
        Ok(format!("android_download:{}:{}", url, filename))
    }
    
    #[cfg(not(target_os = "android"))]
    {
        // 非Android平台返回URL
        Ok(url)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![trigger_system_download])
        .setup(|_app| {
            log::info!("Tauri应用启动成功");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 创建基础Tauri应用，不使用可能导致崩溃的插件
    tauri::Builder::default()
        .setup(|_app| {
            // 最小化设置
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

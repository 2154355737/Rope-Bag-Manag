use std::thread;
use std::io::{self, BufRead};
use crate::models::Users;

pub fn start_command_listener(users: Users) {
    thread::spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line.unwrap().trim() {
                "reload-config" => {
                    println!("[CMD] 重新加载配置文件...（请重启服务以生效）");
                    // 热重载可选实现
                }
                "show-users" => {
                    println!("[CMD] 显示所有用户:");
                    let users = users.lock().unwrap();
                    for (name, user) in users.iter() {
                        println!("- {} (昵称: {}, 星级: {}, 管理员: {})", name, user.nickname, user.star, user.is_admin);
                    }
                }
                "clear-logs" => {
                    println!("[CMD] 清空日志文件...");
                    let _ = std::fs::write("logs/app.log", "");
                }
                "exit" => {
                    println!("[CMD] 服务即将关闭...");
                    std::process::exit(0);
                }
                "help" => {
                    println!("[CMD] 支持的指令: reload-config | show-users | clear-logs | exit | help");
                }
                other => {
                    println!("[CMD] 未知指令: {}", other);
                }
            }
        }
    });
} 
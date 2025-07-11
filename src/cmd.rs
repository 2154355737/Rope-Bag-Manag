use std::thread;
use std::io::{self, BufRead};
use crate::models::Users;
use crate::models::{RopePackages};
use crate::utils::{load_json, save_json};
use std::process::Command;

pub fn start_command_listener(users: Users, ropes: RopePackages, config: crate::config::AppConfig) {
    thread::spawn(move || {
        let stdin = io::stdin();
        let mut users_ref = users.clone();
        let mut ropes_ref = ropes.clone();
        let mut config_ref = config.clone();
        for line in stdin.lock().lines() {
            let input = line.unwrap();
            let args: Vec<&str> = input.trim().split_whitespace().collect();
            if args.is_empty() { continue; }
            match args[0] {
                "reload-config" => {
                    println!("[CMD] 重新加载配置文件...（请重启服务以生效）");
                }
                "show-users" => {
                    println!("[CMD] 显示所有用户:");
                    let users = users_ref.lock().unwrap();
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
                "add-rope" => {
                    if args.len() < 6 {
                        println!("用法: add-rope <绳包名称> <作者> <版本> <简介> <项目直链>");
                        continue;
                    }
                    let name = args[1];
                    let author = args[2];
                    let version = args[3];
                    let desc = args[4];
                    let url = args[5];
                    let mut ropes = ropes_ref.lock().unwrap();
                    let id = ropes.len() as u32 + 1;
                    ropes.insert(id, crate::models::RopePackage {
                        id,
                        name: name.to_string(),
                        author: author.to_string(),
                        version: version.to_string(),
                        desc: desc.to_string(),
                        url: url.to_string(),
                        downloads: 0,
                    });
                    save_json("data/data.json", &*ropes);
                    println!("[CMD] 绳包添加成功");
                }
                "delete-rope" => {
                    if args.len() < 2 {
                        println!("用法: delete-rope <id>");
                        continue;
                    }
                    let id = args[1].parse::<u32>().unwrap_or(0);
                    let mut ropes = ropes_ref.lock().unwrap();
                    if ropes.remove(&id).is_some() {
                        save_json("data/data.json", &*ropes);
                        println!("[CMD] 绳包删除成功");
                    } else {
                        println!("[CMD] 绳包不存在");
                    }
                }
                "reload-users" => {
                    println!("[CMD] 重新加载用户数据库...");
                    let new_users = load_json("data/users.json");
                    let mut users = users_ref.lock().unwrap();
                    *users = new_users;
                    println!("[CMD] 用户数据库已重载");
                }
                "safe-exit" => {
                    println!("[CMD] 正在安全关闭服务端...");
                    // 可在此处添加数据保存等操作
                    std::process::exit(0);
                }
                "restart" => {
                    println!("[CMD] 正在重启服务端...");
                    // Windows下简单实现：重新启动自身
                    let exe = std::env::current_exe().unwrap();
                    Command::new(exe).spawn().unwrap();
                    std::process::exit(0);
                }
                "help" => {
                    println!("[CMD] 支持的指令: reload-config | show-users | clear-logs | exit | add-rope | delete-rope | reload-users | safe-exit | restart | help");
                }
                other => {
                    println!("[CMD] 未知指令: {}", other);
                }
            }
        }
    });
} 
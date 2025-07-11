use std::thread;
use std::io::{self, BufRead};
use crate::models::Users;
use crate::models::{RopePackages};
use crate::utils::{load_json, save_json};
use std::process::Command;

pub fn start_command_listener(users: Users, ropes: RopePackages, _config: crate::config::AppConfig) {
    thread::spawn(move || {
        let stdin = io::stdin();
        let users_ref = users.clone();
        let ropes_ref = ropes.clone();
        // let config_ref = config.clone(); // 已不使用
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
                    // 获取当前时间作为上架时间
                    let upload_time = chrono::Local::now().format("%Y-%m-%d").to_string();
                    
                    ropes.insert(id, crate::models::RopePackage {
                        id,
                        name: name.to_string(),
                        author: author.to_string(),
                        version: version.to_string(),
                        desc: desc.to_string(),
                        url: url.to_string(),
                        downloads: 0,
                        upload_time,
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
                    println!("\n=== 绳包管理器命令帮助 ===");
                    println!("[常用命令]");
                    println!("  show-users         显示所有用户");
                    println!("  reload-users       重新加载用户数据库");
                    println!("  clear-logs         清空日志文件");
                    println!("  exit               立即关闭服务端");
                    println!("  safe-exit          安全关闭服务端（推荐）");
                    println!("  restart            重启服务端");
                    println!("");
                    println!("[绳包管理]");
                    println!("  add-rope <name> <author> <version> <desc> <url>   添加绳包（本地/管理员）");
                    println!("  delete-rope <id>                                   删除绳包（本地/管理员）");
                    println!("");
                    println!("[说明]");
                    println!("  1. 管理员相关API请用Web接口，终端命令仅限本地维护。");
                    println!("  2. 参数如有空格请用引号包裹，url需完整。");
                    println!("  3. 详细API请查阅 docs/接口文档.md 或输入 help-api。");
                    println!("");
                    println!("更多命令可根据实际需求扩展。\n");
                }
                other => {
                    println!("[CMD] 未知指令: {}", other);
                }
            }
        }
    });
} 
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("data.db")?;
    
    // 启用邮件服务
    conn.execute(
        "UPDATE mail_settings SET enabled = 1 WHERE id = 1",
        [],
    )?;
    
    println!("邮件服务已启用！");
    
    // 查看当前配置
    let mut stmt = conn.prepare("SELECT * FROM mail_settings WHERE id = 1")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,      // id
            row.get::<_, String>(1)?,   // smtp_server
            row.get::<_, i32>(2)?,      // smtp_port
            row.get::<_, String>(3)?,   // username
            row.get::<_, String>(4)?,   // password (隐藏)
            row.get::<_, String>(5)?,   // from_name
            row.get::<_, i32>(6)?,      // enabled
            row.get::<_, i32>(7)?,      // use_ssl
            row.get::<_, i32>(8)?,      // auth_required
        ))
    })?;

    for row in rows {
        let (id, server, port, username, _, from_name, enabled, use_ssl, auth_required) = row?;
        println!("配置信息:");
        println!("  ID: {}", id);
        println!("  SMTP服务器: {}", server);
        println!("  端口: {}", port);
        println!("  用户名: {}", username);
        println!("  发送方名称: {}", from_name);
        println!("  已启用: {}", if enabled == 1 { "是" } else { "否" });
        println!("  使用SSL: {}", if use_ssl == 1 { "是" } else { "否" });
        println!("  需要认证: {}", if auth_required == 1 { "是" } else { "否" });
    }
    
    Ok(())
} 
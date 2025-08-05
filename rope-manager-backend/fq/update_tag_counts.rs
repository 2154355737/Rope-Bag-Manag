use rusqlite::{Connection, Result, params};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("data.db")?;
    
    println!("开始统计标签真实使用次数...");
    
    // 1. 先将所有标签的使用次数重置为0
    conn.execute("UPDATE tags SET use_count = 0", [])?;
    println!("✓ 已重置所有标签使用次数为0");
    
    // 2. 统计每个标签在帖子中的使用次数
    let mut stmt = conn.prepare("
        SELECT tag_id, COUNT(*) as count 
        FROM post_tags 
        GROUP BY tag_id
    ")?;
    
    let post_tag_counts = stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?))
    })?;
    
    let mut total_post_usage = 0;
    for result in post_tag_counts {
        let (tag_id, count) = result?;
        total_post_usage += count;
        println!("标签 ID {} 在帖子中使用了 {} 次", tag_id, count);
    }
    
    // 3. 统计每个标签在资源包中的使用次数（如果表存在的话）
    let mut total_package_usage = 0;
    
    // 检查 package_tags 表是否存在
    let table_exists = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='package_tags'");
    let package_tags_exists = match table_exists {
        Ok(mut stmt) => {
            match stmt.exists([]) {
                Ok(exists) => exists,
                Err(_) => false,
            }
        },
        Err(_) => false,
    };
    
    if package_tags_exists {
        let mut stmt = conn.prepare("
            SELECT tag_id, COUNT(*) as count 
            FROM package_tags 
            GROUP BY tag_id
        ")?;
        
        let package_tag_counts = stmt.query_map([], |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?))
        })?;
        
        for result in package_tag_counts {
            let (tag_id, count) = result?;
            total_package_usage += count;
            println!("标签 ID {} 在资源包中使用了 {} 次", tag_id, count);
        }
    } else {
        println!("⚠️ package_tags 表不存在，跳过资源包标签统计");
    }
    
    // 4. 更新标签使用次数：帖子使用次数 + 资源包使用次数
    let update_sql = if package_tags_exists {
        "UPDATE tags 
         SET use_count = (
             COALESCE((
                 SELECT COUNT(*) FROM post_tags WHERE tag_id = tags.id
             ), 0) + 
             COALESCE((
                 SELECT COUNT(*) FROM package_tags WHERE tag_id = tags.id
             ), 0)
         ),
         updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')"
    } else {
        "UPDATE tags 
         SET use_count = (
             COALESCE((
                 SELECT COUNT(*) FROM post_tags WHERE tag_id = tags.id
             ), 0)
         ),
         updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')"
    };
    
    let mut stmt = conn.prepare(update_sql)?;
    
    stmt.execute([])?;
    println!("✓ 已更新所有标签的使用次数");
    
    // 5. 显示统计结果
    println!("\n=== 统计结果 ===");
    println!("帖子标签使用总次数: {}", total_post_usage);
    println!("资源包标签使用总次数: {}", total_package_usage);
    println!("标签使用总次数: {}", total_post_usage + total_package_usage);
    
    // 6. 显示更新后的标签使用情况
    println!("\n=== 标签使用次数排行榜 ===");
    let query_sql = if package_tags_exists {
        "SELECT id, name, use_count, 
                (SELECT COUNT(*) FROM post_tags WHERE tag_id = tags.id) as post_count,
                (SELECT COUNT(*) FROM package_tags WHERE tag_id = tags.id) as package_count
         FROM tags 
         ORDER BY use_count DESC, name ASC"
    } else {
        "SELECT id, name, use_count, 
                (SELECT COUNT(*) FROM post_tags WHERE tag_id = tags.id) as post_count,
                0 as package_count
         FROM tags 
         ORDER BY use_count DESC, name ASC"
    };
    
    let mut stmt = conn.prepare(query_sql)?;
    
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,      // id
            row.get::<_, String>(1)?,   // name
            row.get::<_, i32>(2)?,      // use_count
            row.get::<_, i32>(3)?,      // post_count
            row.get::<_, i32>(4)?,      // package_count
        ))
    })?;
    
    for (index, row) in rows.enumerate() {
        let (id, name, total_count, post_count, package_count) = row?;
        println!(
            "{}. [ID:{}] {} - 总计: {} 次 (帖子: {} 次, 资源: {} 次)", 
            index + 1, id, name, total_count, post_count, package_count
        );
    }
    
    // 7. 显示未使用的标签
    println!("\n=== 未使用的标签 ===");
    let mut stmt = conn.prepare("SELECT id, name FROM tags WHERE use_count = 0 ORDER BY name")?;
    let unused_tags = stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    })?;
    
    let mut unused_count = 0;
    for result in unused_tags {
        let (id, name) = result?;
        println!("- [ID:{}] {}", id, name);
        unused_count += 1;
    }
    
    if unused_count == 0 {
        println!("所有标签都有被使用！");
    } else {
        println!("共有 {} 个标签未被使用", unused_count);
    }
    
    println!("\n✅ 标签使用次数统计完成！");
    
    Ok(())
} 
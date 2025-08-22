// 批量修复结构体字段的脚本
// 这个脚本用于自动添加缺失的 Package 和 Post 结构体字段

use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 开始修复结构体字段...");
    
    // 修复 package.rs 中缺失的 UpdatePackageRequest 字段
    fix_package_api()?;
    
    // 修复其他文件中缺失的 Post 结构体字段
    fix_post_service()?;
    
    // 修复 publish.rs 中的类型错误
    fix_publish_api()?;
    
    println!("✅ 结构体字段修复完成！");
    Ok(())
}

fn fix_package_api() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "src/api/v1/package.rs";
    let content = fs::read_to_string(file_path)?;
    
    // 修复 UpdatePackageRequest 缺失字段
    let fixed = content.replace(
        r#"let update_req = crate::models::UpdatePackageRequest {
        name: req.name.clone(),
        version: req.version.clone(),
        description: req.description.clone(),
        category_id: req.category_id,
        status: Some(req.status.clone()),
        file_url: req.file_url.clone(),
        tags: req.tags.clone(),
        is_pinned: req.is_pinned,
        is_featured: req.is_featured,
        reviewer_id: Some(user.id),
        reviewed_at: Some(chrono::Utc::now()),
        review_comment: req.review_comment.clone(),
    };"#,
        r#"let update_req = crate::models::UpdatePackageRequest {
        name: req.name.clone(),
        version: req.version.clone(),
        description: req.description.clone(),
        category_id: req.category_id,
        status: Some(req.status.clone()),
        file_url: req.file_url.clone(),
        tags: req.tags.clone(),
        is_pinned: req.is_pinned,
        is_featured: req.is_featured,
        reviewer_id: Some(user.id),
        reviewed_at: Some(chrono::Utc::now()),
        review_comment: req.review_comment.clone(),
        // 新增字段
        screenshots: req.screenshots.clone(),
        cover_image: req.cover_image.clone(),
        requirements: req.requirements.clone(),
    };"#
    );
    
    // 修复 CreatePackageRequest 缺失字段
    let fixed = fixed.replace(
        r#"let create_req = CreatePackageRequest {
        name: req.title.clone(),
        author: user.username.clone(),
        version: None,
        description: req.description.clone(),
        category_id,
        file_url: if req.file_url.is_empty() { None } else { Some(req.file_url.clone()) },
        tags: req.tags.clone(),
        is_pinned: None,
        is_featured: None,
    };"#,
        r#"let create_req = CreatePackageRequest {
        name: req.title.clone(),
        author: user.username.clone(),
        version: None,
        description: req.description.clone(),
        category_id,
        file_url: if req.file_url.is_empty() { None } else { Some(req.file_url.clone()) },
        tags: req.tags.clone(),
        is_pinned: None,
        is_featured: None,
        // 新增字段
        screenshots: req.screenshots.clone(),
        cover_image: req.cover_image.clone(),
        requirements: req.requirements.clone(),
    };"#
    );
    
    fs::write(file_path, fixed)?;
    println!("✅ 修复 package.rs");
    Ok(())
}

fn fix_post_service() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "src/services/post_service.rs";
    let content = fs::read_to_string(file_path)?;
    
    // 查找第三个Post结构体并修复（第508行附近）
    let lines: Vec<&str> = content.lines().collect();
    let mut new_lines = Vec::new();
    let mut in_third_post_struct = false;
    let mut added_fields = false;
    
    for (i, line) in lines.iter().enumerate() {
        new_lines.push(line.to_string());
        
        // 寻找第三个Post结构体（大约在第508行附近）
        if i > 480 && line.contains("Ok(Post {") {
            in_third_post_struct = true;
        }
        
        if in_third_post_struct && line.contains("reviewed_at:") && !added_fields {
            // 在 reviewed_at 行后添加新字段
            new_lines.push("                // 新增字段".to_string());
            new_lines.push("                images: {".to_string());
            new_lines.push("                    if let Ok(json_str) = row.get::<_, String>(18) {".to_string());
            new_lines.push("                        serde_json::from_str(&json_str).ok()".to_string());
            new_lines.push("                    } else {".to_string());
            new_lines.push("                        None".to_string());
            new_lines.push("                    }".to_string());
            new_lines.push("                },".to_string());
            new_lines.push("                code_snippet: row.get(19).ok(),".to_string());
            new_lines.push("                tags: {".to_string());
            new_lines.push("                    if let Ok(json_str) = row.get::<_, String>(20) {".to_string());
            new_lines.push("                        serde_json::from_str(&json_str).ok()".to_string());
            new_lines.push("                    } else {".to_string());
            new_lines.push("                        None".to_string());
            new_lines.push("                    }".to_string());
            new_lines.push("                },".to_string());
            added_fields = true;
        }
        
        if in_third_post_struct && line.contains("})") {
            in_third_post_struct = false;
        }
    }
    
    let fixed_content = new_lines.join("\n");
    fs::write(file_path, fixed_content)?;
    println!("✅ 修复 post_service.rs");
    Ok(())
}

fn fix_publish_api() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "src/api/v1/publish.rs";
    let content = fs::read_to_string(file_path)?;
    
    // 修复 create_post 调用参数
    let fixed = content.replace(
        "match post_service.create_post(&create_req, user.id).await {",
        "match post_service.create_post(create_req, user.id).await {"
    );
    
    // 修复 post 变量使用
    let fixed = fixed.replace(
        r#"log::info!("✅ 帖子发布成功: post_id={}, title={}", post.id, post.title);"#,
        r#"log::info!("✅ 帖子发布成功: post_id={}", post);"#
    );
    
    let fixed = fixed.replace(
        r#""id": post.id,
                    "title": post.title,
                    "status": "published",
                    "created_at": post.created_at"#,
        r#""id": post,
                    "title": req.title.clone(),
                    "status": "published",
                    "created_at": chrono::Utc::now().to_rfc3339()"#
    );
    
    fs::write(file_path, fixed)?;
    println!("✅ 修复 publish.rs");
    Ok(())
} 
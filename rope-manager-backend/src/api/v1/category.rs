use actix_web::{web, HttpResponse};
use serde_json::json;
use crate::models::system::{CreateCategoryRequest, UpdateCategoryRequest};
use crate::repositories::system_repo::SystemRepository;
use crate::repositories::package_repo::PackageRepository;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/categories")
            .route("", web::get().to(get_categories))
            .route("", web::post().to(create_category))
            .route("/{id}", web::get().to(get_category))
            .route("/{id}", web::put().to(update_category))
            .route("/{id}", web::delete().to(delete_category))
    );
}

// 获取所有分类
async fn get_categories(
    system_repo: web::Data<SystemRepository>,
    package_repo: web::Data<PackageRepository>,
) -> HttpResponse {
    // 从数据库获取分类
    match system_repo.get_categories().await {
        Ok(categories) => {
            // 手动构建包含count的JSON响应
            let mut categories_with_count = Vec::new();
            
            for cat in categories.iter() {
                let count = package_repo.count_packages_by_category(cat.id).await
                    .unwrap_or(0);
                
                categories_with_count.push(json!({
                    "id": cat.id,
                    "name": cat.name,
                    "description": cat.description,
                    "enabled": cat.enabled,
                    "subscription_locked": cat.subscription_locked,
                    "created_at": cat.created_at,
                    "updated_at": cat.updated_at,
                    "count": count
                }));
            }
            
            HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": {
                    "list": categories_with_count
            }
            }))
        },
        Err(e) => {
            println!("从数据库获取分类失败：{}", e);
            HttpResponse::InternalServerError().json(json!({
                "code": 500,
                "message": format!("获取分类数据失败: {}", e)
            }))
        }
    }
}

// 获取单个分类
async fn get_category(
    path: web::Path<i32>,
    system_repo: web::Data<SystemRepository>,
) -> HttpResponse {
    let category_id = path.into_inner();
    
    match system_repo.get_category_by_id(category_id).await {
        Ok(Some(category)) => HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "success",
            "data": category
        })),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "code": 404,
            "message": "分类不存在"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("获取分类失败: {}", e)
        }))
    }
}

// 创建分类
async fn create_category(
    req: web::Json<CreateCategoryRequest>,
    system_repo: web::Data<SystemRepository>,
) -> HttpResponse {
    match system_repo.create_category(&req).await {
        Ok(category) => HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "分类创建成功",
            "data": category
        })),
        Err(e) => HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": format!("创建分类失败: {}", e)
        }))
    }
}

// 更新分类
async fn update_category(
    path: web::Path<i32>,
    req: web::Json<UpdateCategoryRequest>,
    system_repo: web::Data<SystemRepository>,
) -> HttpResponse {
    let category_id = path.into_inner();
    
    match system_repo.update_category(category_id, &req).await {
        Ok(category) => HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "分类更新成功",
            "data": category
        })),
        Err(e) => HttpResponse::BadRequest().json(json!({
            "code": 400,
            "message": format!("更新分类失败: {}", e)
        }))
    }
}

// 删除分类
async fn delete_category(
    path: web::Path<i32>,
    system_repo: web::Data<SystemRepository>,
) -> HttpResponse {
    let category_id = path.into_inner();
    
    match system_repo.delete_category(category_id).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "code": 0,
            "message": "分类删除成功"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "code": 500,
            "message": format!("删除分类失败: {}", e)
        }))
    }
} 
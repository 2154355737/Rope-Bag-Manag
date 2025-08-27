// 修复Vec字段的帮助函数
use serde_json;

/// 解析JSON数组，返回Vec<String>而不是Option<Vec<String>>
pub fn parse_json_array_vec(json_opt: Option<String>) -> Vec<String> {
    match json_opt {
        Some(json_str) if !json_str.trim().is_empty() => {
            serde_json::from_str(&json_str).unwrap_or_default()
        },
        _ => Vec::new()
    }
}

/// 将Vec<String>序列化为JSON字符串
pub fn serialize_vec_to_json(vec: &Vec<String>) -> Option<String> {
    if vec.is_empty() {
        None
    } else {
        serde_json::to_string(vec).ok()
    }
}

fn main() {
    println!("这是一个帮助函数库，用于处理Vec字段转换");
} 
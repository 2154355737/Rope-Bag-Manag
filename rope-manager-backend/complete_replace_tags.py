#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import re

# Complete mapping from English to Chinese tags
tag_mapping = {
    # 之前已经替换的标签
    '- Auth': '- 用户认证',
    '- Users': '- 用户管理',
    '- Packages': '- 绳包管理',
    '- Comments': '- 评论系统',
    '- Posts': '- 帖子系统',
    '- Tags': '- 标签管理',
    '- Categories': '- 分类管理',
    '- Admin': '- 系统管理',
    '- Health': '- 系统健康',
    
    # 遗漏的标签
    '- Download Security': '- 下载安全',
    '- Community': '- 社区动态',
    '- Feed': '- 动态流',
    '- Resource Records': '- 资源记录',
    '- User Actions': '- 用户行为',
    '- Forbidden Words': '- 违禁词管理',
    '- Subscriptions': '- 订阅管理',
    '- Cache': '- 缓存管理',
    '- Security Management': '- 安全管理',
    '- Announcements': '- 公告管理',
    '- Public': '- 公开接口',
    '- Schemas': '- 数据模型',
}

def replace_tags_in_file(filename):
    try:
        # Read the file
        with open(filename, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Keep track of replacements made
        replacements_made = []
        
        # Replace each English tag with its Chinese equivalent
        for english_tag, chinese_tag in tag_mapping.items():
            old_content = content
            content = content.replace(english_tag, chinese_tag)
            if old_content != content:
                count = old_content.count(english_tag)
                replacements_made.append(f"{english_tag} -> {chinese_tag} ({count} occurrences)")
        
        # Write back to file
        with open(filename, 'w', encoding='utf-8') as f:
            f.write(content)
        
        print(f"Successfully updated tags in {filename}")
        if replacements_made:
            print("Replacements made:")
            for replacement in replacements_made:
                print(f"  {replacement}")
        else:
            print("No replacements needed - all tags are already in Chinese")
        return True
        
    except Exception as e:
        print(f"Error processing file {filename}: {e}")
        return False

if __name__ == "__main__":
    if replace_tags_in_file("openapi.yaml"):
        print("\nAll English tags have been replaced with Chinese equivalents.")
    else:
        print("Failed to update tags.") 
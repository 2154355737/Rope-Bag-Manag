#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import requests
import json
import time
from datetime import datetime

# 服务器地址
BASE_URL = "http://127.0.0.1:15202"

def create_user_action(user_id, action_type, target_type, target_id, description, success=True, error_message=None):
    """创建用户行为记录"""
    url = f"{BASE_URL}/api/user-actions"
    
    data = {
        "action_type": action_type,
        "target_type": target_type,
        "target_id": target_id,
        "description": description,
        "success": success,
        "error_message": error_message
    }
    
    headers = {
        "Content-Type": "application/json",
        "X-Username": user_id
    }
    
    try:
        response = requests.post(url, json=data, headers=headers)
        print(f"创建行为记录: {response.status_code} - {response.text}")
        return response.json()
    except Exception as e:
        print(f"创建行为记录失败: {e}")
        return None

def get_user_actions():
    """获取用户行为记录"""
    url = f"{BASE_URL}/api/user-actions"
    
    try:
        response = requests.get(url)
        print(f"获取行为记录: {response.status_code}")
        if response.status_code == 200:
            data = response.json()
            print(f"总记录数: {data.get('data', {}).get('total', 0)}")
            return data
        else:
            print(f"获取失败: {response.text}")
            return None
    except Exception as e:
        print(f"获取行为记录失败: {e}")
        return None

def get_user_action_stats():
    """获取用户行为统计"""
    url = f"{BASE_URL}/api/user-actions/stats"
    
    try:
        response = requests.get(url)
        print(f"获取行为统计: {response.status_code}")
        if response.status_code == 200:
            data = response.json()
            print(f"统计信息: {json.dumps(data, indent=2, ensure_ascii=False)}")
            return data
        else:
            print(f"获取统计失败: {response.text}")
            return None
    except Exception as e:
        print(f"获取行为统计失败: {e}")
        return None

def main():
    print("开始测试用户行为记录功能...")
    
    # 创建一些测试数据
    test_actions = [
        {
            "user_id": "admin",
            "action_type": "Login",
            "target_type": "System",
            "target_id": "login_page",
            "description": "管理员登录系统",
            "success": True
        },
        {
            "user_id": "user1",
            "action_type": "Upload",
            "target_type": "Package",
            "target_id": "package_123",
            "description": "用户上传绳包",
            "success": True
        },
        {
            "user_id": "user2",
            "action_type": "Download",
            "target_type": "Package",
            "target_id": "package_456",
            "description": "用户下载绳包",
            "success": True
        },
        {
            "user_id": "user3",
            "action_type": "Comment",
            "target_type": "Package",
            "target_id": "package_789",
            "description": "用户评论绳包",
            "success": True
        },
        {
            "user_id": "user4",
            "action_type": "Upload",
            "target_type": "Package",
            "target_id": "package_999",
            "description": "用户上传失败",
            "success": False,
            "error_message": "文件大小超过限制"
        }
    ]
    
    # 创建测试数据
    for action in test_actions:
        create_user_action(**action)
        time.sleep(0.1)  # 稍微延迟一下
    
    print("\n" + "="*50)
    print("测试数据创建完成，现在获取记录...")
    
    # 获取行为记录
    get_user_actions()
    
    print("\n" + "="*50)
    print("获取行为统计...")
    
    # 获取行为统计
    get_user_action_stats()
    
    print("\n测试完成！")

if __name__ == "__main__":
    main() 
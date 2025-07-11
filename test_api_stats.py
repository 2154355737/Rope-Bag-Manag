#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import requests
import time
import json

# 服务器配置
BASE_URL = "http://localhost:8080/api"

def test_api_stats():
    """测试API调用统计功能"""
    
    print("开始测试API调用统计功能...")
    
    # 测试1: 登录
    print("\n1. 测试登录接口")
    login_response = requests.get(f"{BASE_URL}/login", params={
        "username": "admin",
        "password": "admin123"
    })
    print(f"登录响应: {login_response.status_code}")
    
    # 测试2: 获取仪表盘数据
    print("\n2. 测试仪表盘接口")
    dashboard_response = requests.get(f"{BASE_URL}/dashboard")
    print(f"仪表盘响应: {dashboard_response.status_code}")
    
    # 测试3: 获取用户数据
    print("\n3. 测试用户数据接口")
    users_response = requests.get(f"{BASE_URL}/get-users-db")
    print(f"用户数据响应: {users_response.status_code}")
    
    # 测试4: 获取绳包数据
    print("\n4. 测试绳包数据接口")
    packages_response = requests.get(f"{BASE_URL}/get-data-db", params={
        "username": "admin"
    })
    print(f"绳包数据响应: {packages_response.status_code}")
    
    # 测试5: 获取API调用统计
    print("\n5. 测试API调用统计接口")
    api_stats_response = requests.get(f"{BASE_URL}/stats/api-calls", params={
        "username": "admin"
    })
    print(f"API调用统计响应: {api_stats_response.status_code}")
    if api_stats_response.status_code == 200:
        stats_data = api_stats_response.json()
        print(f"API调用统计数据: {json.dumps(stats_data, indent=2, ensure_ascii=False)}")
    
    # 测试6: 获取API性能详情
    print("\n6. 测试API性能详情接口")
    performance_response = requests.get(f"{BASE_URL}/stats/api-performance", params={
        "username": "admin"
    })
    print(f"API性能详情响应: {performance_response.status_code}")
    if performance_response.status_code == 200:
        perf_data = performance_response.json()
        print(f"API性能详情数据: {json.dumps(perf_data, indent=2, ensure_ascii=False)}")
    
    # 测试7: 获取最近调用记录
    print("\n7. 测试最近调用记录接口")
    recent_calls_response = requests.get(f"{BASE_URL}/stats/recent-calls", params={
        "username": "admin"
    })
    print(f"最近调用记录响应: {recent_calls_response.status_code}")
    if recent_calls_response.status_code == 200:
        calls_data = recent_calls_response.json()
        print(f"最近调用记录数据: {json.dumps(calls_data, indent=2, ensure_ascii=False)}")
    
    print("\nAPI调用统计功能测试完成!")

if __name__ == "__main__":
    test_api_stats() 
#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
后端资源/帖子浏览量防刷测试脚本
- 支持目标：Post详情(GET /api/v1/posts/{id})、Package详情(GET /api/v1/packages/{id})
- 支持模式：burst(高频突发)、drip(低频连续)、bot(可疑UA固定间隔)、mixed(混合)
- 可选：随机X-Forwarded-For、随机UA/可疑UA、并发、总请求数、间隔
- 可选：Authorization Bearer Token（模拟已登录用户）

示例：
1) 高频突发测试(帖子ID=123)
python attack_views.py --base-url http://127.0.0.1:15201 --type post --id 123 --mode burst --concurrency 50 --requests 1000 --suspicious-ua

2) 低频连续测试(资源ID=456，每请求间隔1秒)
python attack_views.py --base-url http://127.0.0.1:15201 --type package --id 456 --mode drip --interval 1.0 --requests 200 --concurrency 5

3) 机器人UA+伪IP混合测试
python attack_views.py --base-url http://127.0.0.1:15201 --type post --id 1 --mode mixed --requests 500 --concurrency 20 --xff --suspicious-ua

注意：仅用于自测你的后端防刷，勿用于任何未授权目标。
"""

import argparse
import asyncio
import random
import string
import time
from dataclasses import dataclass
from typing import List, Optional, Tuple

import aiohttp

NORMAL_UA_POOL = [
    # 常见浏览器UA
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 "
    "(KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 13_5_1) AppleWebKit/605.1.15 "
    "(KHTML, like Gecko) Version/16.6 Safari/605.1.15",
    "Mozilla/5.0 (Linux; Android 12; Pixel 5) AppleWebKit/537.36 "
    "(KHTML, like Gecko) Chrome/118.0.5993.80 Mobile Safari/537.36",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 16_6 like Mac OS X) AppleWebKit/605.1.15 "
    "(KHTML, like Gecko) Version/16.6 Mobile/15E148 Safari/604.1",
]

SUSPICIOUS_UA_POOL = [
    "curl/7.88.1",
    "Wget/1.21.3",
    "python-requests/2.31.0",
    "Scrapy/2.8.0 (+https://scrapy.org)",
    "Go-http-client/1.1",
    "Java/1.8.0",
    "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
]


def random_ipv4() -> str:
    # 避免保留网段，简单生成公用测试IP段
    return ".".join(str(random.randint(2, 254)) for _ in range(4))


def rand_str(n: int = 6) -> str:
    return "".join(random.choice(string.ascii_letters + string.digits) for _ in range(n))


@dataclass
class AttackConfig:
    base_url: str
    target_type: str  # 'post' | 'package'
    target_id: int
    mode: str  # 'burst' | 'drip' | 'bot' | 'mixed'
    concurrency: int
    total_requests: int
    interval: float
    use_xff: bool
    suspicious_ua: bool
    token: Optional[str]
    endpoint: str  # 'get' | 'view'  (post: get or view; package: get)


def build_target(cfg: AttackConfig) -> Tuple[str, str]:
    """
    返回 (method, url)
    - post: GET /api/v1/posts/{id}  或 POST /api/v1/posts/{id}/view
    - package: GET /api/v1/packages/{id}
    """
    if cfg.target_type == "post":
        if cfg.endpoint == "view":
            return "POST", f"{cfg.base_url}/api/v1/posts/{cfg.target_id}/view"
        else:
            return "GET", f"{cfg.base_url}/api/v1/posts/{cfg.target_id}"
    elif cfg.target_type == "package":
        return "GET", f"{cfg.base_url}/api/v1/packages/{cfg.target_id}"
    else:
        raise ValueError("target_type 仅支持 post 或 package")


def pick_user_agent(cfg: AttackConfig) -> str:
    if cfg.suspicious_ua:
        # 可疑UA更容易触发防御
        return random.choice(SUSPICIOUS_UA_POOL)
    # 混合或正常UA
    if cfg.mode == "mixed":
        pool = NORMAL_UA_POOL + SUSPICIOUS_UA_POOL
        return random.choice(pool)
    return random.choice(NORMAL_UA_POOL)


async def one_request(session: aiohttp.ClientSession, cfg: AttackConfig) -> Tuple[int, str]:
    method, url = build_target(cfg)
    headers = {
        "User-Agent": pick_user_agent(cfg),
        "Accept": "application/json",
    }
    if cfg.token:
        headers["Authorization"] = f"Bearer {cfg.token}"
    if cfg.use_xff:
        headers["X-Forwarded-For"] = random_ipv4()
        headers["X-Real-IP"] = headers["X-Forwarded-For"]

    try:
        if method == "GET":
            async with session.get(url, headers=headers, timeout=20) as r:
                status = r.status
                txt = await r.text()
                return status, txt[:200]
        else:
            async with session.post(url, headers=headers, timeout=20) as r:
                status = r.status
                txt = await r.text()
                return status, txt[:200]
    except Exception as e:
        return -1, f"ERR:{type(e).__name__}:{e}"


async def worker(worker_id: int, cfg: AttackConfig, n: int, delay_between: float,
                 session: aiohttp.ClientSession, results: dict):
    for i in range(n):
        status, preview = await one_request(session, cfg)
        if status == 200:
            results["ok"] += 1
        elif status in (403, 429):
            results["blocked"] += 1
        elif status == 404:
            results["not_found"] += 1
        elif status == 401:
            results["unauth"] += 1
        elif status == -1:
            results["errors"] += 1
        else:
            results["others"] += 1
        # 可选间隔
        if delay_between > 0:
            await asyncio.sleep(delay_between)


def plan(cfg: AttackConfig) -> Tuple[int, int, float]:
    """
    返回 (workers, per_worker, delay_between)
    """
    workers = cfg.concurrency
    per_worker = max(1, cfg.total_requests // max(1, workers))
    delay = 0.0

    if cfg.mode == "burst":
        delay = 0.0
    elif cfg.mode == "drip":
        # 低频连续：使用配置的 interval
        delay = max(0.1, cfg.interval)
    elif cfg.mode == "bot":
        # 固定间隔 + 可疑UA 更像机器人
        delay = cfg.interval if cfg.interval > 0 else 10.0
    elif cfg.mode == "mixed":
        # 混合：适度间隔
        delay = cfg.interval if cfg.interval > 0 else 0.3
    else:
        delay = cfg.interval

    return workers, per_worker, delay


async def main_async(cfg: AttackConfig):
    workers, per_worker, delay = plan(cfg)

    timeout = aiohttp.ClientTimeout(total=None)
    connector = aiohttp.TCPConnector(limit=0, ssl=False)
    results = {"ok": 0, "blocked": 0, "not_found": 0, "unauth": 0, "errors": 0, "others": 0}

    print(f"[START] mode={cfg.mode} type={cfg.target_type} id={cfg.target_id} "
          f"concurrency={workers} per_worker={per_worker} delay={delay:.3f} "
          f"UA={'suspicious' if cfg.suspicious_ua else 'normal/mixed'} xff={cfg.use_xff}")

    t0 = time.time()
    async with aiohttp.ClientSession(timeout=timeout, connector=connector) as session:
        tasks: List[asyncio.Task] = []
        for w in range(workers):
            tasks.append(asyncio.create_task(worker(w, cfg, per_worker, delay, session, results)))
        await asyncio.gather(*tasks)
    dur = time.time() - t0

    total = sum(results.values())
    print("\n[RESULT]")
    print(f"  total      : {total}")
    print(f"  ok(200)    : {results['ok']}")
    print(f"  blocked(4xx:403/429): {results['blocked']}")
    print(f"  not_found(404)      : {results['not_found']}")
    print(f"  unauth(401)         : {results['unauth']}")
    print(f"  errors              : {results['errors']}")
    print(f"  others              : {results['others']}")
    print(f"  duration(s)         : {dur:.2f}")
    if results["blocked"] > 0:
        print("  -> 已触发后端防护（403/429），请到 /#/system/security 查看安全日志/统计。")


def parse_args() -> AttackConfig:
    p = argparse.ArgumentParser(description="后端浏览量防刷测试脚本")
    p.add_argument("--base-url", default="http://127.0.0.1:15201", help="后端基础地址")
    p.add_argument("--type", choices=["post", "package"], required=True, help="目标类型")
    p.add_argument("--id", type=int, required=True, help="目标ID")
    p.add_argument("--mode", choices=["burst", "drip", "bot", "mixed"], default="burst", help="攻击模式")
    p.add_argument("--concurrency", type=int, default=20, help="并发数")
    p.add_argument("--requests", type=int, default=500, help="总请求数")
    p.add_argument("--interval", type=float, default=0.0, help="每次请求间隔(秒)，drip/bot/mixed可用")
    p.add_argument("--xff", action="store_true", help="随机X-Forwarded-For伪IP")
    p.add_argument("--suspicious-ua", action="store_true", help="使用可疑User-Agent")
    p.add_argument("--token", default=None, help="可选：Bearer Token(已登录用户)")
    p.add_argument("--endpoint", choices=["get", "view"], default="get",
                   help="post类型可选：get=GET /posts/{id}，view=POST /posts/{id}/view；package忽略")
    args = p.parse_args()
    return AttackConfig(
        base_url=args.base_url.rstrip("/"),
        target_type=args.type,
        target_id=args.id,
        mode=args.mode,
        concurrency=args.concurrency,
        total_requests=args.requests,
        interval=args.interval,
        use_xff=bool(args.xff),
        suspicious_ua=bool(args.suspicious_ua),
        token=args.token,
        endpoint=args.endpoint,
    )


def main():
    cfg = parse_args()
    asyncio.run(main_async(cfg))


if __name__ == "__main__":
    main()
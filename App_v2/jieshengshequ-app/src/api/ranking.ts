import { http } from './client';

// 排行榜查询参数
export interface RankingQuery {
  page?: number;
  page_size?: number;
  order_by?: string;
}

// 用户排行榜项目
export interface UserRankingItem {
  id: number;
  username: string;
  nickname?: string;
  avatar_url?: string;
  bio?: string;
  star: number;
  upload_count: number;
  download_count: number;
  created_at: string;
}

// 资源排行榜项目
export interface ResourceRankingItem {
  id: number;
  title: string;
  description?: string;
  downloads: number;
  likes: number;
  rating: number;
  category: string;
  created_at: string;
  tags: string[];
  author: {
    id: number;
    username: string;
    nickname?: string;
    avatar_url?: string;
  };
}

// 帖子排行榜项目
export interface PostRankingItem {
  id: number;
  title: string;
  content_preview?: string;
  views: number;
  likes: number;
  comments: number;
  created_at: string;
  tags: string[];
  author: {
    id: number;
    username: string;
    nickname?: string;
    avatar_url?: string;
  };
}

// 排行榜响应
export interface RankingResponse<T> {
  items: T[];
  total: number;
  page: number;
  page_size: number;
  has_more: boolean;
}

// 获取用户排行榜
export async function getUserRanking(params: RankingQuery = {}): Promise<RankingResponse<UserRankingItem>> {
  return http.get<RankingResponse<UserRankingItem>>('/ranking/users', params);
}

// 获取资源排行榜
export async function getResourceRanking(params: RankingQuery = {}): Promise<RankingResponse<ResourceRankingItem>> {
  return http.get<RankingResponse<ResourceRankingItem>>('/ranking/resources', params);
}

// 获取帖子排行榜
export async function getPostRanking(params: RankingQuery = {}): Promise<RankingResponse<PostRankingItem>> {
  return http.get<RankingResponse<PostRankingItem>>('/ranking/posts', params);
} 
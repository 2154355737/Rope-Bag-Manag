import { api } from '@/utils/apiClient';
import type { ApiResponse } from '@/types';

export interface Post {
  id: number;
  title: string;
  content: string;
  author_id: number;
  author_name?: string;
  category_id?: number;
  category_name?: string;
  status: 'Draft' | 'Published' | 'Archived' | 'Deleted';
  view_count: number;
  like_count: number;
  comment_count: number;
  is_pinned: boolean;
  is_featured: boolean;
  created_at: string;
  updated_at: string;
  // 前端展示需要的额外字段
  author_avatar?: string;
  category?: string;
  tags?: string[];
  excerpt?: string;
  cover?: string;
}

export interface CreatePostRequest {
  title: string;
  content: string;
  category_id?: number;
  tags?: string[];
  status?: 'Draft' | 'Published' | 'Archived' | 'Deleted';
}

export interface UpdatePostRequest {
  title?: string;
  content?: string;
  category_id?: number;
  tags?: string[];
  status?: 'Draft' | 'Published' | 'Archived' | 'Deleted';
  is_pinned?: boolean;
  is_featured?: boolean;
}

export interface PostQueryParams {
  page?: number;
  page_size?: number;
  category_id?: number;
  author_id?: number;
  status?: string;
  search?: string;
  tags?: string[];
  is_pinned?: boolean;
  is_featured?: boolean;
}

export interface PostListResponse {
  list: Post[];
  total: number;
  page: number;
  size: number;
}

export interface Tag {
  id: number;
  name: string;
  description?: string;
  color?: string;
  use_count: number;
  created_at: string;
  updated_at: string;
}

// 获取帖子列表
export const getPosts = (params?: PostQueryParams): Promise<ApiResponse<PostListResponse>> => {
  return api.get('/v1/posts', { params });
};

// 创建帖子
export const createPost = (data: CreatePostRequest): Promise<ApiResponse<{ post_id: number }>> => {
  return api.post('/v1/posts', data);
};

// 获取单个帖子
export const getPost = (id: number): Promise<ApiResponse<Post>> => {
  return api.get(`/v1/posts/${id}`);
};

// 更新帖子
export const updatePost = (id: number, data: UpdatePostRequest): Promise<ApiResponse<void>> => {
  return api.put(`/v1/posts/${id}`, data);
};

// 删除帖子
export const deletePost = (id: number): Promise<ApiResponse<void>> => {
  return api.delete(`/v1/posts/${id}`);
};

// 获取帖子标签
export const getPostTags = (id: number): Promise<ApiResponse<Tag[]>> => {
  return api.get(`/v1/posts/${id}/tags`);
};

// 增加浏览量
export const incrementViewCount = (id: number): Promise<ApiResponse<void>> => {
  return api.post(`/v1/posts/${id}/view`);
};

// 获取精选帖子
export const getFeaturedPosts = (params?: PostQueryParams): Promise<ApiResponse<PostListResponse>> => {
  return api.get('/v1/posts/featured', { params });
};

// 获取热门帖子
export const getPopularPosts = (params?: PostQueryParams): Promise<ApiResponse<PostListResponse>> => {
  return api.get('/v1/posts/popular', { params });
}; 
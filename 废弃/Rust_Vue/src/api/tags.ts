import { api } from '@/utils/apiClient';
import type { ApiResponse } from '@/types';

export interface Tag {
  id: number;
  name: string;
  description?: string;
  color?: string;
  use_count: number;
  created_at: string;
  updated_at: string;
}

export interface CreateTagRequest {
  name: string;
  description?: string;
  color?: string;
}

export interface UpdateTagRequest {
  name?: string;
  description?: string;
  color?: string;
}

export interface TagQueryParams {
  page?: number;
  page_size?: number;
  search?: string;
  sort_by?: 'name' | 'use_count' | 'created_at';
  sort_order?: 'asc' | 'desc';
}

export interface TagListResponse {
  list: Tag[];
  total: number;
  page: number;
  size: number;
}

// 获取标签列表
export const getTags = (params?: TagQueryParams): Promise<ApiResponse<TagListResponse>> => {
  return api.get('/v1/tags', { params });
};

// 创建标签
export const createTag = (data: CreateTagRequest): Promise<ApiResponse<{ tag_id: number }>> => {
  return api.post('/v1/tags', data);
};

// 获取单个标签
export const getTag = (id: number): Promise<ApiResponse<Tag>> => {
  return api.get(`/v1/tags/${id}`);
};

// 更新标签
export const updateTag = (id: number, data: UpdateTagRequest): Promise<ApiResponse<void>> => {
  return api.put(`/v1/tags/${id}`, data);
};

// 删除标签
export const deleteTag = (id: number): Promise<ApiResponse<void>> => {
  return api.delete(`/v1/tags/${id}`);
};

// 获取热门标签
export const getPopularTags = (): Promise<ApiResponse<Tag[]>> => {
  return api.get('/v1/tags/popular');
};

// 获取所有标签（用于下拉选择）
export const getAllTags = (): Promise<ApiResponse<Tag[]>> => {
  return api.get('/v1/tags/all');
}; 
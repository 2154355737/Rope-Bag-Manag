/**
 * @description: Login interface parameters
 */
export interface LoginParams {
  username: string
  password: string
}

export interface RoleInfo {
  roleName: string
  value: string
}

/**
 * @description: Login interface return value
 */
export interface LoginResultModel {
  userId: string | number
  token: string
  role: RoleInfo
  user?: {
    id: number
    username: string
    email: string
    nickname?: string
    role: string
    star: number
    ban_status: string
    created_at: string
    updated_at: string
  }
}

/**
 * @description: Get user information return value
 */
export interface GetUserInfoModel {
  roles: RoleInfo[]
  // 用户id
  userId: string | number
  // 用户名
  username: string
  // 真实名字
  realName: string
  // 头像
  avatar: string
  // 介绍
  desc?: string
  // 绳包管理器用户信息
  id?: number
  email?: string
  nickname?: string
  role?: string
  star?: number
  ban_status?: string
  created_at?: string
  updated_at?: string
}

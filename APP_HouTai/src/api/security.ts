import { defHttp } from '/@/utils/http/axios'

// 下载防刷
export const getDownloadSecurityStats = () => defHttp.get<any>({ url: '/api/v1/download-security/stats' })
export const getDownloadSecurityConfig = () => defHttp.get<any>({ url: '/api/v1/download-security/config' })
export const updateDownloadSecurityConfig = (data: any) => defHttp.put<any>({ url: '/api/v1/download-security/config', data })
// 旧：分页异常列表接口后端暂未提供，保留占位但不使用
export const getDownloadAnomalies = (params?: { page?: number; page_size?: number }) => defHttp.get<any>({ url: '/api/v1/download-security/anomalies', params })

// IP 封禁/白名单
export const getIpBans = (params?: { page?: number; page_size?: number; search?: string }) => defHttp.get<any>({ url: '/api/v1/security-management/ip-bans', params })
export const createIpBan = (data: { ip_address: string; reason?: string; duration_hours?: number }) => defHttp.post<any>({ url: '/api/v1/security-management/ip-bans', data })
export const deleteIpBan = (ip_address: string) => defHttp.delete<any>({ url: '/api/v1/security-management/ip-bans', data: { ip_address } })

export const getIpWhitelist = (params?: { page?: number; page_size?: number; search?: string }) => defHttp.get<any>({ url: '/api/v1/security-management/ip-whitelist', params })
export const createIpWhitelist = (data: { ip_address: string; description?: string }) => defHttp.post<any>({ url: '/api/v1/security-management/ip-whitelist', data })
export const deleteIpWhitelist = (ip_address: string) => defHttp.delete<any>({ url: '/api/v1/security-management/ip-whitelist', data: { ip_address } })

export const getBanStats = () => defHttp.get<any>({ url: '/api/v1/security-management/ban-stats' })
export const getSecurityConfig = () => defHttp.get<any>({ url: '/api/v1/security-management/security-config' })
export const updateSecurityConfig = (data: any) => defHttp.put<any>({ url: '/api/v1/security-management/security-config', data }) 
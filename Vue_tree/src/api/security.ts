import { defHttp } from '/@/utils/http/axios'

// 下载防刷
export const getDownloadSecurityStats = () => defHttp.get<any>({ url: '/api/v1/download-security/stats' })
export const getDownloadSecurityConfig = () => defHttp.get<any>({ url: '/api/v1/download-security/config' })
export const updateDownloadSecurityConfig = (data: any) => defHttp.post<any>({ url: '/api/v1/download-security/config', data })
export const getDownloadAnomalies = (params?: { page?: number; page_size?: number }) => defHttp.get<any>({ url: '/api/v1/download-security/anomalies', params })

// IP 封禁/白名单
export const getIpBans = (params?: { page?: number; page_size?: number; search?: string }) => defHttp.get<any>({ url: '/api/v1/security-management/ip-bans', params })
export const createIpBan = (data: { ip: string; reason?: string; expire_at?: string }) => defHttp.post<any>({ url: '/api/v1/security-management/ip-bans', data })
export const deleteIpBan = (id: number) => defHttp.delete<any>({ url: `/api/v1/security-management/ip-bans/${id}` })

export const getIpWhitelist = (params?: { page?: number; page_size?: number; search?: string }) => defHttp.get<any>({ url: '/api/v1/security-management/ip-whitelist', params })
export const createIpWhitelist = (data: { ip: string; note?: string }) => defHttp.post<any>({ url: '/api/v1/security-management/ip-whitelist', data })
export const deleteIpWhitelist = (id: number) => defHttp.delete<any>({ url: `/api/v1/security-management/ip-whitelist/${id}` })

export const getBanStats = () => defHttp.get<any>({ url: '/api/v1/security-management/ban-stats' })
export const getSecurityConfig = () => defHttp.get<any>({ url: '/api/v1/security-management/security-config' })
export const updateSecurityConfig = (data: any) => defHttp.post<any>({ url: '/api/v1/security-management/security-config', data }) 
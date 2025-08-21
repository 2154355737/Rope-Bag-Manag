import { resourceRecordApi } from '../api/resourceRecords'

/**
 * 全局资源操作记录服务
 * 用于在任何组件中方便记录资源操作
 */
class ResourceLogger {
  /**
   * 记录资源操作
   * @param resourceId 资源ID
   * @param action 操作类型：Create, Update, Delete, Download, Upload, StatusChange等
   * @param resourceType 可选，资源类型，默认为Package
   * @param oldData 可选，操作前的数据
   * @param newData 可选，操作后的数据
   */
  async logAction(resourceId: number, action: string, resourceType: string = 'Package', oldData?: any, newData?: any): Promise<void> {
    try {
      console.log(`记录资源操作: ID=${resourceId}, 操作=${action}, 类型=${resourceType}`)
      await resourceRecordApi.logResourceAction(resourceId, action, { 
        resource_type: resourceType,
        old_data: oldData,
        new_data: newData
      })
    } catch (error) {
      console.error('记录资源操作失败:', error)
    }
  }
  
  /**
   * 记录资源操作（更灵活的接口）
   * @param entityType 实体类型
   * @param operation 操作名称
   * @param entityId 实体ID
   * @param oldData 操作前的数据（可选）
   * @param newData 操作后的数据（可选）
   */
  async logResourceOperation(entityType: string, operation: string, entityId: number, oldData?: any, newData?: any): Promise<void> {
    try {
      // 跳过包创建操作的记录（已在后端记录）
      if (entityType === 'Package' && (operation.toLowerCase() === 'create' || operation.toLowerCase() === 'add')) {
        console.log(`[跳过记录] Package创建操作已在后端记录: ID=${entityId}`)
        return
      }
      
      console.log(`记录${entityType}操作: ID=${entityId}, 操作=${operation}`)
      // 转换为标准操作类型
      let action: string
      switch (operation.toLowerCase()) {
        case 'create':
        case 'add':
          action = 'Create'
          break
        case 'update':
        case 'modify':
        case 'edit':
          action = 'Update'
          break
        case 'delete':
        case 'remove':
          action = 'Delete'
          break
        case 'download':
          action = 'Download'
          break
        case 'upload':
          action = 'Upload'
          break
        case 'hide':
          action = 'Hide'
          break
        case 'show':
          action = 'Show'
          break
        default:
          action = operation
      }
      
      await resourceRecordApi.logResourceAction(
        entityId, 
        action,
        {
          resource_type: entityType,
          old_data: oldData,
          new_data: newData
        }
      )
    } catch (error) {
      console.error('记录资源操作失败:', error)
    }
  }
  
  /**
   * 记录资源创建
   */
  async logCreate(resourceId: number, resourceType: string = 'Package', data?: any): Promise<void> {
    // 跳过包创建操作的记录（已在后端记录）
    if (resourceType === 'Package') {
      console.log(`[跳过记录] Package创建操作已在后端记录: ID=${resourceId}`)
      return
    }
    return this.logAction(resourceId, 'Create', resourceType, null, data)
  }
  
  /**
   * 记录资源更新
   */
  async logUpdate(resourceId: number, resourceType: string = 'Package', oldData?: any, newData?: any): Promise<void> {
    return this.logAction(resourceId, 'Update', resourceType, oldData, newData)
  }
  
  /**
   * 记录资源删除
   */
  async logDelete(resourceId: number, resourceType: string = 'Package', data?: any): Promise<void> {
    return this.logAction(resourceId, 'Delete', resourceType, data)
  }
  
  /**
   * 记录资源下载
   */
  async logDownload(resourceId: number, resourceType: string = 'Package'): Promise<void> {
    return this.logAction(resourceId, 'Download', resourceType)
  }
  
  /**
   * 记录资源上传
   */
  async logUpload(resourceId: number, resourceType: string = 'Package', data?: any): Promise<void> {
    return this.logAction(resourceId, 'Upload', resourceType, null, data)
  }
  
  /**
   * 记录资源状态变更
   */
  async logStatusChange(resourceId: number, resourceType: string = 'Package', oldStatus?: string, newStatus?: string): Promise<void> {
    return this.logAction(resourceId, 'StatusChange', resourceType, oldStatus, newStatus)
  }
}

// 导出单例
export const resourceLogger = new ResourceLogger() 
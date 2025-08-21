import { getDB, exportDatabase } from './sqlite'

// 数据库导出工具
export const databaseExport = {
  // 导出数据库为文件
  exportToFile: async (filename: string = 'database.sqlite'): Promise<void> => {
    try {
      const db = getDB()
      const data = exportDatabase()
      
      // 创建Blob对象
      const blob = new Blob([data], { type: 'application/x-sqlite3' })
      
      // 创建下载链接
      const url = URL.createObjectURL(blob)
      const link = document.createElement('a')
      link.href = url
      link.download = filename
      
      // 触发下载
      document.body.appendChild(link)
      link.click()
      document.body.removeChild(link)
      
      // 清理URL
      URL.revokeObjectURL(url)
      
      console.log('数据库导出成功:', filename)
    } catch (error) {
      console.error('数据库导出失败:', error)
      throw new Error('数据库导出失败')
    }
  },

  // 导出数据库为JSON格式
  exportToJSON: async (): Promise<any> => {
    try {
      const db = getDB()
      
      // 获取所有表的数据
      const tables = ['user', 'resource', 'comment', 'action_log', 'resource_record', 'backup', 'announcement', 'log', 'settings', 'category']
      const data: any = {}
      
      for (const table of tables) {
        try {
          const result = db.exec(`SELECT * FROM ${table}`)
          data[table] = result[0] ? result[0].values : []
        } catch (error) {
          console.warn(`获取表 ${table} 数据失败:`, error)
          data[table] = []
        }
      }
      
      return {
        version: '1.0.0',
        export_time: new Date().toISOString(),
        data
      }
    } catch (error) {
      console.error('导出JSON失败:', error)
      throw new Error('导出JSON失败')
    }
  },

  // 导出JSON到文件
  exportJSONToFile: async (filename: string = 'database.json'): Promise<void> => {
    try {
      const jsonData = await databaseExport.exportToJSON()
      const jsonString = JSON.stringify(jsonData, null, 2)
      
      // 创建Blob对象
      const blob = new Blob([jsonString], { type: 'application/json' })
      
      // 创建下载链接
      const url = URL.createObjectURL(blob)
      const link = document.createElement('a')
      link.href = url
      link.download = filename
      
      // 触发下载
      document.body.appendChild(link)
      link.click()
      document.body.removeChild(link)
      
      // 清理URL
      URL.revokeObjectURL(url)
      
      console.log('JSON导出成功:', filename)
    } catch (error) {
      console.error('JSON导出失败:', error)
      throw new Error('JSON导出失败')
    }
  },

  // 从文件导入数据库
  importFromFile: async (file: File): Promise<void> => {
    try {
      const arrayBuffer = await file.arrayBuffer()
      const uint8Array = new Uint8Array(arrayBuffer)
      
      // 这里需要重新初始化数据库
      // 由于SQL.js的限制，需要重新创建数据库实例
      console.warn('导入数据库功能需要重新初始化SQL.js实例')
      
      // 暂时只支持JSON格式的导入
      if (file.name.endsWith('.json')) {
        const text = await file.text()
        const jsonData = JSON.parse(text)
        await databaseExport.importFromJSON(jsonData)
      } else {
        throw new Error('暂不支持SQLite文件导入，请使用JSON格式')
      }
    } catch (error) {
      console.error('导入数据库失败:', error)
      throw new Error('导入数据库失败')
    }
  },

  // 从JSON数据导入数据库
  importFromJSON: async (jsonData: any): Promise<void> => {
    try {
      const { initDB, execute } = await import('./sqlite')
      
      // 重新初始化数据库
      await initDB()
      
      // 导入数据
      for (const [tableName, tableData] of Object.entries(jsonData.data)) {
        if (Array.isArray(tableData) && tableData.length > 0) {
          // 清空表
          execute(`DELETE FROM ${tableName}`)
          
          // 插入数据
          for (const row of tableData) {
            const columns = Object.keys(row)
            const values = Object.values(row)
            const placeholders = values.map(() => '?').join(',')
            const sql = `INSERT INTO ${tableName} (${columns.join(',')}) VALUES (${placeholders})`
            execute(sql, values)
          }
        }
      }
      
      console.log('数据库导入成功')
    } catch (error) {
      console.error('导入JSON失败:', error)
      throw new Error('导入JSON失败')
    }
  },

  // 备份数据库
  createBackup: async (): Promise<string> => {
    try {
      const timestamp = new Date().toISOString().replace(/[:.]/g, '-')
      const filename = `backup-${timestamp}.sqlite`
      
      await databaseExport.exportToFile(filename)
      return filename
    } catch (error) {
      console.error('创建备份失败:', error)
      throw new Error('创建备份失败')
    }
  },

  // 获取数据库信息
  getDatabaseInfo: async (): Promise<any> => {
    try {
      const db = getDB()
      const tables = ['user', 'resource', 'comment', 'action_log', 'resource_record', 'backup', 'announcement', 'log', 'settings', 'category']
      const info: any = {
        tables: {}
      }
      
      for (const table of tables) {
        try {
          const result = db.exec(`SELECT COUNT(*) as count FROM ${table}`)
          const count = result[0] ? result[0].values[0][0] : 0
          info.tables[table] = count
        } catch (error) {
          console.warn(`获取表 ${table} 信息失败:`, error)
          info.tables[table] = 0
        }
      }
      
      // 获取数据库大小（估算）
      const data = exportDatabase()
      info.size_bytes = data.length
      info.size_mb = (data.length / 1024 / 1024).toFixed(2)
      
      return info
    } catch (error) {
      console.error('获取数据库信息失败:', error)
      throw new Error('获取数据库信息失败')
    }
  }
} 
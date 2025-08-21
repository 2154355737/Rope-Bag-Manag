import initSqlJs, { Database } from 'sql.js'

let db: any = null

export async function initDB() {
  if (db) return db
  const SQL = await initSqlJs({ locateFile: (file: string) => `https://sql.js.org/dist/${file}` })
  db = new SQL.Database()
  return db
}



export function getDB() {
  if (!db) throw new Error('数据库未初始化，请先调用 initDB()')
  return db
}

// 通用查询方法
export function query(sql: string, params: any[] = []): any[] {
  const database = getDB()
  const result = database.exec(sql, params)
  return result[0] ? result[0].values : []
}

// 通用执行方法（用于INSERT、UPDATE、DELETE）
export function execute(sql: string, params: any[] = []): number {
  const database = getDB()
  const result = database.exec(sql, params)
  return result[0] ? result[0].changes : 0
}

// 分页查询辅助方法
export function paginateQuery(baseSql: string, page: number = 1, pageSize: number = 10, params: any[] = []): { data: any[], total: number } {
  const database = getDB()
  
  // 获取总数
  const countSql = `SELECT COUNT(*) as total FROM (${baseSql}) as temp`
  const countResult = database.exec(countSql, params)
  const total = countResult[0] ? countResult[0].values[0][0] : 0
  
  // 获取分页数据
  const offset = (page - 1) * pageSize
  const dataSql = `${baseSql} LIMIT ${pageSize} OFFSET ${offset}`
  const dataResult = database.exec(dataSql, params)
  const data = dataResult[0] ? dataResult[0].values : []
  
  return { data, total }
}

// 用户相关方法
export function getAllUsers() {
  return query('SELECT * FROM user ORDER BY id DESC')
}

export function getUserById(id: number) {
  const result = query('SELECT * FROM user WHERE id = ?', [id])
  return result.length > 0 ? result[0] : null
}

export function getUserByUsername(username: string) {
  const result = query('SELECT * FROM user WHERE username = ?', [username])
  return result.length > 0 ? result[0] : null
}

export function createUser(userData: {
  username: string
  password: string
  role?: string
}) {
  const sql = `
    INSERT INTO user (username, password, role, status) 
    VALUES (?, ?, ?, ?)
  `
  return execute(sql, [
    userData.username,
    userData.password,
    userData.role || 'user',
    1 // status默认为1（启用）
  ])
}

export function updateUser(id: number, userData: {
  role?: string
  banned?: boolean
}) {
  const updates: string[] = []
  const params: any[] = []
  
  if (userData.role !== undefined) {
    updates.push('role = ?')
    params.push(userData.role)
  }
  if (userData.banned !== undefined) {
    updates.push('status = ?')
    params.push(userData.banned ? 0 : 1) // banned为true时status为0，否则为1
  }
  
  if (updates.length === 0) return 0
  
  params.push(id)
  const sql = `UPDATE user SET ${updates.join(', ')} WHERE id = ?`
  return execute(sql, params)
}



// 统计相关方法
export function getStats() {
  const userCount = query('SELECT COUNT(*) as count FROM user')[0]?.[0] || 0
  const activeUserCount = query('SELECT COUNT(*) as count FROM user WHERE status = 1')[0]?.[0] || 0
  
  return {
    total_users: userCount,
    total_packages: 0,
    total_comments: 0,
    active_users: activeUserCount,
    new_users_today: 0,
    new_packages_today: 0,
    system_status: 'running',
    uptime: Date.now()
  }
}

// 导出数据库
export function exportDatabase(): Uint8Array {
  const database = getDB()
  return database.export()
}

// 导入数据库
export function importDatabase(data: Uint8Array) {
  console.warn('导入数据库功能需要重新初始化SQL.js实例')
} 
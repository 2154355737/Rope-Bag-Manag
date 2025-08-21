/**
 * 格式化日期
 * @param dateString ISO格式的日期字符串或Date对象
 * @param format 格式化模式，默认为'YYYY-MM-DD'
 * @returns 格式化后的日期字符串
 */
export function formatDate(dateString: string | Date, format = 'YYYY-MM-DD'): string {
  if (!dateString) return '暂无日期';
  
  const date = dateString instanceof Date ? dateString : new Date(dateString);
  
  if (isNaN(date.getTime())) {
    return '无效日期';
  }
  
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');
  const hours = String(date.getHours()).padStart(2, '0');
  const minutes = String(date.getMinutes()).padStart(2, '0');
  const seconds = String(date.getSeconds()).padStart(2, '0');
  
  return format
    .replace('YYYY', String(year))
    .replace('MM', month)
    .replace('DD', day)
    .replace('HH', hours)
    .replace('mm', minutes)
    .replace('ss', seconds);
}

/**
 * 格式化文件大小
 * @param bytes 字节数
 * @returns 格式化后的文件大小字符串
 */
export function formatFileSize(bytes: number | null | undefined): string {
  if (bytes === null || bytes === undefined) return '未知大小';
  
  if (bytes === 0) return '0 B';
  
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  
  return (bytes / Math.pow(1024, i)).toFixed(2) + ' ' + units[i];
} 
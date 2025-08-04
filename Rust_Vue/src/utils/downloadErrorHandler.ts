import { ElMessage } from 'element-plus'

/**
 * 处理下载错误，显示具体的错误原因
 * @param error 错误对象
 * @param defaultMessage 默认错误消息
 */
export const handleDownloadError = (error: any, defaultMessage: string = '下载失败') => {
  console.error('下载失败:', error)
  
  // 根据错误类型显示不同的提示信息
  if (error.response?.status === 403) {
    const errorMessage = error.response?.data?.message || '下载被阻止'
    if (errorMessage.includes('下载被阻止')) {
      // 解析具体的阻止原因
      if (errorMessage.includes('用户下载频率超限')) {
        ElMessage.warning('下载被阻止：您的下载频率过高，请稍后再试')
      } else if (errorMessage.includes('IP下载频率超限')) {
        ElMessage.warning('下载被阻止：当前IP下载频率过高，请稍后再试')
      } else if (errorMessage.includes('资源下载频率超限')) {
        ElMessage.warning('下载被阻止：该资源下载频率过高，请稍后再试')
      } else if (errorMessage.includes('检测到严重异常行为')) {
        ElMessage.error('下载被阻止：检测到异常行为，请联系管理员')
      } else if (errorMessage.includes('全局下载频率超限')) {
        ElMessage.warning('下载被阻止：系统下载频率过高，请稍后再试')
      } else {
        ElMessage.warning('下载被阻止：' + errorMessage)
      }
    } else {
      ElMessage.warning('下载被阻止：' + errorMessage)
    }
  } else if (error.response?.status === 404) {
    ElMessage.error('资源不存在或已被删除')
  } else if (error.response?.status === 401) {
    ElMessage.warning('请先登录后再下载')
  } else if (error.response?.status === 429) {
    ElMessage.warning('请求过于频繁，请稍后再试')
  } else {
    ElMessage.error(defaultMessage + '：' + (error.message || '网络错误'))
  }
}

/**
 * 获取下载错误的具体原因
 * @param error 错误对象
 * @returns 错误原因字符串
 */
export const getDownloadErrorReason = (error: any): string => {
  if (error.response?.status === 403) {
    const errorMessage = error.response?.data?.message || '下载被阻止'
    if (errorMessage.includes('用户下载频率超限')) {
      return '您的下载频率过高，请稍后再试'
    } else if (errorMessage.includes('IP下载频率超限')) {
      return '当前IP下载频率过高，请稍后再试'
    } else if (errorMessage.includes('资源下载频率超限')) {
      return '该资源下载频率过高，请稍后再试'
    } else if (errorMessage.includes('检测到严重异常行为')) {
      return '检测到异常行为，请联系管理员'
    } else if (errorMessage.includes('全局下载频率超限')) {
      return '系统下载频率过高，请稍后再试'
    } else {
      return errorMessage
    }
  } else if (error.response?.status === 404) {
    return '资源不存在或已被删除'
  } else if (error.response?.status === 401) {
    return '请先登录后再下载'
  } else if (error.response?.status === 429) {
    return '请求过于频繁，请稍后再试'
  } else {
    return error.message || '网络错误'
  }
} 
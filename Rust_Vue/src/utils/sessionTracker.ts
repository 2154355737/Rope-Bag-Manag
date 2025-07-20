import userActionService from './userActionService'

/**
 * 会话追踪器 - 用于记录用户会话行为
 */
export const sessionTracker = {
  /**
   * 初始化会话追踪，挂载全局事件监听器
   */
  init() {
    this.trackSessionStartEnd()
    this.trackTabVisibility()
    this.trackIdleTime()
    
    console.log('[SessionTracker] 会话追踪器已初始化')
  },
  
  /**
   * 跟踪会话开始和结束
   */
  trackSessionStartEnd() {
    // 记录会话开始
    const recordSessionStart = () => {
      const userInfo = JSON.parse(localStorage.getItem('userInfo') || '{}')
      const username = userInfo?.username || '访客'
      
      userActionService.logAction(
        'SessionStart', 
        `用户${username}开始新会话`, 
        'Session', 
        undefined
      ).catch(err => console.error('记录会话开始失败:', err))
    }
    
    // 记录会话结束
    const recordSessionEnd = () => {
      const userInfo = JSON.parse(localStorage.getItem('userInfo') || '{}')
      const username = userInfo?.username || '访客'
      
      // 只有登录用户才记录会话结束
      if (username !== '访客') {
        userActionService.logAction(
          'SessionEnd', 
          `用户${username}结束会话`, 
          'Session', 
          undefined
        ).catch(err => console.error('记录会话结束失败:', err))
      }
    }
    
    // 页面加载时记录会话开始
    window.addEventListener('load', recordSessionStart)
    
    // 页面关闭时记录会话结束
    window.addEventListener('beforeunload', recordSessionEnd)
  },
  
  /**
   * 跟踪标签页可见性
   */
  trackTabVisibility() {
    let hidden: string, visibilityChange: string
    
    // 不同浏览器的属性兼容
    if (typeof document.hidden !== 'undefined') {
      hidden = 'hidden'
      visibilityChange = 'visibilitychange'
    } else if (typeof (document as any).msHidden !== 'undefined') {
      hidden = 'msHidden'
      visibilityChange = 'msvisibilitychange'
    } else if (typeof (document as any).webkitHidden !== 'undefined') {
      hidden = 'webkitHidden'
      visibilityChange = 'webkitvisibilitychange'
    }
    
    // 标签页可见性变化处理
    const handleVisibilityChange = () => {
      const userInfo = JSON.parse(localStorage.getItem('userInfo') || '{}')
      const username = userInfo?.username || '访客'
      
      // 只记录已登录用户
      if (username !== '访客') {
        if (document[hidden as any]) {
          // 标签页不可见
          userActionService.logAction(
            'TabHidden', 
            `用户${username}切换离开页面`, 
            'Session', 
            undefined
          ).catch(err => console.error('记录标签页隐藏失败:', err))
        } else {
          // 标签页可见
          userActionService.logAction(
            'TabVisible', 
            `用户${username}切换回页面`, 
            'Session', 
            undefined
          ).catch(err => console.error('记录标签页可见失败:', err))
        }
      }
    }
    
    document.addEventListener(visibilityChange, handleVisibilityChange, false)
  },
  
  /**
   * 跟踪用户闲置时间
   */
  trackIdleTime() {
    let idleTime = 0
    let idleInterval: number | null = null
    let isIdle = false
    const idleThreshold = 5 // 5分钟无操作视为闲置
    
    // 重置闲置计时器
    const resetIdleTime = () => {
      idleTime = 0
      
      // 如果之前处于闲置状态，记录用户活动恢复
      if (isIdle) {
        isIdle = false
        const userInfo = JSON.parse(localStorage.getItem('userInfo') || '{}')
        const username = userInfo?.username || '访客'
        
        if (username !== '访客') {
          userActionService.logAction(
            'UserActive', 
            `用户${username}恢复活动`, 
            'Session', 
            undefined
          ).catch(err => console.error('记录用户活动恢复失败:', err))
        }
      }
    }
    
    // 开始闲置检测
    const startIdleTracking = () => {
      idleInterval = window.setInterval(() => {
        idleTime += 1
        
        // 超过闲置阈值
        if (idleTime >= idleThreshold && !isIdle) {
          isIdle = true
          const userInfo = JSON.parse(localStorage.getItem('userInfo') || '{}')
          const username = userInfo?.username || '访客'
          
          if (username !== '访客') {
            userActionService.logAction(
              'UserIdle', 
              `用户${username}闲置${idleThreshold}分钟`, 
              'Session', 
              undefined
            ).catch(err => console.error('记录用户闲置失败:', err))
          }
        }
      }, 60000) // 每分钟检查一次
    }
    
    // 监听用户活动事件
    const events = ['mousedown', 'mousemove', 'keypress', 'scroll', 'touchstart']
    events.forEach(event => {
      document.addEventListener(event, resetIdleTime, true)
    })
    
    // 开始追踪
    startIdleTracking()
  }
}

export default sessionTracker 
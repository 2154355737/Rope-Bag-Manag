import { App } from '@capacitor/app'
import { Capacitor } from '@capacitor/core'

export interface BackButtonHandler {
  priority: number
  handler: () => boolean | Promise<boolean>
}

class BackButtonManager {
  private handlers: BackButtonHandler[] = []
  private isInitialized = false

  /**
   * 初始化返回键监听器
   */
  initialize() {
    if (this.isInitialized || !Capacitor.isNativePlatform()) {
      return
    }

    App.addListener('backButton', async (event) => {
      console.log('硬件返回键被按下', event)
      
      // 按优先级排序处理器（数字越大优先级越高）
      const sortedHandlers = [...this.handlers].sort((a, b) => b.priority - a.priority)
      
      // 依次执行处理器，直到有一个返回 true（表示已处理）
      for (const handlerObj of sortedHandlers) {
        try {
          const handled = await handlerObj.handler()
          if (handled) {
            console.log('返回键事件已被处理')
            return
          }
        } catch (error) {
          console.error('返回键处理器执行错误:', error)
        }
      }
      
      // 如果没有处理器处理该事件，则执行默认行为（退出应用）
      console.log('没有处理器处理返回键事件，执行默认行为')
      App.exitApp()
    })

    this.isInitialized = true
    console.log('Android返回键监听器已初始化')
  }

  /**
   * 注册返回键处理器
   * @param handler 处理器函数，返回 true 表示已处理该事件
   * @param priority 优先级，数字越大优先级越高
   */
  addHandler(handler: () => boolean | Promise<boolean>, priority: number = 0): () => void {
    const handlerObj: BackButtonHandler = { handler, priority }
    this.handlers.push(handlerObj)
    
    console.log(`注册返回键处理器，优先级: ${priority}`)
    
    // 返回移除函数
    return () => {
      const index = this.handlers.indexOf(handlerObj)
      if (index > -1) {
        this.handlers.splice(index, 1)
        console.log(`移除返回键处理器，优先级: ${priority}`)
      }
    }
  }

  /**
   * 移除所有处理器
   */
  clearHandlers() {
    this.handlers = []
    console.log('清除所有返回键处理器')
  }

  /**
   * 获取当前处理器数量
   */
  getHandlerCount(): number {
    return this.handlers.length
  }
}

// 创建全局实例
export const backButtonManager = new BackButtonManager()

/**
 * 初始化返回键管理器
 */
export const initializeBackButton = () => {
  backButtonManager.initialize()
}

/**
 * 添加返回键处理器的便捷函数
 */
export const addBackButtonHandler = (
  handler: () => boolean | Promise<boolean>, 
  priority: number = 0
): (() => void) => {
  return backButtonManager.addHandler(handler, priority)
} 
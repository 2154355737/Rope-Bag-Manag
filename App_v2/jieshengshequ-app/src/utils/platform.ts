import { Capacitor } from '@capacitor/core'
import { Keyboard } from '@capacitor/keyboard'

export const getPlatform = () => {
  return Capacitor.getPlatform()
}

export const isNative = () => {
  return Capacitor.isNativePlatform()
}

export const isAndroid = () => {
  return Capacitor.getPlatform() === 'android'
}

export const isIOS = () => {
  return Capacitor.getPlatform() === 'ios'
}

export const isWeb = () => {
  return Capacitor.getPlatform() === 'web'
}

// 添加平台类名到body
export const addPlatformClass = () => {
  const platform = getPlatform()
  document.body.classList.add(`platform-${platform}`)
  
  if (isNative()) {
    document.body.classList.add('platform-native')
  } else {
    document.body.classList.add('platform-web')
  }
}

// 键盘事件处理
export const initializeKeyboard = () => {
  console.log('🔧 初始化键盘处理...')
  console.log('平台信息:', {
    platform: getPlatform(),
    isNative: isNative(),
    isAndroid: isAndroid(),
    isIOS: isIOS(),
    isWeb: isWeb()
  })
  
  if (!isNative()) {
    console.log('⚠️ 非原生平台，跳过键盘事件监听')
    console.log('💡 Web平台提示：可以在控制台手动调用 testWebKeyboard() 来模拟键盘事件')
    return
  }

  console.log('✅ 原生平台，开始设置键盘事件监听器...')
  let activeInput: HTMLElement | null = null
  let keyboardHeight = 0

  // 监听输入框获得焦点
  document.addEventListener('focusin', (event) => {
    const target = event.target as HTMLElement
    if (target && (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA')) {
      activeInput = target
      console.log('输入框获得焦点:', target)
    }
  })

  // 监听输入框失去焦点
  document.addEventListener('focusout', () => {
    activeInput = null
    console.log('输入框失去焦点')
  })

  // 键盘即将显示
  Keyboard.addListener('keyboardWillShow', (info) => {
    keyboardHeight = info.keyboardHeight
    document.body.classList.add('keyboard-open')
    document.documentElement.style.setProperty('--keyboard-height', `${keyboardHeight}px`)
    
    // 强制更新主内容区域的滚动属性
    const mainContent = document.querySelector('.main-content') as HTMLElement
    if (mainContent) {
      // 确保主内容区域可以滚动
      mainContent.style.overflowY = 'auto'
      mainContent.style.height = `calc(100vh - ${keyboardHeight}px)`
      mainContent.style.maxHeight = `calc(100vh - ${keyboardHeight}px)`
      console.log('已设置主内容区域滚动属性')
    }
    
    console.log('键盘即将显示，高度:', keyboardHeight)
  })

  // 键盘已显示
  Keyboard.addListener('keyboardDidShow', (info) => {
    keyboardHeight = info.keyboardHeight
    console.log('键盘已显示，高度:', keyboardHeight)
    
    // 调试信息
    setTimeout(() => {
      debugKeyboardStatus()
    }, 100)
    
    // 现代化键盘系统已接管滚动处理，旧版滚动逻辑已禁用
    console.log('📱 现代化键盘系统已接管滚动处理，跳过旧版滚动逻辑')
    
    // 仅保留调试信息输出
    setTimeout(() => {
      console.log('=== 滚动后状态 ===')
      debugKeyboardStatus()
    }, 500)
  })

  // 键盘即将隐藏
  Keyboard.addListener('keyboardWillHide', () => {
    document.body.classList.remove('keyboard-open')
    document.documentElement.style.setProperty('--keyboard-height', '0px')
    
    // 恢复主内容区域的默认样式
    const mainContent = document.querySelector('.main-content') as HTMLElement
    if (mainContent) {
      mainContent.style.overflowY = ''
      mainContent.style.height = ''
      mainContent.style.maxHeight = ''
      console.log('已恢复主内容区域默认样式')
    }
    
    keyboardHeight = 0
    console.log('键盘即将隐藏')
  })

  // 键盘已隐藏
  Keyboard.addListener('keyboardDidHide', () => {
    console.log('键盘已隐藏')
  })
}

// 滚动到活动输入框
const scrollToActiveInput = (input: HTMLElement, keyboardHeight: number) => {
  try {
    const inputRect = input.getBoundingClientRect()
    
    // 使用修正后的视口高度
    let viewportHeight = window.innerHeight
    if (viewportHeight > 10000 && window.screen?.height) {
      viewportHeight = window.screen.height
    }
    if (viewportHeight > 3000 || viewportHeight < 400) {
      viewportHeight = 800
    }
    
    // 键盘顶部的位置（从底部算起）
    const keyboardTop = viewportHeight - keyboardHeight
    
    // 输入框底部位置，包含缓冲区
    const bufferSpace = 20 // 输入框底部与键盘顶部的缓冲距离
    const targetPosition = keyboardTop - bufferSpace // 目标位置：键盘顶部上方20px
    
    // 查找可滚动的父容器
    const scrollContainer = findScrollableParent(input) || window
    const isWindowScroll = scrollContainer === window
    
    console.log('=== 滚动计算开始 ===')
    console.log('键盘高度:', keyboardHeight)
    console.log('视口高度:', `${viewportHeight}px (修正后，原始: ${window.innerHeight}px)`)
    console.log('键盘顶部位置:', keyboardTop)
    console.log('目标位置(键盘顶部-缓冲):', targetPosition)
    console.log('输入框位置:', { 
      top: inputRect.top, 
      bottom: inputRect.bottom,
      height: inputRect.height 
    })
    console.log('滚动容器:', isWindowScroll ? 'window' : 'element', 
      isWindowScroll ? '' : (scrollContainer as Element)?.tagName)
    
    if (!isWindowScroll) {
      const containerElement = scrollContainer as Element
      console.log('容器滚动信息:', {
        scrollTop: containerElement.scrollTop,
        scrollHeight: containerElement.scrollHeight,
        clientHeight: containerElement.clientHeight
      })
    } else {
      console.log('窗口滚动信息:', {
        scrollY: window.scrollY,
        scrollHeight: document.documentElement.scrollHeight,
        clientHeight: window.innerHeight
      })
    }
    
    // 检查输入框底部是否超过了目标位置
    if (inputRect.bottom > targetPosition) {
      // 计算需要滚动的距离：让输入框底部定位到目标位置
      const scrollAmount = inputRect.bottom - targetPosition
      
      console.log('需要滚动距离:', scrollAmount)
      console.log('预期滚动后输入框底部位置:', inputRect.bottom - scrollAmount)
      
      // 执行滚动
      if (isWindowScroll) {
        const beforeScrollY = window.scrollY
        window.scrollBy({
          top: scrollAmount,
          behavior: 'smooth'
        })
        console.log('窗口滚动:', beforeScrollY, '->', beforeScrollY + scrollAmount)
      } else {
        const container = scrollContainer as Element
        const beforeScrollTop = container.scrollTop
        container.scrollTo({
          top: container.scrollTop + scrollAmount,
          behavior: 'smooth'
        })
        console.log('容器滚动:', beforeScrollTop, '->', beforeScrollTop + scrollAmount)
      }
      
      // 简化的验证，避免与现代化系统冲突
      setTimeout(() => {
        const newInputRect = input.getBoundingClientRect()
        console.log('=== 滚动后验证 ===')
        console.log('新的输入框位置:', { 
          top: newInputRect.top, 
          bottom: newInputRect.bottom 
        })
        console.log('目标位置:', targetPosition)
        const isSuccess = newInputRect.bottom <= targetPosition + 10 // 10px容差
        console.log('是否达到目标:', isSuccess ? '✅ 成功' : '❌ 失败')
        console.log('实际偏差:', newInputRect.bottom - targetPosition)
      }, 500)
      
    } else {
      console.log('输入框底部未被遮挡，无需滚动')
    }
    console.log('=== 滚动计算结束 ===')
  } catch (error) {
    console.error('滚动到输入框时出错:', error)
  }
}

// 查找可滚动的父容器
const findScrollableParent = (element: HTMLElement): Element | null => {
  let parent = element.parentElement
  
  while (parent) {
    const style = window.getComputedStyle(parent)
    const overflow = style.overflow + style.overflowY + style.overflowX
    
    // 优先查找 main-content 类的容器
    if (parent.classList.contains('main-content')) {
      console.log('找到 main-content 滚动容器')
      return parent
    }
    
    // 检查是否有滚动属性
    if (overflow.includes('scroll') || overflow.includes('auto')) {
      console.log('找到滚动容器:', parent.tagName, parent.className)
      return parent
    }
    
    parent = parent.parentElement
  }
  
  console.log('未找到滚动容器，使用window')
  return null
}

// 获取设备信息
export const getDeviceInfo = async () => {
  if (isNative()) {
    try {
      const { Device } = await import('@capacitor/device')
      return await Device.getInfo()
    } catch (error) {
      console.error('获取设备信息失败:', error)
      return null
    }
  }
  return null
}

// 手动显示键盘
export const showKeyboard = async () => {
  if (isNative()) {
    try {
      await Keyboard.show()
    } catch (error) {
      console.error('显示键盘失败:', error)
    }
  }
}

// 手动隐藏键盘
export const hideKeyboard = async () => {
  if (isNative()) {
    try {
      await Keyboard.hide()
    } catch (error) {
      console.error('隐藏键盘失败:', error)
    }
  }
}

// 调试函数：输出当前键盘和输入框状态
export const debugKeyboardStatus = () => {
  const keyboardHeight = document.documentElement.style.getPropertyValue('--keyboard-height') || '0px'
  const isKeyboardOpen = document.body.classList.contains('keyboard-open')
  const activeElement = document.activeElement
  
  console.log('=== 键盘调试信息 ===')
  console.log('键盘高度 CSS 变量:', keyboardHeight)
  console.log('键盘开启状态:', isKeyboardOpen)
  console.log('当前焦点元素:', activeElement?.tagName, activeElement?.id || activeElement?.className)
  
  if (activeElement && (activeElement.tagName === 'INPUT' || activeElement.tagName === 'TEXTAREA')) {
    const rect = activeElement.getBoundingClientRect()
    const numericKeyboardHeight = parseInt(keyboardHeight.replace('px', '')) || 0
    
    // 使用修正后的视口高度
    let viewportHeight = window.innerHeight
    if (viewportHeight > 10000 && window.screen?.height) {
      viewportHeight = window.screen.height
    }
    if (viewportHeight > 3000 || viewportHeight < 400) {
      viewportHeight = 800
    }
    
    const keyboardTop = viewportHeight - numericKeyboardHeight
    const bufferSpace = 20
    const targetPosition = keyboardTop - bufferSpace
    const isInputBlocked = rect.bottom > targetPosition
    
    console.log('输入框位置:', {
      top: rect.top,
      bottom: rect.bottom,
      left: rect.left,
      right: rect.right,
      width: rect.width,
      height: rect.height
    })
    console.log('定位信息:', {
      originalInnerHeight: window.innerHeight,
      correctedViewportHeight: viewportHeight,
      keyboardHeight: numericKeyboardHeight,
      keyboardTop,
      targetPosition: targetPosition,
      bufferSpace,
      inputBottom: rect.bottom,
      isInputBlocked,
      needScrollDistance: isInputBlocked ? rect.bottom - targetPosition : 0
    })
    console.log('🎯 定位逻辑: 输入框底部(' + rect.bottom + ') -> 键盘顶部上方(' + targetPosition + ')')
  }
  console.log('==================')
} 

// 备选滚动策略：更直接的方法
const alternativeScrollToInput = (input: HTMLElement, keyboardHeight: number) => {
  try {
    console.log('=== 使用备选滚动策略 ===')
    
    // 先使用 scrollIntoView 让输入框可见
    input.scrollIntoView({
      behavior: 'smooth',
      block: 'center',
      inline: 'nearest'
    })
    
    // 然后根据键盘高度进行微调
    setTimeout(() => {
      const rect = input.getBoundingClientRect()
      const viewportHeight = window.innerHeight
      const keyboardTop = viewportHeight - keyboardHeight
      const targetBottom = keyboardTop - 30 // 30px缓冲
      
      if (rect.bottom > targetBottom) {
        const additionalScroll = rect.bottom - targetBottom
        window.scrollBy({
          top: additionalScroll,
          behavior: 'smooth'
        })
        console.log('备选策略额外滚动:', additionalScroll)
      }
    }, 200)
    
  } catch (error) {
    console.error('备选滚动策略失败:', error)
  }
}

// 导出备选策略供测试使用
export const testAlternativeScroll = (keyboardHeight: number) => {
  const activeElement = document.activeElement
  if (activeElement && (activeElement.tagName === 'INPUT' || activeElement.tagName === 'TEXTAREA')) {
    alternativeScrollToInput(activeElement as HTMLElement, keyboardHeight)
  }
} 

// Web平台键盘测试函数
export const testWebKeyboard = (mockKeyboardHeight = 350) => {
  console.log('🧪 Web平台键盘测试开始...')
  console.log('模拟键盘高度:', mockKeyboardHeight)
  
  // 模拟键盘显示
  document.body.classList.add('keyboard-open')
  document.documentElement.style.setProperty('--keyboard-height', `${mockKeyboardHeight}px`)
  
  // 强制更新主内容区域的滚动属性
  const mainContent = document.querySelector('.main-content') as HTMLElement
  if (mainContent) {
    mainContent.style.overflowY = 'auto'
    mainContent.style.height = `calc(100vh - ${mockKeyboardHeight}px)`
    mainContent.style.maxHeight = `calc(100vh - ${mockKeyboardHeight}px)`
    console.log('✅ 已设置主内容区域滚动属性')
    console.log('主内容区域信息:', {
      scrollHeight: mainContent.scrollHeight,
      clientHeight: mainContent.clientHeight,
      offsetHeight: mainContent.offsetHeight
    })
  }
  
  // 查找当前焦点元素
  const activeElement = document.activeElement
  if (activeElement && (activeElement.tagName === 'INPUT' || activeElement.tagName === 'TEXTAREA')) {
    console.log('找到活动输入框:', activeElement.tagName, activeElement.id || activeElement.className)
    
    // 现代化键盘系统已接管滚动处理，测试函数不再调用旧版滚动逻辑
    console.log('📱 现代化键盘系统已接管滚动处理，测试函数跳过旧版滚动逻辑')
  } else {
    console.log('❌ 没有找到活动的输入框，请先点击一个输入框然后再调用此函数')
  }
}

// 清除Web平台键盘测试状态
export const clearWebKeyboardTest = () => {
  console.log('🧹 清除Web键盘测试状态...')
  document.body.classList.remove('keyboard-open')
  document.documentElement.style.setProperty('--keyboard-height', '0px')
}

// 全局暴露测试函数（仅在开发环境）
if (typeof window !== 'undefined' && import.meta.env.DEV) {
  const globalWindow = window as any
  globalWindow.testWebKeyboard = testWebKeyboard
  globalWindow.clearWebKeyboardTest = clearWebKeyboardTest
  globalWindow.debugKeyboardStatus = debugKeyboardStatus
  console.log('🔧 开发模式：已暴露键盘调试函数到 window 对象')
  console.log('可用函数: testWebKeyboard(), clearWebKeyboardTest(), debugKeyboardStatus()')
} 
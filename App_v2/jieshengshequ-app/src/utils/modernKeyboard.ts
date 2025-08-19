/**
 * 现代化键盘处理系统
 * 使用最新的Web API和设计模式
 */

import { Capacitor } from '@capacitor/core'
import { Keyboard } from '@capacitor/keyboard'

interface KeyboardState {
  isOpen: boolean
  height: number
  animationDuration: number
}

interface ViewportInfo {
  height: number
  visualHeight: number
  keyboardHeight: number
}

class ModernKeyboardManager {
  private state: KeyboardState = {
    isOpen: false,
    height: 0,
    animationDuration: 300
  }

  private observers: Set<IntersectionObserver> = new Set()
  private resizeObserver?: ResizeObserver
  private focusedElement: HTMLElement | null = null
  private callbacks: Set<(state: KeyboardState) => void> = new Set()

  constructor() {
    this.initialize()
  }

  private initialize() {
    console.log('🚀 初始化现代化键盘管理器...')
    
    if (Capacitor.isNativePlatform()) {
      this.initializeNativeKeyboard()
    } else {
      this.initializeWebKeyboard()
    }

    this.setupViewportObserver()
    this.setupFocusTracking()
    this.setupCSSEnvironment()
  }

  /**
   * 原生平台键盘处理
   */
  private initializeNativeKeyboard() {
    console.log('📱 设置原生键盘监听器...')

    Keyboard.addListener('keyboardWillShow', (info) => {
      this.updateState({
        isOpen: true,
        height: info.keyboardHeight,
        animationDuration: 300
      })
    })

    Keyboard.addListener('keyboardDidShow', (info) => {
      this.handleKeyboardShow(info.keyboardHeight)
    })

    Keyboard.addListener('keyboardWillHide', () => {
      this.updateState({
        isOpen: false,
        height: 0,
        animationDuration: 300
      })
    })

    Keyboard.addListener('keyboardDidHide', () => {
      this.handleKeyboardHide()
    })
  }

  /**
   * Web平台键盘处理 - 使用现代化方法
   */
  private initializeWebKeyboard() {
    console.log('🌐 设置Web键盘检测...')

    // 使用 Visual Viewport API
    if ('visualViewport' in window) {
      const visualViewport = window.visualViewport!
      
      visualViewport.addEventListener('resize', () => {
        this.handleViewportChange()
      })
    }

    // 备选方案：监听窗口大小变化
    window.addEventListener('resize', () => {
      this.handleViewportChange()
    })
  }

  /**
   * 设置视口观察器
   */
  private setupViewportObserver() {
    if ('ResizeObserver' in window) {
      this.resizeObserver = new ResizeObserver((entries) => {
        for (const entry of entries) {
          if (entry.target === document.documentElement) {
            this.handleViewportChange()
          }
        }
      })
      
      this.resizeObserver.observe(document.documentElement)
    }
  }

  /**
   * 设置焦点跟踪
   */
  private setupFocusTracking() {
    document.addEventListener('focusin', (event) => {
      const target = event.target as HTMLElement
      if (this.isInputElement(target)) {
        this.focusedElement = target
        this.trackElementVisibility(target)
        console.log('📝 输入元素获得焦点:', target.tagName, target.id)
      }
    })

    document.addEventListener('focusout', () => {
      if (this.focusedElement) {
        this.stopTrackingElement(this.focusedElement)
        this.focusedElement = null
        console.log('📝 输入元素失去焦点')
      }
    })
  }

  /**
   * 设置CSS环境变量
   */
  private setupCSSEnvironment() {
    // 设置现代CSS环境变量
    document.documentElement.style.setProperty('--keyboard-height', '0px')
    document.documentElement.style.setProperty('--viewport-height', '100vh')
    document.documentElement.style.setProperty('--safe-keyboard-height', '0px')
  }

  /**
   * 处理视口变化
   */
  private handleViewportChange() {
    // 在原生平台上，主要依赖Capacitor的键盘事件
    // 这个方法主要用于Web平台的键盘检测
    if (Capacitor.isNativePlatform()) {
      console.log('🔍 原生平台：跳过视口变化处理，依赖Capacitor键盘事件')
      return
    }

    const viewportInfo = this.getViewportInfo()
    const keyboardHeight = viewportInfo.keyboardHeight

    if (keyboardHeight > 50) { // 假设键盘至少50px高
      if (!this.state.isOpen) {
        this.updateState({
          isOpen: true,
          height: keyboardHeight,
          animationDuration: 300
        })
        this.handleKeyboardShow(keyboardHeight)
      }
    } else {
      if (this.state.isOpen) {
        this.updateState({
          isOpen: false,
          height: 0,
          animationDuration: 300
        })
        this.handleKeyboardHide()
      }
    }
  }

  /**
   * 获取视口信息
   */
  private getViewportInfo(): ViewportInfo {
    // 在原生应用中，使用 screen.height 作为基准可能更准确
    const screenHeight = window.screen?.height || window.innerHeight
    const innerHeight = window.innerHeight
    const visualHeight = window.visualViewport?.height || innerHeight
    
    // 选择更合理的高度值
    const height = Math.min(screenHeight, innerHeight, 3000) // 限制最大值为3000px
    const keyboardHeight = Math.max(0, height - visualHeight)

    console.log('🔍 视口信息调试:', {
      screenHeight,
      innerHeight,
      visualHeight,
      selectedHeight: height,
      keyboardHeight
    })

    return {
      height,
      visualHeight,
      keyboardHeight
    }
  }

  /**
   * 处理键盘显示
   */
  private handleKeyboardShow(keyboardHeight: number) {
    console.log('⌨️ 键盘显示，高度:', keyboardHeight)

    // 更新CSS变量
    this.updateCSSVariables(keyboardHeight)

    // 添加键盘打开类
    document.body.classList.add('modern-keyboard-open')
    
    // 添加导航栏上移动画类
    document.body.classList.add('keyboard-animating-up')
    document.body.classList.remove('keyboard-animating-down')

    // 强制刷新滚动容器
    this.refreshScrollContainers()

    // 优化内容区域，移除多余空白 - 多次调用确保彻底优化
    setTimeout(() => {
      this.optimizeContentArea()
    }, 100)
    
    // 再次优化，确保DOM更新后的最终状态
    setTimeout(() => {
      this.optimizeContentArea()
    }, 300)

    // 处理当前焦点元素
    if (this.focusedElement) {
      // 延迟确保元素可见，让CSS变量和布局先更新
      setTimeout(() => {
        this.ensureElementVisible(this.focusedElement!, keyboardHeight)
      }, 150) // 延迟稍微增加，确保优化完成
    }

    // 通知回调
    this.notifyCallbacks()
  }

  /**
   * 处理键盘隐藏
   */
  private handleKeyboardHide() {
    console.log('⌨️ 键盘隐藏')

    // 更新CSS变量
    this.updateCSSVariables(0)

    // 移除键盘打开类
    document.body.classList.remove('modern-keyboard-open')
    
    // 添加导航栏下移动画类
    document.body.classList.add('keyboard-animating-down')
    document.body.classList.remove('keyboard-animating-up')
    
    // 动画完成后清理动画类
    setTimeout(() => {
      document.body.classList.remove('keyboard-animating-down')
      document.body.classList.remove('keyboard-animating-up')
    }, 300)

    // 刷新滚动容器，恢复默认高度
    this.refreshScrollContainers()

    // 通知回调
    this.notifyCallbacks()
  }

  /**
   * 刷新滚动容器
   */
  private refreshScrollContainers() {
    // 查找所有滚动容器并强制重新计算
    const scrollContainers = document.querySelectorAll('.modern-main-content, .scroll-container, .main-content')
    
    // 获取修正后的视口高度
    let viewportHeight = window.innerHeight
    if (viewportHeight > 10000 && window.screen?.height) {
      viewportHeight = window.screen.height
    }
    if (viewportHeight > 3000 || viewportHeight < 400) {
      viewportHeight = 800
    }
    
    const keyboardHeight = this.state.height
    const availableHeight = viewportHeight - keyboardHeight
    
    scrollContainers.forEach(container => {
      const element = container as HTMLElement
      
      // 强制设置正确的高度，不依赖CSS变量
      if (this.state.isOpen) {
        element.style.height = `${availableHeight}px`
        element.style.maxHeight = `${availableHeight}px`
        element.style.overflowY = 'auto'
        element.style.paddingBottom = '0px' // 移除额外的padding避免空白
        element.style.boxSizing = 'border-box' // 确保边框盒模型
        
        // 🔥 关键：强制限制直接子元素的高度，防止子元素撑开滚动区域
        const directChildren = Array.from(element.children) as HTMLElement[]
        directChildren.forEach(child => {
          const childMaxHeight = availableHeight - 10 // 留10px缓冲
          child.style.maxHeight = `${childMaxHeight}px`
          child.style.height = `${childMaxHeight}px` // 强制固定高度
          child.style.minHeight = 'unset' // 移除min-height
          child.style.overflowY = 'auto'
          console.log('🔧 强制限制直接子元素高度:', {
            childClass: child.className,
            强制高度: `${childMaxHeight}px`,
            note: '移除min-h-screen等异常CSS'
          })
        })
        
        console.log('🔄 强制设置滚动容器高度:', {
          container: element.className,
          height: `${availableHeight}px`,
          viewportHeight,
          keyboardHeight,
          scrollHeight: element.scrollHeight,
          clientHeight: element.clientHeight,
          note: '已移除padding-bottom避免空白区域'
        })
      } else {
        // 键盘隐藏时恢复默认
        element.style.height = ''
        element.style.maxHeight = ''
        element.style.overflowY = ''
        element.style.paddingBottom = ''
        element.style.boxSizing = ''
        element.style.minHeight = ''
        
        // 🔥 关键修复：同时恢复所有子元素的样式
        const directChildren = Array.from(element.children) as HTMLElement[]
        directChildren.forEach(child => {
          child.style.height = '' // 恢复高度
          child.style.maxHeight = '' // 恢复最大高度
          child.style.minHeight = '' // 恢复最小高度
          child.style.overflowY = '' // 恢复滚动
          
          console.log('🔄 恢复子元素样式:', {
            childClass: child.className,
            note: '恢复到原始CSS状态'
          })
        })
        
        console.log('🔄 恢复滚动容器和子元素默认状态:', element.className)
      }
    })
  }

  /**
   * 强力优化内容区域，彻底移除空白区域
   */
  private optimizeContentArea() {
    const scrollContainers = document.querySelectorAll('.modern-main-content, .scroll-container, .main-content')
    
    scrollContainers.forEach(container => {
      const element = container as HTMLElement
      
      if (this.state.isOpen) {
        // 获取详细的尺寸信息
        const rect = element.getBoundingClientRect()
        const computedStyle = getComputedStyle(element)
        const contentHeight = element.scrollHeight
        const containerHeight = element.clientHeight
        
        console.log('🔍 空白区域分析:', {
          container: element.className,
          containerHeight,
          contentHeight,
          boundingHeight: rect.height,
          paddingTop: computedStyle.paddingTop,
          paddingBottom: computedStyle.paddingBottom,
          marginTop: computedStyle.marginTop,
          marginBottom: computedStyle.marginBottom
        })
        
        // 计算实际内容占用的高度
        let actualContentHeight = 0
        const children = Array.from(element.children) as HTMLElement[]
        children.forEach(child => {
          const childRect = child.getBoundingClientRect()
          const childStyle = getComputedStyle(child)
          const childTotalHeight = childRect.height + 
            parseFloat(childStyle.marginTop) + 
            parseFloat(childStyle.marginBottom)
          actualContentHeight += childTotalHeight
        })
        
        console.log('📊 子元素高度分析:', {
          childrenCount: children.length,
          actualContentHeight,
          scrollHeight: contentHeight,
          difference: contentHeight - actualContentHeight
        })
        
        // 🔥 关键修复：完全忽略异常的actualContentHeight，直接使用容器高度
        console.log('⚠️ 检测到异常内容高度，强制使用容器高度')
        const finalHeight = containerHeight // 直接使用正确的容器高度
        
        if (finalHeight < containerHeight) {
          element.style.height = `${finalHeight}px`
          element.style.maxHeight = `${finalHeight}px`
          
          console.log('🎯 强制设置正确高度:', {
            container: element.className,
            原始高度: containerHeight,
            异常内容高度: actualContentHeight,
            强制设置高度: finalHeight,
            note: '忽略异常高度，强制使用正确值'
          })
        }
        
        // 强制限制所有子元素的高度 - 这是关键修复！
        children.forEach((child, index) => {
          child.style.marginBottom = index === children.length - 1 ? '0' : ''
          
          // 🔥 关键修复：强制限制子元素的最大高度为容器高度
          const maxChildHeight = finalHeight - 10 // 留10px缓冲
          child.style.maxHeight = `${maxChildHeight}px`
          child.style.height = `${maxChildHeight}px` // 强制设置固定高度，不使用auto
          child.style.minHeight = 'unset' // 移除min-height限制
          child.style.overflowY = 'auto' // 如果内容过多则滚动
          
          console.log('🔧 限制子元素高度:', {
            childIndex: index,
            childClass: child.className,
            原始高度: child.getBoundingClientRect().height,
            限制高度: maxChildHeight,
            note: '防止子元素撑开滚动区域'
          })
          
          // 递归处理深层子元素，防止嵌套元素使用异常高度
          const deepChildren = child.querySelectorAll('*') as NodeListOf<HTMLElement>
          deepChildren.forEach(deepChild => {
            const deepStyle = getComputedStyle(deepChild)
            if (deepStyle.height === '66667px' || deepStyle.minHeight === '66667px') {
              deepChild.style.height = 'auto'
              deepChild.style.minHeight = 'auto'
              deepChild.style.maxHeight = `${maxChildHeight}px`
              console.log('🔧 修复深层子元素异常高度:', deepChild.tagName, deepChild.className)
            }
          })
          
          // 检查是否有隐藏的空元素
          if (child.offsetHeight === 0 && child.innerHTML.trim() === '') {
            child.style.display = 'none'
            console.log('🗑️ 隐藏空元素:', child.className)
          }
        })
        
        // 强制移除容器本身的内边距
        element.style.paddingBottom = '0px'
        element.style.paddingTop = '0px'
        element.style.minHeight = 'auto' // 移除可能的最小高度限制
      } else {
        // 🔥 键盘隐藏时：恢复所有子元素的原始状态
        const children = Array.from(element.children) as HTMLElement[]
        children.forEach((child, index) => {
          child.style.height = '' // 恢复原始高度
          child.style.maxHeight = '' // 恢复原始最大高度
          child.style.minHeight = '' // 恢复原始最小高度
          child.style.overflowY = '' // 恢复原始滚动
          child.style.marginBottom = '' // 恢复原始边距
          child.style.display = '' // 恢复原始显示状态
          
          console.log('🔄 恢复子元素完整状态:', {
            childIndex: index,
            childClass: child.className,
            note: '键盘隐藏，恢复所有样式'
          })
          
          // 同时恢复深层子元素
          const deepChildren = child.querySelectorAll('*') as NodeListOf<HTMLElement>
          deepChildren.forEach(deepChild => {
            deepChild.style.height = ''
            deepChild.style.minHeight = ''
            deepChild.style.maxHeight = ''
          })
        })
        
        // 恢复容器本身
        element.style.paddingBottom = ''
        element.style.paddingTop = ''
        element.style.minHeight = ''
      }
    })
  }

  /**
   * 更新CSS变量
   */
  private updateCSSVariables(keyboardHeight: number) {
    const root = document.documentElement
    
    // 使用修正后的视口高度
    let viewportHeight = window.innerHeight
    if (viewportHeight > 10000 && window.screen?.height) {
      viewportHeight = window.screen.height
    }
    if (viewportHeight > 3000 || viewportHeight < 400) {
      viewportHeight = 800
    }
    
    const availableHeight = viewportHeight - keyboardHeight
    const safeKeyboardHeight = keyboardHeight + 20 // 添加安全边距

    root.style.setProperty('--keyboard-height', `${keyboardHeight}px`)
    root.style.setProperty('--viewport-height', `${viewportHeight}px`)
    root.style.setProperty('--available-height', `${availableHeight}px`)
    root.style.setProperty('--safe-keyboard-height', `${safeKeyboardHeight}px`)

    console.log('📐 更新CSS变量:', {
      keyboardHeight,
      viewportHeight: `${viewportHeight}px (修正后)`,
      originalInnerHeight: window.innerHeight,
      availableHeight,
      safeKeyboardHeight
    })
  }

  /**
   * 跟踪元素可见性
   */
  private trackElementVisibility(element: HTMLElement) {
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (!entry.isIntersecting && this.state.isOpen) {
            console.log('👀 元素不可见，需要调整位置')
            this.ensureElementVisible(element, this.state.height)
          }
        })
      },
      {
        root: this.findScrollContainer(element),
        rootMargin: `-${this.state.height}px 0px 0px 0px`,
        threshold: 0.1
      }
    )

    observer.observe(element)
    this.observers.add(observer)
  }

  /**
   * 停止跟踪元素
   */
  private stopTrackingElement(element: HTMLElement) {
    this.observers.forEach(observer => {
      observer.unobserve(element)
      observer.disconnect()
    })
    this.observers.clear()
  }

  /**
   * 确保元素可见
   */
  private ensureElementVisible(element: HTMLElement, keyboardHeight: number) {
    console.log('🎯 确保元素可见...')

    // 使用更可靠的视口高度计算方法
    let viewportHeight = window.innerHeight
    
    // 在原生应用中，如果innerHeight异常大，使用screen.height
    if (viewportHeight > 10000 && window.screen?.height) {
      viewportHeight = window.screen.height
      console.log('🔧 检测到异常视口高度，使用screen.height:', viewportHeight)
    }
    
    // 进一步验证：如果还是异常，使用常见的手机屏幕高度
    if (viewportHeight > 3000 || viewportHeight < 400) {
      viewportHeight = 800 // 使用合理的默认值
      console.log('🔧 使用默认视口高度:', viewportHeight)
    }

    const targetTop = viewportHeight - keyboardHeight - 20 // 20px缓冲区，与调试系统保持一致
    const scrollContainer = this.findScrollContainer(element)

    console.log('📐 视口计算:', {
      originalInnerHeight: window.innerHeight,
      screenHeight: window.screen?.height,
      usedViewportHeight: viewportHeight,
      keyboardHeight,
      targetTop,
      scrollContainer: scrollContainer ? scrollContainer.tagName + '.' + scrollContainer.className : 'window'
    })

    if (scrollContainer) {
      // 使用精确的滚动容器定位
      this.scrollElementIntoView(element, scrollContainer, targetTop)
    } else {
      // 使用 scrollIntoView 作为备选方案
      element.scrollIntoView({
        behavior: 'smooth',
        block: 'center',
        inline: 'nearest'
      })
      console.log('📜 使用scrollIntoView备选方案')
    }
  }

     /**
    * 将元素滚动到滚动容器的可见区域 - 优化版，消除停顿
    */
   private scrollElementIntoView(element: HTMLElement, scrollContainer: Element, targetTop: number) {
     const elementRect = element.getBoundingClientRect()
     const containerRect = scrollContainer.getBoundingClientRect()
     
     console.log('📊 滚动前状态:', {
       elementRect: { top: elementRect.top, bottom: elementRect.bottom },
       containerRect: { top: containerRect.top, bottom: containerRect.bottom },
       targetTop,
       currentScrollTop: scrollContainer.scrollTop,
       elementNeedsScroll: elementRect.bottom > targetTop
     })

     if (elementRect.bottom > targetTop) {
       // 更精确的一次性滚动计算
       const bufferSpace = 35 // 增加缓冲空间，确保一次到位
       const scrollDistance = elementRect.bottom - targetTop + bufferSpace
       const newScrollTop = scrollContainer.scrollTop + scrollDistance
       
       console.log('📜 执行一次性精确滚动:', {
         elementBottom: elementRect.bottom,
         targetTop,
         bufferSpace,
         scrollDistance,
         fromScrollTop: scrollContainer.scrollTop,
         toScrollTop: newScrollTop
       })

       // 一次性滚动到精确位置
       scrollContainer.scrollTo({
         top: Math.max(0, newScrollTop),
         behavior: 'smooth'
       })

       // 简化验证，只做最终确认，不进行二次滚动
       setTimeout(() => {
         const newRect = element.getBoundingClientRect()
         const isOptimal = newRect.bottom <= targetTop + 30
         
         console.log('📊 滚动完成验证:', {
           newElementBottom: newRect.bottom,
           targetTop,
           isOptimal: isOptimal ? '✅ 最佳位置' : '⚠️ 可接受偏差',
           deviation: newRect.bottom - targetTop,
           finalScrollTop: scrollContainer.scrollTop,
           note: '已优化为一次性滚动，消除停顿'
         })
       }, 200) // 减少验证延迟
     } else {
       console.log('✅ 元素已在可见区域内，无需滚动')
     }
   }

  /**
   * 查找滚动容器
   */
  private findScrollContainer(element: HTMLElement): Element | null {
    let parent = element.parentElement

    while (parent && parent !== document.body) {
      const style = getComputedStyle(parent)
      const overflow = style.overflow + style.overflowY

      if (overflow.includes('scroll') || overflow.includes('auto')) {
        return parent
      }

      // 检查常见的滚动容器类名
      if (parent.classList.contains('main-content') || 
          parent.classList.contains('scroll-container')) {
        return parent
      }

      parent = parent.parentElement
    }

    return null
  }

  /**
   * 判断是否为输入元素
   */
  private isInputElement(element: HTMLElement): boolean {
    return element.tagName === 'INPUT' || 
           element.tagName === 'TEXTAREA' || 
           element.contentEditable === 'true'
  }

  /**
   * 更新状态
   */
  private updateState(newState: Partial<KeyboardState>) {
    this.state = { ...this.state, ...newState }
  }

  /**
   * 通知回调
   */
  private notifyCallbacks() {
    this.callbacks.forEach(callback => {
      try {
        callback(this.state)
      } catch (error) {
        console.error('键盘状态回调错误:', error)
      }
    })
  }

  /**
   * 公共API
   */
  public getState(): KeyboardState {
    return { ...this.state }
  }

  public onStateChange(callback: (state: KeyboardState) => void): () => void {
    this.callbacks.add(callback)
    return () => this.callbacks.delete(callback)
  }

  public forceUpdate() {
    this.handleViewportChange()
  }

  public destroy() {
    this.observers.forEach(observer => observer.disconnect())
    this.observers.clear()
    
    if (this.resizeObserver) {
      this.resizeObserver.disconnect()
    }

    this.callbacks.clear()
    console.log('🗑️ 键盘管理器已销毁')
  }

  // 测试方法
  public simulateKeyboard(height: number) {
    console.log('🧪 模拟键盘:', height)
    if (height > 0) {
      this.handleKeyboardShow(height)
    } else {
      this.handleKeyboardHide()
    }
  }

  /**
   * 调试布局状态
   */
  public debugLayout() {
    console.log('=== 布局调试信息 ===')
    
    // 检查CSS变量
    const root = document.documentElement
    const cssVars = {
      keyboardHeight: root.style.getPropertyValue('--keyboard-height'),
      viewportHeight: root.style.getPropertyValue('--viewport-height'),
      availableHeight: root.style.getPropertyValue('--available-height')
    }
    console.log('CSS变量:', cssVars)
    
    // 检查视口信息
    const viewportInfo = {
      innerHeight: window.innerHeight,
      screenHeight: window.screen?.height,
      visualViewportHeight: window.visualViewport?.height
    }
    console.log('视口信息:', viewportInfo)
    
    // 检查滚动容器
    const scrollContainers = document.querySelectorAll('.modern-main-content, .scroll-container, .main-content')
    scrollContainers.forEach((container, index) => {
      const element = container as HTMLElement
      const rect = element.getBoundingClientRect()
      const computedStyle = getComputedStyle(element)
      
      console.log(`滚动容器 ${index + 1}:`, {
        className: element.className,
        // 尺寸信息
        clientHeight: element.clientHeight,
        scrollHeight: element.scrollHeight,
        offsetHeight: element.offsetHeight,
        // 位置信息
        boundingRect: {
          top: rect.top,
          bottom: rect.bottom,
          height: rect.height
        },
        // 样式信息
        computedHeight: computedStyle.height,
        computedMaxHeight: computedStyle.maxHeight,
        overflowY: computedStyle.overflowY,
        // 内联样式
        inlineHeight: element.style.height,
        inlineMaxHeight: element.style.maxHeight,
        inlineOverflowY: element.style.overflowY
      })
    })
    
    console.log('键盘状态:', this.state)
    console.log('==================')
  }

  /**
   * 专门分析空白区域的调试函数
   */
  public debugWhitespace() {
    console.log('=== 空白区域专项分析 ===')
    
    const scrollContainers = document.querySelectorAll('.modern-main-content, .scroll-container, .main-content')
    
    scrollContainers.forEach((container, index) => {
      const element = container as HTMLElement
      const rect = element.getBoundingClientRect()
      
      console.log(`容器 ${index + 1} (${element.className}):`)
      console.log('- 容器尺寸:', {
        clientHeight: element.clientHeight,
        scrollHeight: element.scrollHeight,
        offsetHeight: element.offsetHeight,
        boundingHeight: rect.height
      })
      
      // 分析所有子元素
      const children = Array.from(element.children) as HTMLElement[]
      let totalChildHeight = 0
      
      children.forEach((child, childIndex) => {
        const childRect = child.getBoundingClientRect()
        const childStyle = getComputedStyle(child)
        const childHeight = childRect.height + 
          parseFloat(childStyle.marginTop) + 
          parseFloat(childStyle.marginBottom)
        
        totalChildHeight += childHeight
        
        console.log(`  子元素 ${childIndex + 1}:`, {
          tagName: child.tagName,
          className: child.className,
          height: childRect.height,
          marginTop: childStyle.marginTop,
          marginBottom: childStyle.marginBottom,
          totalHeight: childHeight,
          isEmpty: child.innerHTML.trim() === '',
          isHidden: child.offsetHeight === 0
        })
      })
      
      const wastedSpace = element.scrollHeight - totalChildHeight
      const wastedPercentage = (wastedSpace / element.scrollHeight * 100).toFixed(1)
      
      console.log('- 空白分析:', {
        子元素总高度: totalChildHeight,
        滚动区域高度: element.scrollHeight,
        浪费空间: wastedSpace,
        浪费比例: `${wastedPercentage}%`,
        建议压缩到: totalChildHeight + 30
      })
      
      console.log('---')
    })
    
    console.log('===================')
  }


}

// 单例实例
export const keyboardManager = new ModernKeyboardManager()



// 导出类型
export type { KeyboardState, ViewportInfo }
export { ModernKeyboardManager } 
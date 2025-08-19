import React from 'react'
import { useBackButton } from '@/hooks/use-back-button'

/**
 * 返回键处理组件
 * 必须放在 Router 内部使用
 */
const BackButtonHandler: React.FC = () => {
  // 初始化返回键处理
  useBackButton({
    enableDoubleBackExit: true,
    doubleBackExitInterval: 2000
  })

  // 这个组件不渲染任何UI
  return null
}

export default BackButtonHandler 
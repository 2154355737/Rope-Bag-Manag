import { useEffect } from 'react'
import { useLocation } from 'react-router-dom'

interface ScrollRestorationProps {
  enabled?: boolean
  behavior?: ScrollBehavior
}

const ScrollRestoration: React.FC<ScrollRestorationProps> = ({ 
  enabled = false, 
  behavior = 'auto' 
}) => {
  const location = useLocation()

  useEffect(() => {
    if (!enabled) {
      // 禁用自动滚动 - 保持当前位置
      return
    }

    // 如果启用，则滚动到顶部
    window.scrollTo({
      top: 0,
      left: 0,
      behavior
    })
  }, [location.pathname, enabled, behavior])

  return null
}

export default ScrollRestoration 
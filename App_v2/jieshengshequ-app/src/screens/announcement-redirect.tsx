import React, { useEffect } from 'react'
import { useNavigate } from 'react-router-dom'
import { getAnnouncements } from '../api/announcements'
import { toast } from '@/hooks/use-toast'

const AnnouncementRedirect: React.FC = () => {
  const navigate = useNavigate()

  useEffect(() => {
    const run = async () => {
      try {
        const list = await getAnnouncements({ page: 1, pageSize: 1 })
        if (Array.isArray(list) && list.length > 0) {
          navigate(`/announcement/${list[0].id}`, { replace: true })
        } else {
          toast({ title: '暂无公告', description: '当前没有可显示的公告', variant: 'destructive' })
          navigate('/home', { replace: true })
        }
      } catch {
        toast({ title: '获取公告失败', description: '请稍后再试', variant: 'destructive' })
        navigate('/home', { replace: true })
      }
    }
    run()
  }, [navigate])

  return (
    <div className="flex items-center justify-center h-screen">
      <span className="text-sm text-muted-foreground">正在为你加载公告...</span>
    </div>
  )
}

export default AnnouncementRedirect 
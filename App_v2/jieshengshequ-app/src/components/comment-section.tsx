import React, { useState, useEffect } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { 
  Send, ThumbsUp, MessageSquare, Flag, MoreHorizontal, Star, ChevronDown, Loader2
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Input } from '@/components/ui/input'
import { toast } from '@/hooks/use-toast'

export interface Comment {
  id: number
  author: {
    name: string
    avatar: string
    verified?: boolean
  }
  content: string
  time: string
  likes: number
  isLiked?: boolean
  replies?: Comment[]
  rating?: number // 评分（1-5星，可选）
  helpful?: number // 有用数量（用于评价类评论）
}

interface CommentSectionProps {
  comments: Comment[]
  totalCount?: number
  onSubmitComment?: (content: string) => void
  onSubmitReply?: (commentId: number, content: string) => void
  onLikeComment?: (commentId: number) => void
  onReportComment?: (commentId: number) => void
  onLoadMoreComments?: (page: number) => Promise<Comment[]>
  placeholder?: string
  maxLength?: number
  className?: string
  showReplyCount?: boolean
  pageSize?: number
  initialCommentsToShow?: number
  hasMoreComments?: boolean
  isLoadingComments?: boolean
}

const CommentSection: React.FC<CommentSectionProps> = ({
  comments,
  totalCount,
  onSubmitComment,
  onSubmitReply,
  onLikeComment,
  onReportComment,
  onLoadMoreComments,
  placeholder = "发表评论...",
  maxLength = 200,
  className = "",
  showReplyCount = true,
  pageSize = 10,
  initialCommentsToShow = 5,
  hasMoreComments = false,
  isLoadingComments = false
}) => {
  const [commentText, setCommentText] = useState('')
  const [replyText, setReplyText] = useState('')
  const [showReplyInput, setShowReplyInput] = useState<number | null>(null)
  const [expandedReplies, setExpandedReplies] = useState<Set<number>>(new Set())
  const [displayedComments, setDisplayedComments] = useState<Comment[]>([])
  const [showAllComments, setShowAllComments] = useState(false)
  const [currentPage, setCurrentPage] = useState(1)
  const [isLoadingMore, setIsLoadingMore] = useState(false)

  // 初始化显示的评论
  useEffect(() => {
    if (comments.length === 0) {
      setDisplayedComments([])
      setShowAllComments(false)
      return
    }

    // 初始只显示指定数量的评论
    if (!showAllComments) {
      setDisplayedComments(comments.slice(0, initialCommentsToShow))
    } else {
      setDisplayedComments(comments)
    }
  }, [comments, showAllComments, initialCommentsToShow])

  // 加载更多评论
  const handleLoadMoreComments = async () => {
    if (isLoadingMore || !onLoadMoreComments) return

    setIsLoadingMore(true)
    try {
      const nextPage = currentPage + 1
      const newComments = await onLoadMoreComments(nextPage)
      
      if (newComments.length > 0) {
        setDisplayedComments(prev => [...prev, ...newComments])
        setCurrentPage(nextPage)
      }
    } catch (error) {
      toast({
        title: "加载失败",
        description: "评论加载失败，请重试",
        variant: "destructive"
      })
    } finally {
      setIsLoadingMore(false)
    }
  }

  // 展开显示所有已有的评论
  const handleShowMoreComments = () => {
    if (comments.length <= initialCommentsToShow) return
    
    if (!showAllComments) {
      setDisplayedComments(comments)
      setShowAllComments(true)
    }
  }

  // 计算是否需要显示"查看更多"按钮  
  const shouldShowMoreButton = !showAllComments && comments.length > initialCommentsToShow
  const shouldShowLoadMoreButton = showAllComments && hasMoreComments && onLoadMoreComments

  // 调试信息 (开发环境)
  if (process.env.NODE_ENV === 'development') {
    console.log('CommentSection 状态:', {
      commentsLength: comments.length,
      initialCommentsToShow,
      showAllComments,
      shouldShowMoreButton,
      shouldShowLoadMoreButton,
      displayedCommentsLength: displayedComments.length
    })
  }

  // 提交评论
  const handleSubmitComment = () => {
    if (!commentText.trim()) {
      toast({
        title: "评论不能为空",
        variant: "destructive"
      })
      return
    }

    onSubmitComment?.(commentText.trim())
    setCommentText('')
    
    toast({
      title: "评论发送成功",
      description: "您的评论已发布"
    })
  }

  // 提交回复
  const handleSubmitReply = (commentId: number) => {
    if (!replyText.trim()) {
      toast({
        title: "回复不能为空",
        variant: "destructive"
      })
      return
    }

    onSubmitReply?.(commentId, replyText.trim())
    setReplyText('')
    setShowReplyInput(null)
    
    toast({
      title: "回复发送成功",
      description: "您的回复已发布"
    })
  }

  // 点赞评论
  const handleLikeComment = (commentId: number) => {
    onLikeComment?.(commentId)
  }

  // 举报评论
  const handleReportComment = (commentId: number) => {
    onReportComment?.(commentId)
    toast({
      title: "举报已提交",
      description: "我们会尽快处理您的举报"
    })
  }

  // 切换回复展开状态
  const toggleReplies = (commentId: number) => {
    const newExpanded = new Set(expandedReplies)
    if (newExpanded.has(commentId)) {
      newExpanded.delete(commentId)
    } else {
      newExpanded.add(commentId)
    }
    setExpandedReplies(newExpanded)
  }

  // 渲染评论项
  const renderComment = (comment: Comment, isReply: boolean = false) => (
    <motion.div
      key={comment.id}
      initial={{ opacity: 0, y: 10 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.3 }}
      className={isReply ? "ml-8 mt-3" : ""}
    >
      <Card className={isReply ? "bg-muted/30" : ""}>
        <CardContent className="p-3">
          <div className="flex items-start">
            <Avatar className={`${isReply ? 'h-6 w-6' : 'h-8 w-8'} mr-2 shrink-0`}>
              <AvatarImage src={comment.author.avatar} />
              <AvatarFallback>{comment.author.name[0]}</AvatarFallback>
            </Avatar>
            
            <div className="flex-1 min-w-0">
              <div className="flex items-center justify-between mb-1">
                <div className="flex items-center gap-1">
                  <span className={`font-medium ${isReply ? 'text-sm' : 'text-sm'}`}>
                    {comment.author.name}
                  </span>
                  {comment.author.verified && (
                    <div className="w-3 h-3 bg-blue-500 rounded-full flex items-center justify-center">
                      <div className="w-1.5 h-1.5 bg-white rounded-full" />
                    </div>
                  )}
                </div>
                <div className="flex items-center gap-1">
                  <span className="text-xs text-muted-foreground">{comment.time}</span>
                  <Button variant="ghost" size="icon" className="h-6 w-6">
                    <MoreHorizontal size={12} />
                  </Button>
                </div>
              </div>
              
              <p className={`${isReply ? 'text-sm' : 'text-sm'} mb-2 break-words`}>
                {comment.content}
              </p>
              
              <div className="flex items-center gap-0.5">
                <Button 
                  variant={comment.isLiked ? "like-active" : "like"} 
                  size="compact" 
                  className="text-[10px]"
                  onClick={() => handleLikeComment(comment.id)}
                >
                  <ThumbsUp size={10} className="mr-0.5" /> 
                  {comment.likes}
                </Button>
                
                {!isReply && (
                  <Button 
                    variant="minimal" 
                    size="compact" 
                    className="text-[10px]"
                    onClick={() => setShowReplyInput(showReplyInput === comment.id ? null : comment.id)}
                  >
                    <MessageSquare size={10} className="mr-0.5" /> 
                    回复
                  </Button>
                )}
                
                <Button 
                  variant="report" 
                  size="compact" 
                  className="text-[10px]"
                  onClick={() => handleReportComment(comment.id)}
                >
                  <Flag size={10} className="mr-0.5" /> 
                  举报
                </Button>
              </div>
              
              {/* 回复输入框 */}
              {showReplyInput === comment.id && (
                <motion.div
                  initial={{ opacity: 0, height: 0 }}
                  animate={{ opacity: 1, height: 'auto' }}
                  exit={{ opacity: 0, height: 0 }}
                  className="mt-3"
                >
                  <div className="flex items-center gap-2">
                    <Input
                      placeholder={`回复 ${comment.author.name}...`}
                      className="text-sm h-8"
                      value={replyText}
                      onChange={(e) => setReplyText(e.target.value)}
                      maxLength={maxLength}
                      onKeyPress={(e) => {
                        if (e.key === 'Enter') {
                          handleSubmitReply(comment.id)
                        }
                      }}
                    />
                    <Button 
                      size="sm" 
                      className="h-8 px-3"
                      disabled={!replyText.trim()}
                      onClick={() => handleSubmitReply(comment.id)}
                    >
                      发送
                    </Button>
                  </div>
                </motion.div>
              )}
              
              {/* 回复列表 */}
              {comment.replies && comment.replies.length > 0 && (
                <div className="mt-3">
                  {!expandedReplies.has(comment.id) ? (
                    <Button
                      variant="ghost"
                      size="sm"
                      className="h-6 px-2 text-xs text-primary"
                      onClick={() => toggleReplies(comment.id)}
                    >
                      查看 {comment.replies.length} 条回复
                    </Button>
                  ) : (
                    <>
                      <div className="space-y-2">
                        {comment.replies.map((reply) => renderComment(reply, true))}
                      </div>
                      <Button
                        variant="ghost"
                        size="sm"
                        className="h-6 px-2 text-xs text-muted-foreground mt-2"
                        onClick={() => toggleReplies(comment.id)}
                      >
                        收起回复
                      </Button>
                    </>
                  )}
                </div>
              )}
            </div>
          </div>
        </CardContent>
      </Card>
    </motion.div>
  )

  return (
    <div className={`space-y-4 ${className}`}>
      {/* 评论标题 */}
      <div className="flex items-center justify-between">
        <h3 className="text-lg font-medium">
          评论 {totalCount !== undefined ? `(${totalCount})` : `(${comments.length})`}
        </h3>
        <div className="flex items-center gap-2">
          {showReplyCount && comments.length > 0 && (
            <span className="text-sm text-muted-foreground">
              {comments.reduce((acc, comment) => acc + (comment.replies?.length || 0), 0)} 条回复
            </span>
          )}
          {/* 调试信息 (仅开发环境显示) */}
          {process.env.NODE_ENV === 'development' && (
            <span className="text-xs text-muted-foreground bg-muted px-2 py-1 rounded">
              显示: {displayedComments.length}/{comments.length}
            </span>
          )}
        </div>
      </div>

      {/* 评论输入框 */}
      <Card>
        <CardContent className="p-4">
          <div className="flex items-center gap-3">
            <Input
              placeholder={placeholder}
              className="flex-1"
              value={commentText}
              onChange={(e) => setCommentText(e.target.value)}
              maxLength={maxLength}
              onKeyPress={(e) => {
                if (e.key === 'Enter' && !e.shiftKey) {
                  e.preventDefault()
                  handleSubmitComment()
                }
              }}
            />
            <Button 
              disabled={!commentText.trim()}
              onClick={handleSubmitComment}
            >
              <Send size={16} className="mr-1" />
              发送
            </Button>
          </div>
          <div className="flex justify-between items-center mt-2">
            <span className="text-xs text-muted-foreground">
              {commentText.length}/{maxLength} 字符
            </span>
            <span className="text-xs text-muted-foreground">
              按 Enter 发送，Shift + Enter 换行
            </span>
          </div>
        </CardContent>
      </Card>

      {/* 评论列表 */}
      <div className="space-y-3">
        {comments.length === 0 ? (
          <Card>
            <CardContent className="p-8 text-center">
              <MessageSquare size={48} className="mx-auto text-muted-foreground mb-2" />
              <p className="text-muted-foreground">暂无评论</p>
              <p className="text-sm text-muted-foreground mt-1">来发表第一条评论吧！</p>
            </CardContent>
          </Card>
        ) : (
          <>
            <AnimatePresence mode="popLayout">
              {displayedComments.map((comment) => renderComment(comment))}
            </AnimatePresence>

            {/* 查看更多评论按钮 */}
            {shouldShowMoreButton && (
              <motion.div
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                className="flex justify-center pt-4"
              >
                <Button
                  variant="outline"
                  onClick={handleShowMoreComments}
                  className="group hover:bg-primary/5 transition-colors"
                  disabled={isLoadingComments}
                >
                  <ChevronDown size={16} className="mr-2 group-hover:animate-bounce transition-transform" />
                  查看更多评论 ({comments.length - initialCommentsToShow} 条)
                </Button>
              </motion.div>
            )}

            {/* 加载更多评论按钮 */}
            {shouldShowLoadMoreButton && (
              <motion.div
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                className="flex flex-col items-center pt-4"
              >
                <div className="w-full h-px bg-gradient-to-r from-transparent via-border to-transparent mb-4" />
                <Button
                  variant="outline"
                  onClick={handleLoadMoreComments}
                  disabled={isLoadingMore || isLoadingComments}
                  className="group hover:bg-primary/5 transition-colors min-w-32"
                >
                  {isLoadingMore ? (
                    <>
                      <Loader2 size={16} className="mr-2 animate-spin" />
                      加载中...
                    </>
                  ) : (
                    <>
                      <ChevronDown size={16} className="mr-2 group-hover:animate-bounce transition-transform" />
                      加载更多评论
                    </>
                  )}
                </Button>
                <div className="text-xs text-muted-foreground mt-2">
                  {hasMoreComments ? '还有更多精彩评论' : ''}
                </div>
              </motion.div>
            )}

            {/* 加载状态指示器 */}
            {isLoadingComments && (
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                className="flex justify-center py-6"
              >
                <div className="flex flex-col items-center gap-3">
                  <div className="flex items-center gap-2 text-muted-foreground">
                    <Loader2 size={20} className="animate-spin text-primary" />
                    <span className="text-sm">正在加载评论...</span>
                  </div>
                  <div className="flex gap-1">
                    {[0, 1, 2].map((i) => (
                      <motion.div
                        key={i}
                        className="w-2 h-2 bg-primary/30 rounded-full"
                        animate={{
                          scale: [1, 1.2, 1],
                          opacity: [0.3, 1, 0.3]
                        }}
                        transition={{
                          duration: 1,
                          repeat: Infinity,
                          delay: i * 0.2
                        }}
                      />
                    ))}
                  </div>
                </div>
              </motion.div>
            )}

            {/* 无更多评论提示 */}
            {showAllComments && !hasMoreComments && comments.length > initialCommentsToShow && (
              <motion.div
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                className="flex flex-col items-center py-6"
              >
                <div className="w-full h-px bg-gradient-to-r from-transparent via-border to-transparent mb-4" />
                <div className="text-center text-muted-foreground">
                  <MessageSquare size={24} className="mx-auto mb-2 opacity-50" />
                  <p className="text-sm">已显示全部评论</p>
                  <p className="text-xs mt-1">感谢大家的热情参与 🎉</p>
                </div>
              </motion.div>
            )}
          </>
        )}
      </div>
    </div>
  )
}

export default CommentSection 
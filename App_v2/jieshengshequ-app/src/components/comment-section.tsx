import React, { useState } from 'react'
import { motion } from 'framer-motion'
import { 
  Send, ThumbsUp, MessageSquare, Flag, MoreHorizontal, Star
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
  placeholder?: string
  maxLength?: number
  className?: string
  showReplyCount?: boolean
}

const CommentSection: React.FC<CommentSectionProps> = ({
  comments,
  totalCount,
  onSubmitComment,
  onSubmitReply,
  onLikeComment,
  onReportComment,
  placeholder = "发表评论...",
  maxLength = 200,
  className = "",
  showReplyCount = true
}) => {
  const [commentText, setCommentText] = useState('')
  const [replyText, setReplyText] = useState('')
  const [showReplyInput, setShowReplyInput] = useState<number | null>(null)
  const [expandedReplies, setExpandedReplies] = useState<Set<number>>(new Set())

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
              
              <div className="flex items-center gap-1">
                <Button 
                  variant="ghost" 
                  size="sm" 
                  className={`h-6 px-2 text-xs ${comment.isLiked ? 'text-primary' : 'text-muted-foreground'}`}
                  onClick={() => handleLikeComment(comment.id)}
                >
                  <ThumbsUp size={12} className="mr-1" /> 
                  {comment.likes}
                </Button>
                
                {!isReply && (
                  <Button 
                    variant="ghost" 
                    size="sm" 
                    className="h-6 px-2 text-xs text-muted-foreground"
                    onClick={() => setShowReplyInput(showReplyInput === comment.id ? null : comment.id)}
                  >
                    <MessageSquare size={12} className="mr-1" /> 
                    回复
                  </Button>
                )}
                
                <Button 
                  variant="ghost" 
                  size="sm" 
                  className="h-6 px-2 text-xs text-muted-foreground"
                  onClick={() => handleReportComment(comment.id)}
                >
                  <Flag size={12} className="mr-1" /> 
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
        {showReplyCount && comments.length > 0 && (
          <span className="text-sm text-muted-foreground">
            {comments.reduce((acc, comment) => acc + (comment.replies?.length || 0), 0)} 条回复
          </span>
        )}
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
          comments.map((comment) => renderComment(comment))
        )}
      </div>
    </div>
  )
}

export default CommentSection 
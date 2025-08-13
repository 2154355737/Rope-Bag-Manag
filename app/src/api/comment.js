import request from '../utils/request'

// 点赞评论
export const likeComment = (commentId, like = true) => {
  return request.post(`/comments/${commentId}/like`, {
    like
  })
}

// 取消点赞评论
export const unlikeComment = (commentId) => {
  return request.post(`/comments/${commentId}/like`, {
    like: false
  })
}

// 检查评论点赞状态
export const checkCommentLikeStatus = (commentId) => {
  return request.get(`/comments/${commentId}/like-status`)
}

// 点踩评论
export const dislikeComment = (commentId, dislike = true) => {
  return request.post(`/comments/${commentId}/dislike`, {
    dislike
  })
}

// 取消点踩评论
export const undislikeComment = (commentId) => {
  return request.post(`/comments/${commentId}/dislike`, {
    dislike: false
  })
} 
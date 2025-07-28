import { api, ApiResponse } from '../utils/apiClient'

export interface ForbiddenWord {
  id: number
  word: string
}

export const forbiddenWordApi = {
  getAll(): Promise<ApiResponse<ForbiddenWord[]>> {
    return api.get('/v1/forbidden-words')
  },
  add(word: string): Promise<{ code: number }> {
    return api.post('/v1/forbidden-words', { word })
  },
  delete(id: number): Promise<{ code: number }> {
    return api.delete(`/v1/forbidden-words/${id}`)
  }
} 
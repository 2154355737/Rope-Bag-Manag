import { AxiosError, AxiosInstance } from 'axios'
/**
 *  请求重试机制
 */

export class AxiosRetry {
  /**
   * 重试
   */
  retry(AxiosInstance: AxiosInstance, error: AxiosError) {
    // 4xx（如404）为客户端错误，不进行自动重试
    const status = error?.response?.status
    if (status && status >= 400 && status < 500) {
      return Promise.reject(error)
    }

    // @ts-ignore
    const { config } = error.response
    const { waitTime, count } = config?.requestOptions?.retryRequest
    config.__retryCount = config.__retryCount || 0
    if (config.__retryCount >= count) {
      return Promise.reject(error)
    }
    config.__retryCount += 1
    return this.delay(waitTime).then(() => AxiosInstance(config))
  }

  /**
   * 延迟
   */
  private delay(waitTime: number) {
    return new Promise((resolve) => setTimeout(resolve, waitTime))
  }
}

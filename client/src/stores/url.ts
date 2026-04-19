import { defineStore } from 'pinia'
import { ref } from 'vue'
import { api, ApiError } from '@/services/api'
import type { CreateShortUrlRequest, UrlResponse } from '@/types'
import { MAX_RECENT_URLS } from '@/constants'

export const useUrlStore = defineStore('url', () => {
  const recentUrls = ref<UrlResponse[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function createShortUrl(data: CreateShortUrlRequest): Promise<UrlResponse | null> {
    loading.value = true
    error.value = null

    try {
      const result = await api.createShortUrl(data)
      recentUrls.value.unshift(result)

      if (recentUrls.value.length > MAX_RECENT_URLS) {
        recentUrls.value = recentUrls.value.slice(0, MAX_RECENT_URLS)
      }

      return result
    } catch (err) {
      if (err instanceof ApiError) {
        error.value = err.message
      } else {
        error.value = 'Failed to create short URL'
      }
      return null
    } finally {
      loading.value = false
    }
  }

  function clearError() {
    error.value = null
  }

  function clearRecentUrls() {
    recentUrls.value = []
  }

  return {
    recentUrls,
    loading,
    error,
    createShortUrl,
    clearError,
    clearRecentUrls,
  }
}, {
  persist: {
    pick: ['recentUrls'],
  },
})

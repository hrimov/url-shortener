import { describe, it, expect, vi, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useUrlStore } from './url'

vi.mock('@/services/api', () => ({
  api: {
    createShortUrl: vi.fn(),
  },
  ApiError: class ApiError extends Error {
    status: number
    constructor(message: string, status: number) {
      super(message)
      this.name = 'ApiError'
      this.status = status
    }
  },
}))

import { api, ApiError } from '@/services/api'

describe('url store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.clearAllMocks()
  })

  it('starts with empty state', () => {
    const store = useUrlStore()
    expect(store.recentUrls).toEqual([])
    expect(store.loading).toBe(false)
    expect(store.error).toBeNull()
  })

  it('creates short URL and adds to recent list', async () => {
    const mockResponse = {
      id: 1,
      long_url: 'https://example.com',
      short_url: 'abc1234',
      status: 'active' as const,
      created_at: '2026-01-01T00:00:00Z',
      updated_at: '2026-01-01T00:00:00Z',
    }

    vi.mocked(api.createShortUrl).mockResolvedValueOnce(mockResponse)

    const store = useUrlStore()
    const result = await store.createShortUrl({ long_url: 'https://example.com' })

    expect(result).toEqual(mockResponse)
    expect(store.recentUrls).toHaveLength(1)
    expect(store.recentUrls[0].short_url).toBe('abc1234')
    expect(store.loading).toBe(false)
    expect(store.error).toBeNull()
  })

  it('prepends new URLs to recent list', async () => {
    const store = useUrlStore()

    vi.mocked(api.createShortUrl)
      .mockResolvedValueOnce({
        id: 1,
        long_url: 'https://first.com',
        short_url: 'first',
        status: 'active',
        created_at: '2026-01-01T00:00:00Z',
        updated_at: '2026-01-01T00:00:00Z',
      })
      .mockResolvedValueOnce({
        id: 2,
        long_url: 'https://second.com',
        short_url: 'second',
        status: 'active',
        created_at: '2026-01-01T00:00:00Z',
        updated_at: '2026-01-01T00:00:00Z',
      })

    await store.createShortUrl({ long_url: 'https://first.com' })
    await store.createShortUrl({ long_url: 'https://second.com' })

    expect(store.recentUrls).toHaveLength(2)
    expect(store.recentUrls[0].short_url).toBe('second')
    expect(store.recentUrls[1].short_url).toBe('first')
  })

  it('caps recent URLs at MAX_RECENT_URLS', async () => {
    const store = useUrlStore()

    for (let i = 0; i < 11; i++) {
      vi.mocked(api.createShortUrl).mockResolvedValueOnce({
        id: i,
        long_url: `https://example${i}.com`,
        short_url: `url${i}`,
        status: 'active',
        created_at: '2026-01-01T00:00:00Z',
        updated_at: '2026-01-01T00:00:00Z',
      })
      await store.createShortUrl({ long_url: `https://example${i}.com` })
    }

    expect(store.recentUrls).toHaveLength(10)
    expect(store.recentUrls[0].short_url).toBe('url10')
  })

  it('sets error on ApiError', async () => {
    vi.mocked(api.createShortUrl).mockRejectedValueOnce(
      new ApiError('Custom short URL already exists', 409),
    )

    const store = useUrlStore()
    const result = await store.createShortUrl({
      long_url: 'https://example.com',
      custom_short_url: 'taken',
    })

    expect(result).toBeNull()
    expect(store.error).toBe('Custom short URL already exists')
    expect(store.loading).toBe(false)
  })

  it('sets generic error on unexpected failure', async () => {
    vi.mocked(api.createShortUrl).mockRejectedValueOnce(new Error('Network error'))

    const store = useUrlStore()
    const result = await store.createShortUrl({ long_url: 'https://example.com' })

    expect(result).toBeNull()
    expect(store.error).toBe('Failed to create short URL')
  })

  it('clearError resets error state', async () => {
    vi.mocked(api.createShortUrl).mockRejectedValueOnce(new Error('fail'))

    const store = useUrlStore()
    await store.createShortUrl({ long_url: 'https://example.com' })
    expect(store.error).not.toBeNull()

    store.clearError()
    expect(store.error).toBeNull()
  })

  it('clearRecentUrls empties the list', async () => {
    const store = useUrlStore()

    vi.mocked(api.createShortUrl).mockResolvedValueOnce({
      id: 1,
      long_url: 'https://example.com',
      short_url: 'abc',
      status: 'active',
      created_at: '2026-01-01T00:00:00Z',
      updated_at: '2026-01-01T00:00:00Z',
    })

    await store.createShortUrl({ long_url: 'https://example.com' })
    expect(store.recentUrls).toHaveLength(1)

    store.clearRecentUrls()
    expect(store.recentUrls).toEqual([])
  })
})

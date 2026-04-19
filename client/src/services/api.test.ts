import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { api, ApiError } from './api'

describe('api service', () => {
  const mockFetch = vi.fn()

  beforeEach(() => {
    vi.stubGlobal('fetch', mockFetch)
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  describe('createShortUrl', () => {
    it('returns UrlResponse on success', async () => {
      const mockResponse = {
        id: 1,
        long_url: 'https://example.com',
        short_url: 'abc1234',
        status: 'active',
        created_at: '2026-01-01T00:00:00Z',
        updated_at: '2026-01-01T00:00:00Z',
      }

      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve(mockResponse),
      })

      const result = await api.createShortUrl({ long_url: 'https://example.com' })

      expect(result).toEqual(mockResponse)
      expect(mockFetch).toHaveBeenCalledOnce()

      const [url, options] = mockFetch.mock.calls[0]
      expect(url).toContain('/url')
      expect(options.method).toBe('POST')
      expect(options.headers).toEqual({ 'Content-Type': 'application/json' })
      expect(JSON.parse(options.body)).toEqual({ long_url: 'https://example.com' })
    })

    it('throws ApiError with server message on failure', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 409,
        json: () => Promise.resolve({ error: 'Custom short URL already exists' }),
      })

      try {
        await api.createShortUrl({ long_url: 'https://example.com', custom_short_url: 'taken' })
        expect.fail('should have thrown')
      } catch (err) {
        expect(err).toBeInstanceOf(ApiError)
        expect((err as ApiError).status).toBe(409)
        expect((err as ApiError).message).toBe('Custom short URL already exists')
      }
    })

    it('throws ApiError with default message when response body is not JSON', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 500,
        json: () => Promise.reject(new Error('invalid json')),
      })

      await expect(api.createShortUrl({ long_url: 'https://example.com' })).rejects.toThrow(
        'Failed to create short URL',
      )
    })
  })

  describe('healthcheck', () => {
    it('returns status on success', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve({ status: 'ok', database: true, cache: true }),
      })

      const result = await api.healthcheck()
      expect(result.status).toBe('ok')
    })

    it('throws ApiError on failure', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 503,
      })

      await expect(api.healthcheck()).rejects.toThrow(ApiError)
    })
  })

  describe('getShortUrl', () => {
    it('builds URL from window.location.origin when no API_BASE', () => {
      const url = api.getShortUrl('abc123')
      expect(url).toContain('abc123')
    })
  })
})

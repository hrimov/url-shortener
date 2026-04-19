import type { CreateShortUrlRequest, UrlResponse, ErrorResponse } from '@/types'

/**
 * API base URL:
 * - If VITE_API_URL is set (e.g. http://localhost:3100), use it directly.
 * - In Kubernetes (no VITE_API_URL), calls go to same-origin `/api/...`.
 */
const API_BASE = import.meta.env.VITE_API_URL || ''
const USE_API_PREFIX = !API_BASE

function buildUrl(path: string): string {
  const normalizedPath = path.startsWith('/') ? path : `/${path}`
  const effectivePath = USE_API_PREFIX ? `/api${normalizedPath}` : normalizedPath

  if (!API_BASE) return effectivePath

  const base = API_BASE.endsWith('/') ? API_BASE.slice(0, -1) : API_BASE
  return `${base}${effectivePath}`
}

export class ApiError extends Error {
  status: number

  constructor(message: string, status: number) {
    super(message)
    this.name = 'ApiError'
    this.status = status
  }
}

export const api = {
  async createShortUrl(data: CreateShortUrlRequest): Promise<UrlResponse> {
    const response = await fetch(buildUrl('/url'), {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
      signal: AbortSignal.timeout(10_000),
    })

    if (!response.ok) {
      let message = 'Failed to create short URL'
      try {
        const errorBody = (await response.json()) as ErrorResponse
        if (errorBody.error) message = errorBody.error
      } catch {
        /* keep default message */
      }
      throw new ApiError(message, response.status)
    }

    return (await response.json()) as UrlResponse
  },

  async healthcheck(): Promise<{ status: string }> {
    const response = await fetch(buildUrl('/healthcheck'), {
      signal: AbortSignal.timeout(5_000),
    })

    if (!response.ok) {
      throw new ApiError('Healthcheck failed', response.status)
    }

    return (await response.json()) as { status: string }
  },

  /**
   * Build the publicly visible short URL.
   * - In Kubernetes (no VITE_API_URL): same-origin `/{short}`.
   * - Locally (VITE_API_URL=http://localhost:3100): `http://localhost:3100/{short}`.
   */
  getShortUrl(shortUrl: string): string {
    if (!API_BASE) {
      return `${window.location.origin}/${shortUrl}`
    }
    const base = API_BASE.endsWith('/') ? API_BASE.slice(0, -1) : API_BASE
    return `${base}/${shortUrl}`
  },
}

export interface CreateShortUrlRequest {
  long_url: string
  custom_short_url?: string
}

export interface UrlResponse {
  id: number
  long_url: string
  short_url: string
  status: UrlStatus
  created_at: string
  updated_at: string
}

export interface ErrorResponse {
  error: string
}

export type UrlStatus = 'active' | 'inactive' | 'expired'

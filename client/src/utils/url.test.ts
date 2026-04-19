import { describe, it, expect } from 'vitest'
import { normalizeUrl, isValidUrl } from './url'

describe('normalizeUrl', () => {
  it('returns URL unchanged when it has http scheme', () => {
    expect(normalizeUrl('http://example.com')).toBe('http://example.com')
  })

  it('returns URL unchanged when it has https scheme', () => {
    expect(normalizeUrl('https://example.com')).toBe('https://example.com')
  })

  it('is case-insensitive for scheme detection', () => {
    expect(normalizeUrl('HTTP://example.com')).toBe('HTTP://example.com')
    expect(normalizeUrl('HTTPS://example.com')).toBe('HTTPS://example.com')
  })

  it('prepends http:// when no scheme is present', () => {
    expect(normalizeUrl('example.com')).toBe('http://example.com')
  })

  it('prepends http:// for URLs with paths but no scheme', () => {
    expect(normalizeUrl('example.com/path?q=1')).toBe('http://example.com/path?q=1')
  })

  it('does not double-prepend for ftp:// URLs (non http/https)', () => {
    expect(normalizeUrl('ftp://example.com')).toBe('http://ftp://example.com')
  })
})

describe('isValidUrl', () => {
  it('accepts valid http URL', () => {
    expect(isValidUrl('http://example.com')).toBe(true)
  })

  it('accepts valid https URL', () => {
    expect(isValidUrl('https://example.com')).toBe(true)
  })

  it('accepts URL without scheme (gets normalized)', () => {
    expect(isValidUrl('example.com')).toBe(true)
  })

  it('accepts URL with path and query params', () => {
    expect(isValidUrl('https://example.com/path?foo=bar&baz=1')).toBe(true)
  })

  it('rejects empty string', () => {
    expect(isValidUrl('')).toBe(false)
  })

  it('rejects whitespace-only input', () => {
    expect(isValidUrl('   ')).toBe(false)
  })

  it('rejects obviously invalid URL', () => {
    expect(isValidUrl('not a url at all')).toBe(false)
  })

  it('rejects URL where hostname contains spaces (Chrome URL API encodes them)', () => {
    expect(isValidUrl('http://not a host')).toBe(false)
    expect(isValidUrl('no spaces allowed')).toBe(false)
  })

  it('rejects single-label hostname with no TLD', () => {
    expect(isValidUrl('http://askdakdka')).toBe(false)
    expect(isValidUrl('http://localhost')).toBe(false)
    expect(isValidUrl('justahostname')).toBe(false)
  })

  it('rejects TLD shorter than 2 chars', () => {
    expect(isValidUrl('http://example.c')).toBe(false)
  })

  it('accepts hostname with subdomains', () => {
    expect(isValidUrl('https://sub.example.com')).toBe(true)
    expect(isValidUrl('https://a.b.c.example.co.uk')).toBe(true)
  })

})

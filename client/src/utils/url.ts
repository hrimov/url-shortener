const SCHEME_RE = /^https?:\/\//i
const HOSTNAME_RE = /^([a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,63}$/

export function normalizeUrl(raw: string): string {
  return SCHEME_RE.test(raw) ? raw : `http://${raw}`
}

export function isValidUrl(raw: string): boolean {
  const normalized = normalizeUrl(raw)
  try {
    const url = new URL(normalized)
    return HOSTNAME_RE.test(url.hostname)
  } catch {
    return false
  }
}

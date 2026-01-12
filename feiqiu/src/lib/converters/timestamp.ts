/**
 * Timestamp conversion utilities
 *
 * Converts between backend i64 millisecond timestamps and frontend ISO 8601 strings.
 */

/**
 * Converts i64 milliseconds to ISO 8601 date string
 * @param millis - Timestamp in milliseconds since Unix epoch
 * @returns ISO 8601 formatted date string
 */
export function toIsoDate(millis: number): string {
  // Handle edge cases
  if (!Number.isFinite(millis) || millis === 0) {
    return new Date().toISOString()
  }
  return new Date(millis).toISOString()
}

/**
 * Converts ISO 8601 date string to i64 milliseconds
 * @param iso - ISO 8601 formatted date string
 * @returns Timestamp in milliseconds since Unix epoch
 */
export function fromIsoDate(iso: string): number {
  const date = new Date(iso)
  const millis = date.getTime()

  // Handle invalid dates
  if (Number.isNaN(millis)) {
    return Date.now()
  }

  return millis
}

/**
 * Gets current timestamp as i64 milliseconds
 * @returns Current timestamp in milliseconds
 */
export function nowInMillis(): number {
  return Date.now()
}

/**
 * Gets current timestamp as ISO string
 * @returns Current timestamp as ISO string
 */
export function nowAsIso(): string {
  return new Date().toISOString()
}

/**
 * Formats a timestamp for display (relative time)
 * @param iso - ISO 8601 date string or milliseconds
 * @returns Human readable relative time (e.g., "2 hours ago")
 */
export function formatRelativeTime(iso: string | number): string {
  const millis = typeof iso === 'string' ? fromIsoDate(iso) : iso
  const now = Date.now()
  const diff = now - millis

  const seconds = Math.floor(diff / 1000)
  const minutes = Math.floor(seconds / 60)
  const hours = Math.floor(minutes / 60)
  const days = Math.floor(hours / 24)

  if (seconds < 60) {
    return 'just now'
  } else if (minutes < 60) {
    return `${minutes} minute${minutes > 1 ? 's' : ''} ago`
  } else if (hours < 24) {
    return `${hours} hour${hours > 1 ? 's' : ''} ago`
  } else if (days < 7) {
    return `${days} day${days > 1 ? 's' : ''} ago`
  } else {
    return new Date(millis).toLocaleDateString()
  }
}

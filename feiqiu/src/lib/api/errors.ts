/**
 * API Error types
 *
 * Custom error classes for IPC communication failures.
 */

/**
 * Error thrown when an IPC command fails
 */
export class IpcError extends Error {
  constructor(
    public command: string,
    public originalError: unknown
  ) {
    super(`IPC command '${command}' failed: ${formatError(originalError)}`)
    this.name = 'IpcError'
  }

  /**
   * Check if this error is caused by a network issue
   */
  isNetworkError(): boolean {
    return this.originalError instanceof TypeError ||
           (this.originalError as any)?.code === 'ECONNREFUSED'
  }
}

/**
 * Error thrown when a network operation fails
 */
export class NetworkError extends Error {
  constructor(message: string) {
    super(`Network error: ${message}`)
    this.name = 'NetworkError'
  }
}

/**
 * Error thrown when backend is unavailable
 */
export class BackendUnavailableError extends Error {
  constructor() {
    super('Backend is unavailable. Please check if the application is running.')
    this.name = 'BackendUnavailableError'
  }
}

/**
 * Error thrown when a timeout occurs
 */
export class TimeoutError extends Error {
  constructor(command: string, timeoutMs: number) {
    super(`Command '${command}' timed out after ${timeoutMs}ms`)
    this.name = 'TimeoutError'
  }
}

/**
 * Error thrown when validation fails
 */
export class ValidationError extends Error {
  constructor(field: string, message: string) {
    super(`Validation failed for '${field}': ${message}`)
    this.name = 'ValidationError'
  }
}

/**
 * Helper function to format error messages
 */
function formatError(error: unknown): string {
  if (error instanceof Error) {
    return error.message
  }
  if (typeof error === 'string') {
    return error
  }
  return String(error)
}

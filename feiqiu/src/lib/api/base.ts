/**
 * API Base Infrastructure
 *
 * Provides base functionality for all API modules including error handling,
 * type-safe command invocation, and common utilities.
 */

import { invoke } from '@tauri-apps/api/core'
import type {
  PeerDto,
  MessageDto,
  ConfigDto,
  TaskDto,
  SystemInfoDto,
  NetworkStatusDto,
  PeerStatsDto,
} from './types'
import {
  IpcError,
  NetworkError,
  BackendUnavailableError,
  TimeoutError,
} from './errors'

/**
 * Default timeout for IPC commands (in milliseconds)
 */
const DEFAULT_TIMEOUT = 30000

/**
 * Invokes a Tauri command with error handling and type safety
 *
 * @param command - The IPC command name
 * @param args - Arguments to pass to the command
 * @param timeoutMs - Optional timeout in milliseconds
 * @returns Promise<T> with the command result
 * @throws IpcError if the command fails
 * @throws TimeoutError if the command times out
 */
export async function invokeCommand<T>(
  command: string,
  args?: Record<string, unknown>,
  timeoutMs: number = DEFAULT_TIMEOUT
): Promise<T> {
  try {
    // Create timeout promise
    const timeoutPromise = new Promise<never>((_, reject) => {
      setTimeout(() => {
        reject(new TimeoutError(command, timeoutMs))
      }, timeoutMs)
    })

    // Race between invoke and timeout
    const result = await Promise.race([
      invoke<T>(command, args),
      timeoutPromise,
    ])

    return result
  } catch (error) {
    // Handle specific error types
    if (error instanceof TimeoutError) {
      throw error
    }

    // Check for backend unavailability
    const errorMessage = error instanceof Error ? error.message : String(error)
    if (
      errorMessage.includes('not been registered') ||
      errorMessage.includes('not implemented') ||
      errorMessage.includes('unknown command')
    ) {
      throw new BackendUnavailableError()
    }

    // Check for network errors
    if (
      errorMessage.includes('network') ||
      errorMessage.includes('ECONNREFUSED') ||
      errorMessage.includes('ENOTFOUND')
    ) {
      throw new NetworkError(errorMessage)
    }

    // Wrap in IpcError
    throw new IpcError(command, error)
  }
}

/**
 * Checks if the backend is available by invoking a simple command
 *
 * @returns true if backend is available
 */
export async function isBackendAvailable(): Promise<boolean> {
  try {
    await invokeCommand<object>('get_system_info', undefined, 5000)
    return true
  } catch {
    return false
  }
}

/**
 * Creates a debounced function that delays invoking func until after wait milliseconds
 *
 * @param func - The function to debounce
 * @param wait - Milliseconds to wait
 * @returns Debounced function
 */
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeoutId: ReturnType<typeof setTimeout> | null = null

  return function (this: any, ...args: Parameters<T>) {
    const context = this
    clearTimeout(timeoutId!)
    timeoutId = setTimeout(() => {
      func.apply(context, args)
    }, wait)
  }
}

/**
 * Creates a throttled function that only invokes func at most once per every wait milliseconds
 *
 * @param func - The function to throttle
 * @param wait - Milliseconds to wait between invocations
 * @returns Throttled function
 */
export function throttle<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let inThrottle = false

  return function (this: any, ...args: Parameters<T>) {
    const context = this
    if (!inThrottle) {
      func.apply(context, args)
      inThrottle = true
      setTimeout(() => {
        inThrottle = false
      }, wait)
    }
  }
}

/**
 * Retries an async operation with exponential backoff
 *
 * @param fn - The async function to retry
 * @param maxRetries - Maximum number of retries
 * @param baseDelayMs - Base delay between retries in milliseconds
 * @returns Promise<T> with the result
 */
export async function retryWithBackoff<T>(
  fn: () => Promise<T>,
  maxRetries: number = 3,
  baseDelayMs: number = 1000
): Promise<T> {
  for (let attempt = 0; attempt <= maxRetries; attempt++) {
    try {
      return await fn()
    } catch (error) {
      if (attempt === maxRetries) {
        throw error
      }

      // Exponential backoff: 1000ms, 2000ms, 4000ms, etc.
      const delay = baseDelayMs * Math.pow(2, attempt)
      await new Promise(resolve => setTimeout(resolve, delay))
    }
  }

  // This should never be reached, but TypeScript needs it
  throw new Error('Max retries exceeded')
}

/**
 * Formats an error for logging
 *
 * @param error - The error to format
 * @returns Formatted error string
 */
export function formatError(error: unknown): string {
  if (error instanceof Error) {
    return `${error.name}: ${error.message}`
  }
  return String(error)
}

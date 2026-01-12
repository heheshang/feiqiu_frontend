/**
 * useFileTransfers Hook
 *
 * Custom hook for fetching and managing file transfer data from the backend.
 * Provides real-time updates when transfer requests are received, progress changes, or transfers complete/fail.
 */

import { useState, useEffect, useCallback, useRef } from 'react'
import { transfersApi } from '@/lib/api'
import { eventsManager, onEvent } from '@/lib/events'
import type { FileTransfer } from '@/lib/converters'
import type {
  FileTransferRequestedEvent,
  FileTransferProgressEvent,
  FileTransferCompletedEvent,
  FileTransferFailedEvent,
} from '@/lib/events'

/**
 * Hook return value
 */
export interface UseFileTransfersResult {
  /** Array of file transfers */
  transfers: FileTransfer[]
  /** Loading state */
  isLoading: boolean
  /** Error state */
  error: Error | null
  /** Statistics about transfers */
  stats: {
    totalTransfers: number
    activeTransfers: number
    pendingTransfers: number
    completedTransfers: number
    failedTransfers: number
    cancelledTransfers: number
  } | null
  /** Function to manually refresh transfers */
  refresh: () => Promise<void>
  /** Function to accept a transfer */
  acceptTransfer: (transferId: string) => Promise<void>
  /** Function to reject a transfer */
  rejectTransfer: (transferId: string) => Promise<void>
  /** Function to cancel a transfer */
  cancelTransfer: (transferId: string) => Promise<void>
}

/**
 * Options for the useFileTransfers hook
 */
export interface UseFileTransfersOptions {
  /** Transfer filters to apply */
  filters?: {
    peerIp?: string
    direction?: 'incoming' | 'outgoing'
    status?: 'pending' | 'active' | 'completed' | 'failed' | 'cancelled'
    after?: number
    before?: number
    limit?: number
  }
  /** Whether to fetch transfers on mount (default: true) */
  enabled?: boolean
  /** Polling interval in milliseconds (default: 3000) */
  refreshInterval?: number
  /** Whether to subscribe to real-time events (default: true) */
  subscribeToEvents?: boolean
}

/**
 * Hook for fetching and managing file transfer data
 *
 * @param options - Hook options
 * @returns Transfer data and management functions
 *
 * @example
 * ```tsx
 * function FileTransferList() {
 *   const { transfers, acceptTransfer, rejectTransfer, isLoading } = useFileTransfers()
 *
 *   if (isLoading) return <div>Loading...</div>
 *
 *   return (
 *     <ul>
 *       {transfers.map(transfer => (
 *         <li key={transfer.id}>
 *           {transfer.fileName} - {transfer.status}
 *           {transfer.status === 'waiting' && (
 *             <>
 *               <button onClick={() => acceptTransfer(transfer.id)}>Accept</button>
 *               <button onClick={() => rejectTransfer(transfer.id)}>Reject</button>
 *             </>
 *           )}
 *         </li>
 *       ))}
 *     </ul>
 *   )
 * }
 * ```
 */
export function useFileTransfers(options: UseFileTransfersOptions = {}): UseFileTransfersResult {
  const {
    filters,
    enabled = true,
    refreshInterval = 3000,
    subscribeToEvents = true,
  } = options

  const [transfers, setTransfers] = useState<FileTransfer[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<Error | null>(null)
  const [stats, setStats] = useState<UseFileTransfersResult['stats']>(null)

  // Use ref to track if component is mounted
  const isMountedRef = useRef(true)
  const pollingIntervalRef = useRef<ReturnType<typeof setInterval> | null>(null)

  /**
   * Fetch transfers from backend
   */
  const fetchTransfers = useCallback(async () => {
    if (!enabled || !isMountedRef.current) {
      return
    }

    setIsLoading(true)
    setError(null)

    try {
      const [transfersData, statsData] = await Promise.all([
        transfersApi.getFileTransfers(filters),
        transfersApi.getTransferStats(),
      ])

      if (isMountedRef.current) {
        setTransfers(transfersData)
        setStats(statsData)
      }
    } catch (err) {
      if (isMountedRef.current) {
        setError(err instanceof Error ? err : new Error(String(err)))
        console.error('[useFileTransfers] Failed to fetch transfers:', err)
      }
    } finally {
      if (isMountedRef.current) {
        setIsLoading(false)
      }
    }
  }, [enabled, filters])

  /**
   * Manually refresh transfers
   */
  const refresh = useCallback(async () => {
    await fetchTransfers()
  }, [fetchTransfers])

  /**
   * Accept a file transfer
   */
  const acceptTransfer = useCallback(async (transferId: string): Promise<void> => {
    try {
      await transfersApi.acceptFileTransfer(transferId)

      // Update local state optimistically
      if (isMountedRef.current) {
        setTransfers((prev) =>
          prev.map((t) =>
            t.id === transferId ? { ...t, status: 'transferring' as any } : t
          )
        )
      }
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setError(error)
      throw error
    }
  }, [])

  /**
   * Reject a file transfer
   */
  const rejectTransfer = useCallback(async (transferId: string): Promise<void> => {
    try {
      await transfersApi.rejectFileTransfer(transferId)

      // Update local state
      if (isMountedRef.current) {
        setTransfers((prev) =>
          prev.map((t) =>
            t.id === transferId ? { ...t, status: 'cancelled' as any } : t
          )
        )
      }
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setError(error)
      throw error
    }
  }, [])

  /**
   * Cancel a file transfer
   */
  const cancelTransfer = useCallback(async (transferId: string): Promise<void> => {
    try {
      await transfersApi.cancelFileTransfer(transferId)

      // Update local state
      if (isMountedRef.current) {
        setTransfers((prev) =>
          prev.map((t) =>
            t.id === transferId ? { ...t, status: 'cancelled' as any } : t
          )
        )
      }
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setError(error)
      throw error
    }
  }, [])

  /**
   * Setup event listeners for real-time updates
   */
  useEffect(() => {
    if (!subscribeToEvents || !enabled) {
      return
    }

    const subscriptions: ReturnType<typeof onEvent>[] = []

    // Listen for new transfer requests
    subscriptions.push(
      onEvent<FileTransferRequestedEvent>('file_transfer_requested', (event) => {
        const newTransfer = event.transfer as any

        // Check if the transfer matches our filters
        if (filters?.peerIp && newTransfer.peerIp !== filters.peerIp) {
          return
        }
        if (filters?.direction && newTransfer.direction !== filters.direction) {
          return
        }

        setTransfers((prev) => [...prev, newTransfer])
      })
    )

    // Listen for transfer progress updates
    subscriptions.push(
      onEvent<FileTransferProgressEvent>('file_transfer_progress', (event) => {
        setTransfers((prev) =>
          prev.map((t) =>
            t.id === event.transfer_id
              ? {
                  ...t,
                  progress: event.progress,
                  // Could calculate transferSpeed and remainingTime here
                }
              : t
          )
        )
      })
    )

    // Listen for completed transfers
    subscriptions.push(
      onEvent<FileTransferCompletedEvent>('file_transfer_completed', (event) => {
        setTransfers((prev) =>
          prev.map((t) =>
            t.id === event.transfer_id
              ? {
                  ...t,
                  status: 'completed' as any,
                  progress: 100,
                  endTime: new Date().toISOString(),
                }
              : t
          )
        )
      })
    )

    // Listen for failed transfers
    subscriptions.push(
      onEvent<FileTransferFailedEvent>('file_transfer_failed', (event) => {
        setTransfers((prev) =>
          prev.map((t) =>
            t.id === event.transfer_id
              ? {
                  ...t,
                  status: 'failed' as any,
                  errorMessage: event.error,
                }
              : t
          )
        )
      })
    )

    // Start the events manager if not already started
    eventsManager.start().catch(console.error)

    return () => {
      subscriptions.forEach((sub) => sub.remove())
    }
  }, [subscribeToEvents, enabled, filters])

  /**
   * Fetch transfers on mount
   */
  useEffect(() => {
    fetchTransfers()
  }, [fetchTransfers])

  /**
   * Setup polling for periodic refresh
   */
  useEffect(() => {
    if (!enabled || refreshInterval <= 0) {
      return
    }

    pollingIntervalRef.current = setInterval(() => {
      fetchTransfers()
    }, refreshInterval)

    return () => {
      if (pollingIntervalRef.current) {
        clearInterval(pollingIntervalRef.current)
        pollingIntervalRef.current = null
      }
    }
  }, [enabled, refreshInterval, fetchTransfers])

  /**
   * Cleanup on unmount
   */
  useEffect(() => {
    isMountedRef.current = true

    return () => {
      isMountedRef.current = false
    }
  }, [])

  return {
    transfers,
    isLoading,
    error,
    stats,
    refresh,
    acceptTransfer,
    rejectTransfer,
    cancelTransfer,
  }
}

/**
 * Hook for getting active transfers
 * Convenience wrapper around useFileTransfers
 *
 * @param options - Additional hook options
 * @returns Transfer data and management functions
 */
export function useActiveTransfers(
  options?: Omit<UseFileTransfersOptions, 'filters'>
): UseFileTransfersResult {
  const result = useFileTransfers(options)

  // Filter for only active/transferring transfers
  return {
    ...result,
    transfers: result.transfers.filter((t) => t.status === 'transferring'),
  }
}

/**
 * Hook for getting pending transfers
 * Convenience wrapper around useFileTransfers
 *
 * @param options - Additional hook options
 * @returns Transfer data and management functions
 */
export function usePendingTransfers(
  options?: Omit<UseFileTransfersOptions, 'filters'>
): UseFileTransfersResult {
  const result = useFileTransfers(options)

  // Filter for only waiting/pending transfers
  return {
    ...result,
    transfers: result.transfers.filter((t) => t.status === 'waiting'),
  }
}

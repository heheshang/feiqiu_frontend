/**
 * useFileTransfers Hook
 *
 * Custom hook for fetching and managing file transfer data from backend.
 * Provides real-time updates when transfer requests are received, progress changes, or transfers complete/fail.
 */

import { useEffect, useCallback, useRef } from 'react'
import { transfersApi } from '@/lib/api'
import { eventsManager, onEvent } from '@/lib/events'
import { toFrontendTransfer } from '@/lib/converters'
import type { FileTransfer } from '@/lib/converters'
import type {
  FileTransferRequestedEvent,
  FileTransferProgressEvent,
  FileTransferCompletedEvent,
  FileTransferFailedEvent,
} from '@/lib/events'
import { useTransfersStore } from '@/stores/transfersStore'

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

  const transfers = useTransfersStore((state) => state.transfers)
  const isLoading = useTransfersStore((state) => state.isLoading)
  const error = useTransfersStore((state) => state.error)
  const stats = useTransfersStore((state) => state.stats)

  const setTransfers = useTransfersStore((state) => state.setTransfers)
  const setTransfersLoading = useTransfersStore((state) => state.setTransfersLoading)
  const setTransfersError = useTransfersStore((state) => state.setTransfersError)
  const setTransfersStats = useTransfersStore((state) => state.setTransfersStats)
  const updateTransfer = useTransfersStore((state) => state.updateTransfer)
  const addTransfer = useTransfersStore((state) => state.addTransfer)

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

    setTransfersLoading(true)
    setTransfersError(null)

    try {
      const [transfersData, statsData] = await Promise.all([
        transfersApi.getFileTransfers(filters),
        transfersApi.getTransferStats(),
      ])

      if (isMountedRef.current) {
        setTransfers(transfersData)
        setTransfersStats(statsData)
      }
    } catch (err) {
      if (isMountedRef.current) {
        const error = err instanceof Error ? err : new Error(String(err))
        setTransfersError(error)
        console.error('[useFileTransfers] Failed to fetch transfers:', err)
      }
    } finally {
      if (isMountedRef.current) {
        setTransfersLoading(false)
      }
    }
  }, [enabled, filters, setTransfersLoading, setTransfersError, setTransfers, setTransfersStats])

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

      if (isMountedRef.current) {
        updateTransfer(transferId, { status: 'transferring' as any })
      }
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setTransfersError(error)
      throw error
    }
  }, [updateTransfer, setTransfersError])

  /**
   * Reject a file transfer
   */
  const rejectTransfer = useCallback(async (transferId: string): Promise<void> => {
    try {
      await transfersApi.rejectFileTransfer(transferId)

      if (isMountedRef.current) {
        updateTransfer(transferId, { status: 'cancelled' as any })
      }
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setTransfersError(error)
      throw error
    }
  }, [updateTransfer, setTransfersError])

  /**
   * Cancel a file transfer
   */
  const cancelTransfer = useCallback(async (transferId: string): Promise<void> => {
    try {
      await transfersApi.cancelFileTransfer(transferId)

      if (isMountedRef.current) {
        updateTransfer(transferId, { status: 'cancelled' as any })
      }
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setTransfersError(error)
      throw error
    }
  }, [updateTransfer, setTransfersError])

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
        const transferDto = event.transfer as any

        if (filters?.peerIp && transferDto.peer_ip !== filters.peerIp) {
          return
        }
        if (filters?.direction && transferDto.direction !== filters.direction) {
          return
        }

        const newTransfer = toFrontendTransfer(transferDto)
        addTransfer(newTransfer)
      })
    )

    // Listen for transfer progress updates
    subscriptions.push(
      onEvent<FileTransferProgressEvent>('file_transfer_progress', (event) => {
        updateTransfer(event.transfer_id, { progress: event.progress })
      })
    )

    // Listen for completed transfers
    subscriptions.push(
      onEvent<FileTransferCompletedEvent>('file_transfer_completed', (event) => {
        updateTransfer(event.transfer_id, {
          status: 'completed' as any,
          progress: 100,
          endTime: new Date().toISOString(),
        })
      })
    )

    // Listen for failed transfers
    subscriptions.push(
      onEvent<FileTransferFailedEvent>('file_transfer_failed', (event) => {
        updateTransfer(event.transfer_id, {
          status: 'failed' as any,
          errorMessage: event.error,
        })
      })
    )

    // Start events manager if not already started
    eventsManager.start().catch(console.error)

    return () => {
      subscriptions.forEach((sub) => sub.remove())
    }
  }, [subscribeToEvents, enabled, filters, addTransfer, updateTransfer])

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

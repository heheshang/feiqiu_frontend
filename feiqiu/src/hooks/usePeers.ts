/**
 * usePeers Hook
 *
 * Custom hook for fetching and managing peer data from the backend.
 * Provides real-time updates when peers are discovered, status changes, or lost.
 */

import { useState, useEffect, useCallback, useRef } from 'react'
import { peersApi } from '@/lib/api'
import { eventsManager, onEvent } from '@/lib/events'
import type { Peer } from '@/lib/converters'
import type { PeerDiscoveredEvent, PeerStatusChangedEvent, PeerLostEvent } from '@/lib/events'

/**
 * Hook return value
 */
export interface UsePeersResult {
  /** Array of all peers */
  peers: Peer[]
  /** Array of only online peers */
  onlinePeers: Peer[]
  /** Loading state */
  isLoading: boolean
  /** Error state */
  error: Error | null
  /** Statistics about peers */
  stats: {
    totalPeers: number
    onlinePeers: number
    offlinePeers: number
    awayPeers: number
  } | null
  /** Function to manually refresh peers */
  refresh: () => Promise<void>
}

/**
 * Options for the usePeers hook
 */
export interface UsePeersOptions {
  /** Whether to fetch peers on mount (default: true) */
  enabled?: boolean
  /** Polling interval in milliseconds (default: 5000) */
  refreshInterval?: number
  /** Whether to subscribe to real-time events (default: true) */
  subscribeToEvents?: boolean
}

/**
 * Hook for fetching and managing peer data
 *
 * @param options - Hook options
 * @returns Peer data and management functions
 *
 * @example
 * ```tsx
 * function PeerList() {
 *   const { peers, isLoading, error, refresh } = usePeers()
 *
 *   if (isLoading) return <div>Loading...</div>
 *   if (error) return <div>Error: {error.message}</div>
 *
 *   return (
 *     <ul>
 *       {peers.map(peer => (
 *         <li key={peer.id}>{peer.name} - {peer.status}</li>
 *       ))}
 *     </ul>
 *   )
 * }
 * ```
 */
export function usePeers(options: UsePeersOptions = {}): UsePeersResult {
  const {
    enabled = true,
    refreshInterval = 5000,
    subscribeToEvents = true,
  } = options

  const [peers, setPeers] = useState<Peer[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<Error | null>(null)
  const [stats, setStats] = useState<UsePeersResult['stats']>(null)

  // Use ref to track if component is mounted
  const isMountedRef = useRef(true)
  const pollingIntervalRef = useRef<ReturnType<typeof setInterval> | null>(null)

  /**
   * Fetch peers from backend
   */
  const fetchPeers = useCallback(async () => {
    if (!enabled || !isMountedRef.current) {
      return
    }

    setIsLoading(true)
    setError(null)

    try {
      const [peersData, statsData] = await Promise.all([
        peersApi.getPeers(),
        peersApi.getPeerStats(),
      ])

      if (isMountedRef.current) {
        setPeers(peersData)
        setStats(statsData)
      }
    } catch (err) {
      if (isMountedRef.current) {
        setError(err instanceof Error ? err : new Error(String(err)))
        console.error('[usePeers] Failed to fetch peers:', err)
      }
    } finally {
      if (isMountedRef.current) {
        setIsLoading(false)
      }
    }
  }, [enabled])

  /**
   * Manually refresh peers
   */
  const refresh = useCallback(async () => {
    await fetchPeers()
  }, [fetchPeers])

  /**
   * Setup event listeners for real-time updates
   */
  useEffect(() => {
    if (!subscribeToEvents || !enabled) {
      return
    }

    const subscriptions: ReturnType<typeof onEvent>[] = []

    // Listen for new peers
    subscriptions.push(
      onEvent<PeerDiscoveredEvent>('peer_discovered', (event) => {
        setPeers((prev) => {
          // Check if peer already exists
          if (prev.some((p) => p.ip === event.peer.ip)) {
            return prev
          }
          return [...prev, event.peer as any]
        })
      })
    )

    // Listen for peer status changes
    subscriptions.push(
      onEvent<PeerStatusChangedEvent>('peer_status_changed', (event) => {
        setPeers((prev) =>
          prev.map((peer) =>
            peer.ip === event.peer_ip
              ? { ...peer, status: event.new_status as any }
              : peer
          )
        )
      })
    )

    // Listen for lost peers
    subscriptions.push(
      onEvent<PeerLostEvent>('peer_lost', (event) => {
        setPeers((prev) =>
          prev.map((peer) =>
            peer.ip === event.peer_ip
              ? { ...peer, status: 'offline' as any }
              : peer
          )
        )
      })
    )

    // Start the events manager if not already started
    eventsManager.start().catch(console.error)

    return () => {
      subscriptions.forEach((sub) => sub.remove())
    }
  }, [subscribeToEvents, enabled])

  /**
   * Fetch peers on mount
   */
  useEffect(() => {
    fetchPeers()
  }, [fetchPeers])

  /**
   * Setup polling for periodic refresh
   */
  useEffect(() => {
    if (!enabled || refreshInterval <= 0) {
      return
    }

    pollingIntervalRef.current = setInterval(() => {
      fetchPeers()
    }, refreshInterval)

    return () => {
      if (pollingIntervalRef.current) {
        clearInterval(pollingIntervalRef.current)
        pollingIntervalRef.current = null
      }
    }
  }, [enabled, refreshInterval, fetchPeers])

  /**
   * Cleanup on unmount
   */
  useEffect(() => {
    isMountedRef.current = true

    return () => {
      isMountedRef.current = false
    }
  }, [])

  /**
   * Compute derived state
   */
  const onlinePeers = peers.filter((p) => p.status === 'online')

  return {
    peers,
    onlinePeers,
    isLoading,
    error,
    stats,
    refresh,
  }
}

/**
 * usePeers Hook
 *
 * Custom hook for fetching and managing peer data from backend.
 * Provides real-time updates when peers are discovered, status changes, or lost.
 */

import { useEffect, useCallback, useRef } from 'react'
import { peersApi } from '@/lib/api'
import { eventsManager, onEvent } from '@/lib/events'
import { toFrontendPeer } from '@/lib/converters'
import type { Peer } from '@/lib/converters'
import type { PeerDiscoveredEvent, PeerStatusChangedEvent, PeerLostEvent } from '@/lib/events'
import { usePeersStore } from '@/stores/peersStore'

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

  const peers = usePeersStore((state) => state.peers)
  const isLoading = usePeersStore((state) => state.isLoading)
  const error = usePeersStore((state) => state.error)
  const stats = usePeersStore((state) => state.stats)

  const setPeers = usePeersStore((state) => state.setPeers)
  const setPeersLoading = usePeersStore((state) => state.setPeersLoading)
  const setPeersError = usePeersStore((state) => state.setPeersError)
  const setPeersStats = usePeersStore((state) => state.setPeersStats)
  const addPeer = usePeersStore((state) => state.addPeer)
  const updatePeer = usePeersStore((state) => state.updatePeer)

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

    setPeersLoading(true)
    setPeersError(null)

    try {
      const [peersData, statsData] = await Promise.all([
        peersApi.getPeers(),
        peersApi.getPeerStats(),
      ])

      if (isMountedRef.current) {
        setPeers(peersData)
        setPeersStats(statsData)
      }
    } catch (err) {
      if (isMountedRef.current) {
        const error = err instanceof Error ? err : new Error(String(err))
        setPeersError(error)
        console.error('[usePeers] Failed to fetch peers:', err)
      }
    } finally {
      if (isMountedRef.current) {
        setPeersLoading(false)
      }
    }
  }, [enabled, setPeersLoading, setPeersError, setPeers, setPeersStats])

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
        const peerDto = {
          ...event.peer,
          displayName: event.peer.display_name,
          lastSeen: event.peer.last_seen,
        }
        const peer = toFrontendPeer(peerDto as any)
        addPeer(peer)
      })
    )

    // Listen for peer status changes
    subscriptions.push(
      onEvent<PeerStatusChangedEvent>('peer_status_changed', (event) => {
        updatePeer(event.peer_ip, { status: event.new_status as any })
      })
    )

    // Listen for lost peers
    subscriptions.push(
      onEvent<PeerLostEvent>('peer_lost', (event) => {
        updatePeer(event.peer_ip, { status: 'offline' as any })
      })
    )

    // Start events manager if not already started
    eventsManager.start().catch(console.error)

    return () => {
      subscriptions.forEach((sub) => sub.remove())
    }
  }, [subscribeToEvents, enabled, addPeer, updatePeer])

  /**
   * Fetch peers on mount
   */
  useEffect(() => {
    fetchPeers()
  }, [fetchPeers])



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

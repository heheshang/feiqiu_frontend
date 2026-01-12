/**
 * useRealtimeEvents Hook
 *
 * Custom hook for subscribing to real-time events from the backend.
 * Provides a convenient way to listen to specific event types within components.
 */

import { useEffect, useCallback, useRef } from 'react'
import { eventsManager, onEvent } from '@/lib/events'
import type { Event, EventListener, EventListenerOptions } from '@/lib/events'

/**
 * Hook return value
 */
export interface UseRealtimeEventsResult {
  /** Whether the events manager is currently polling */
  isPolling: boolean
  /** Number of active listeners */
  listenerCount: number
  /** Function to manually start polling */
  start: () => Promise<void>
  /** Function to manually stop polling */
  stop: () => void
}

/**
 * Options for the useRealtimeEvents hook
 */
export interface UseRealtimeEventsOptions {
  /** Whether to automatically start polling on mount (default: true) */
  enabled?: boolean
}

/**
 * Hook for subscribing to real-time events
 *
 * @param eventType - The type of event to listen for
 * @param listener - The listener function
 * @param options - Additional options
 * @returns Object with polling state and control functions
 *
 * @example
 * ```tsx
 * function MessageNotifications() {
 *   const { isPolling } = useRealtimeEvents('message_received', (event) => {
 *     toast(`New message from ${event.message.sender_name}`)
 *   })
 *
 *   return <div>{isPolling ? 'Listening...' : 'Stopped'}</div>
 * }
 * ```
 */
export function useRealtimeEvents<T extends Event = Event>(
  eventType: string,
  listener: EventListener<T>,
  options?: EventListenerOptions & UseRealtimeEventsOptions
): UseRealtimeEventsResult {
  const { once, filter, enabled = true } = options || {}

  const subscriptionRef = useRef<ReturnType<typeof onEvent> | null>(null)
  const isPollingRef = useRef(eventsManager['isPolling'] || false)

  /**
   * Start polling
   */
  const start = useCallback(async () => {
    await eventsManager.start()
    isPollingRef.current = true
  }, [])

  /**
   * Stop polling
   */
  const stop = useCallback(() => {
    eventsManager.stop()
    isPollingRef.current = false
  }, [])

  /**
   * Get listener count
   */
  const listenerCount = eventsManager.getListenerCount()

  /**
   * Setup event listener
   */
  useEffect(() => {
    if (!enabled) {
      return
    }

    subscriptionRef.current = onEvent<T>(eventType, listener, { once, filter })

    return () => {
      subscriptionRef.current?.remove()
      subscriptionRef.current = null
    }
  }, [eventType, listener, enabled, once, filter])

  /**
   * Auto-start polling if enabled
   */
  useEffect(() => {
    if (!enabled) {
      return
    }

    eventsManager.start().catch(console.error)

    return () => {
      // Don't stop polling on unmount as other components may be using it
    }
  }, [enabled])

  return {
    isPolling: isPollingRef.current,
    listenerCount,
    start,
    stop,
  }
}

/**
 * Hook for listening to multiple event types
 *
 * @param eventListeners - Object mapping event types to listener functions
 * @param options - Additional options
 * @returns Object with polling state and control functions
 *
 * @example
 * ```tsx
 * function MultiEventComponent() {
 *   const { isPolling } = useMultipleEvents({
 *     message_received: (event) => console.log('Message:', event),
 *     file_transfer_requested: (event) => console.log('Transfer:', event),
 *   })
 *
 *   return <div>{isPolling ? 'Active' : 'Inactive'}</div>
 * }
 * ```
 */
export function useMultipleEvents<T extends Event = Event>(
  eventListeners: Record<string, EventListener<T>>,
  options?: UseRealtimeEventsOptions
): UseRealtimeEventsResult {
  const { enabled = true } = options || {}

  const subscriptionsRef = useRef<ReturnType<typeof onEvent>[]>([])
  const isPollingRef = useRef(eventsManager['isPolling'] || false)

  /**
   * Start polling
   */
  const start = useCallback(async () => {
    await eventsManager.start()
    isPollingRef.current = true
  }, [])

  /**
   * Stop polling
   */
  const stop = useCallback(() => {
    eventsManager.stop()
    isPollingRef.current = false
  }, [])

  /**
   * Setup event listeners
   */
  useEffect(() => {
    if (!enabled) {
      return
    }

    const subscriptions: ReturnType<typeof onEvent>[] = []

    for (const [eventType, listener] of Object.entries(eventListeners)) {
      subscriptions.push(onEvent(eventType, listener))
    }

    subscriptionsRef.current = subscriptions

    return () => {
      subscriptions.forEach((sub) => sub.remove())
      subscriptionsRef.current = []
    }
  }, [enabled, eventListeners])

  /**
   * Auto-start polling if enabled
   */
  useEffect(() => {
    if (!enabled) {
      return
    }

    eventsManager.start().catch(console.error)

    return () => {
      // Don't stop polling on unmount as other components may be using it
    }
  }, [enabled])

  /**
   * Get listener count
   */
  const listenerCount = eventsManager.getListenerCount()

  return {
    isPolling: isPollingRef.current,
    listenerCount,
    start,
    stop,
  }
}

/**
 * Hook for getting events manager state and controls
 * Does not subscribe to any specific events
 *
 * @param options - Additional options
 * @returns Object with polling state and control functions
 *
 * @example
 * ```tsx
 * function EventsStatus() {
 *   const { isPolling, listenerCount, start, stop } = useEventsManager()
 *
 *   return (
 *     <div>
 *       <p>Status: {isPolling ? 'Active' : 'Inactive'}</p>
 *       <p>Listeners: {listenerCount}</p>
 *       <button onClick={start}>Start</button>
 *       <button onClick={stop}>Stop</button>
 *     </div>
 *   )
 * }
 * ```
 */
export function useEventsManager(
  options?: UseRealtimeEventsOptions
): UseRealtimeEventsResult {
  const { enabled = true } = options || {}

  const isPollingRef = useRef(eventsManager['isPolling'] || false)

  /**
   * Start polling
   */
  const start = useCallback(async () => {
    await eventsManager.start()
    isPollingRef.current = true
  }, [])

  /**
   * Stop polling
   */
  const stop = useCallback(() => {
    eventsManager.stop()
    isPollingRef.current = false
  }, [])

  /**
   * Auto-start polling if enabled
   */
  useEffect(() => {
    if (!enabled) {
      return
    }

    eventsManager.start().catch(console.error)

    return () => {
      // Don't stop polling on unmount as other components may be using it
    }
  }, [enabled])

  /**
   * Get listener count
   */
  const listenerCount = eventsManager.getListenerCount()

  return {
    isPolling: isPollingRef.current,
    listenerCount,
    start,
    stop,
  }
}

/**
 * Hook for listening to a single event once
 * Convenience wrapper around useRealtimeEvents with once: true
 *
 * @param eventType - The type of event to listen for
 * @param listener - The listener function
 * @param options - Additional options
 * @returns Object with polling state and control functions
 *
 * @example
 * ```tsx
 * function OneTimeNotification() {
 *   useRealtimeEventOnce('peer_discovered', (event) => {
 *     toast(`New peer discovered: ${event.peer.username}`)
 *   })
 *
 *   return <div>Waiting for first peer...</div>
 * }
 * ```
 */
export function useRealtimeEventOnce<T extends Event = Event>(
  eventType: string,
  listener: EventListener<T>,
  options?: Omit<EventListenerOptions & UseRealtimeEventsOptions, 'once'>
): UseRealtimeEventsResult {
  return useRealtimeEvents<T>(eventType, listener, {
    ...options,
    once: true,
  })
}

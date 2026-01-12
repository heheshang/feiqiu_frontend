/**
 * Events Manager
 *
 * Manages real-time event streaming from the Tauri backend.
 * Uses poll_events command to get events and dispatches them to registered listeners.
 */

import { invokeCommand } from '../api/base'
import type { Event, EventListener, EventListenerOptions, EventSubscription } from './types'

/**
 * Polling interval in milliseconds (how often to check for new events)
 */
const DEFAULT_POLL_INTERVAL = 1000

/**
 * Maximum number of events to fetch in a single poll
 */
const DEFAULT_BATCH_SIZE = 100

/**
 * Event listener entry
 */
interface ListenerEntry<T extends Event = Event> {
  listener: EventListener<T>
  options: EventListenerOptions
}

/**
 * Events Manager class
 *
 * Singleton instance that manages event polling and listener dispatching.
 */
class EventsManager {
  private listeners: Map<string, ListenerEntry[]> = new Map()
  private pollTimer: ReturnType<typeof setInterval> | null = null
  private isPolling = false
  private pollInterval = DEFAULT_POLL_INTERVAL
  private batchSize = DEFAULT_BATCH_SIZE
  private lastEventId: string | null = null

  /**
   * Start polling for events
   */
  async start(): Promise<void> {
    if (this.isPolling) {
      return
    }

    this.isPolling = true
    this.poll()

    console.log('[EventsManager] Started polling for events')
  }

  /**
   * Stop polling for events
   */
  stop(): void {
    if (!this.isPolling) {
      return
    }

    if (this.pollTimer) {
      clearInterval(this.pollTimer)
      this.pollTimer = null
    }

    this.isPolling = false
    console.log('[EventsManager] Stopped polling for events')
  }

  /**
   * Poll once for events
   */
  private async poll(): Promise<void> {
    try {
      const events = await this.fetchEvents()

      for (const event of events) {
        this.dispatchEvent(event)
        this.lastEventId = (event as any).id || null
      }
    } catch (error) {
      console.error('[EventsManager] Error polling events:', error)
    } finally {
      if (this.isPolling) {
        this.pollTimer = setTimeout(() => this.poll(), this.pollInterval)
      }
    }
  }

  /**
   * Fetch events from backend using poll_events command
   */
  private async fetchEvents(): Promise<Event[]> {
    try {
      const result = await invokeCommand<any[]>('poll_events', {
        after: this.lastEventId,
        limit: this.batchSize,
      })
      return result || []
    } catch (error) {
      console.error('[EventsManager] Failed to fetch events:', error)
      return []
    }
  }

  /**
   * Dispatch an event to all registered listeners
   */
  private dispatchEvent(event: Event): void {
    const eventType = event.type
    const entries = this.listeners.get(eventType) || []

    // Create a copy of the array to avoid issues if listeners are removed during iteration
    const entriesToRemove: number[] = []

    for (let i = 0; i < entries.length; i++) {
      const entry = entries[i]
      const { listener, options } = entry

      try {
        // Check filter if provided
        if (options.filter && !options.filter(event)) {
          continue
        }

        // Call the listener
        listener(event)

        // Mark for removal if once option is set
        if (options.once) {
          entriesToRemove.push(i)
        }
      } catch (error) {
        console.error(`[EventsManager] Error in listener for ${eventType}:`, error)
      }
    }

    // Remove one-time listeners (in reverse order to maintain indices)
    for (const index of entriesToRemove.reverse()) {
      entries.splice(index, 1)
    }

    // Update the map
    if (entries.length === 0) {
      this.listeners.delete(eventType)
    } else {
      this.listeners.set(eventType, entries)
    }
  }

  /**
   * Add an event listener
   *
   * @param eventType - The type of event to listen for
   * @param listener - The listener function
   * @param options - Optional listener options
   * @returns Subscription object with remove method
   */
  on<T extends Event>(
    eventType: string,
    listener: EventListener<T>,
    options: EventListenerOptions = {}
  ): EventSubscription {
    let entries = this.listeners.get(eventType)

    if (!entries) {
      entries = []
      this.listeners.set(eventType, entries)
    }

    entries.push({ listener, options })

    // Auto-start polling if this is the first listener
    if (this.listeners.size === 1 && !this.isPolling) {
      this.start().catch(console.error)
    }

    return {
      remove: () => this.off(eventType, listener),
    }
  }

  /**
   * Remove an event listener
   *
   * @param eventType - The type of event
   * @param listener - The listener function to remove
   */
  off<T extends Event>(eventType: string, listener: EventListener<T>): void {
    const entries = this.listeners.get(eventType)

    if (!entries) {
      return
    }

    const index = entries.findIndex(entry => entry.listener === listener)

    if (index !== -1) {
      entries.splice(index, 1)

      if (entries.length === 0) {
        this.listeners.delete(eventType)
      }
    }

    // Stop polling if there are no more listeners
    if (this.listeners.size === 0 && this.isPolling) {
      this.stop()
    }
  }

  /**
   * Remove all listeners for a specific event type
   *
   * @param eventType - The type of event
   */
  offAll(eventType: string): void {
    this.listeners.delete(eventType)

    // Stop polling if there are no more listeners
    if (this.listeners.size === 0 && this.isPolling) {
      this.stop()
    }
  }

  /**
   * Add a one-time event listener
   *
   * @param eventType - The type of event to listen for
   * @param listener - The listener function
   * @param options - Optional listener options (filter still applies)
   * @returns Subscription object with remove method
   */
  once<T extends Event>(
    eventType: string,
    listener: EventListener<T>,
    options: Omit<EventListenerOptions, 'once'> = {}
  ): EventSubscription {
    return this.on(eventType, listener, { ...options, once: true })
  }

  /**
   * Set the polling interval
   *
   * @param intervalMs - Polling interval in milliseconds
   */
  setPollInterval(intervalMs: number): void {
    this.pollInterval = Math.max(100, intervalMs)
  }

  /**
   * Set the batch size for event fetching
   *
   * @param size - Maximum number of events to fetch per poll
   */
  setBatchSize(size: number): void {
    this.batchSize = Math.max(1, size)
  }

  /**
   * Get the number of active listeners
   */
  getListenerCount(): number {
    let count = 0
    for (const entries of this.listeners.values()) {
      count += entries.length
    }
    return count
  }

  /**
   * Get the number of listeners for a specific event type
   */
  getListenerCountForType(eventType: string): number {
    return this.listeners.get(eventType)?.length || 0
  }

  /**
   * Remove all listeners
   */
  removeAllListeners(): void {
    this.listeners.clear()
    this.stop()
  }
}

/**
 * Global events manager instance
 */
export const eventsManager = new EventsManager()

/**
 * Convenience function to add an event listener
 */
export function onEvent<T extends Event>(
  eventType: string,
  listener: EventListener<T>,
  options?: EventListenerOptions
): EventSubscription {
  return eventsManager.on(eventType, listener, options)
}

/**
 * Convenience function to remove an event listener
 */
export function offEvent<T extends Event>(eventType: string, listener: EventListener<T>): void {
  eventsManager.off(eventType, listener)
}

/**
 * Convenience function to add a one-time event listener
 */
export function onceEvent<T extends Event>(
  eventType: string,
  listener: EventListener<T>,
  options?: Omit<EventListenerOptions, 'once'>
): EventSubscription {
  return eventsManager.once(eventType, listener, options)
}

/**
 * Convenience function to remove all listeners for an event type
 */
export function offAllEvents(eventType: string): void {
  eventsManager.offAll(eventType)
}

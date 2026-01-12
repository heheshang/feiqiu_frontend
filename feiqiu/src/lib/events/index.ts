/**
 * Events System
 *
 * Exports all event-related functionality for real-time updates from the backend.
 *
 * Usage:
 * ```ts
 * import { onEvent, eventsManager } from '@/lib/events'
 * import type { MessageReceivedEvent } from '@/lib/events'
 *
 * // Listen for message events
 * const subscription = onEvent('message_received', (event) => {
 *   console.log('New message:', event.message)
 * })
 *
 * // Later: remove the listener
 * subscription.remove()
 *
 * // Or start/stop the manager manually
 * await eventsManager.start()
 * eventsManager.stop()
 * ```
 */

export * from './types'
export * from './manager'

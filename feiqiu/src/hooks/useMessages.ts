/**
 * useMessages Hook
 *
 * Custom hook for fetching and managing message data from backend.
 * Provides real-time updates when new messages are received or sent.
 */

import { useEffect, useCallback, useRef } from 'react'
import { messagesApi } from '@/lib/api'
import { eventsManager, onEvent } from '@/lib/events'
import { toFrontendMessage } from '@/lib/converters'
import type { Message } from '@/lib/converters'
import type { MessageReceivedEvent, MessageSentEvent } from '@/lib/events'
import { useMessagesStore } from '@/stores/messagesStore'

/**
 * Hook return value
 */
export interface UseMessagesResult {
  /** Array of messages */
  messages: Message[]
  /** Loading state */
  isLoading: boolean
  /** Error state */
  error: Error | null
  /** Function to manually refresh messages */
  refresh: () => Promise<void>
  /** Function to send a message */
  sendMessage: (content: string, receiverIp: string) => Promise<Message>
  /** Function to get messages for a specific peer */
  getMessagesByPeer: (peerIp: string, limit?: number) => Promise<Message[]>
}

/**
 * Options for the useMessages hook
 */
export interface UseMessagesOptions {
  /** Message filters to apply */
  filters?: {
    senderIp?: string
    receiverIp?: string
    after?: number
    before?: number
    limit?: number
  }
  /** Whether to fetch messages on mount (default: true) */
  enabled?: boolean
  /** Polling interval in milliseconds (default: 5000) */
  refreshInterval?: number
  /** Whether to subscribe to real-time events (default: true) */
  subscribeToEvents?: boolean
}

/**
 * Hook for fetching and managing message data
 *
 * @param options - Hook options
 * @returns Message data and management functions
 *
 * @example
 * ```tsx
 * function MessageList({ peerIp }) {
 *   const { messages, isLoading, sendMessage } = useMessages({
 *     filters: { senderIp: peerIp }
 *   })
 *
 *   const handleSend = async (content: string) => {
 *     await sendMessage(content, peerIp)
 *   }
 *
 *   if (isLoading) return <div>Loading...</div>
 *
 *   return (
 *     <ul>
 *       {messages.map(msg => (
 *         <li key={msg.id}>{msg.content}</li>
 *       ))}
 *     </ul>
 *   )
 * }
 * ```
 */
export function useMessages(options: UseMessagesOptions = {}): UseMessagesResult {
  const {
    filters,
    enabled = true,
    refreshInterval = 5000,
    subscribeToEvents = true,
  } = options

  const messages = useMessagesStore((state) => state.messages)
  const isLoading = useMessagesStore((state) => state.isLoading)
  const error = useMessagesStore((state) => state.error)

  const setMessages = useMessagesStore((state) => state.setMessages)
  const setMessagesLoading = useMessagesStore((state) => state.setMessagesLoading)
  const setMessagesError = useMessagesStore((state) => state.setMessagesError)
  const addMessage = useMessagesStore((state) => state.addMessage)
  const updateMessage = useMessagesStore((state) => state.updateMessage)

  // Use ref to track if component is mounted
  const isMountedRef = useRef(true)
  const pollingIntervalRef = useRef<ReturnType<typeof setInterval> | null>(null)

  /**
   * Fetch messages from backend
   */
  const fetchMessages = useCallback(async () => {
    if (!enabled || !isMountedRef.current) {
      return
    }

    setMessagesLoading(true)
    setMessagesError(null)

    try {
      const messagesData = await messagesApi.getMessages(filters)

      if (isMountedRef.current) {
        setMessages(messagesData)
      }
    } catch (err) {
      if (isMountedRef.current) {
        const error = err instanceof Error ? err : new Error(String(err))
        setMessagesError(error)
        console.error('[useMessages] Failed to fetch messages:', err)
      }
    } finally {
      if (isMountedRef.current) {
        setMessagesLoading(false)
      }
    }
  }, [enabled, filters, setMessagesLoading, setMessagesError, setMessages])

  /**
   * Manually refresh messages
   */
  const refresh = useCallback(async () => {
    await fetchMessages()
  }, [fetchMessages])

  /**
   * Send a message
   */
  const sendMessage = useCallback(async (
    content: string,
    receiverIp: string
  ): Promise<Message> => {
    try {
      const message = await messagesApi.sendMessage(content, receiverIp)

      // Add new message to list optimistically
      if (isMountedRef.current) {
        addMessage(message)
      }

      return message
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setMessagesError(error)
      throw error
    }
  }, [addMessage, setMessagesError])

  /**
   * Get messages for a specific peer
   */
  const getMessagesByPeer = useCallback(async (
    peerIp: string,
    limit?: number
  ): Promise<Message[]> => {
    try {
      const peerMessages = await messagesApi.getMessagesByPeer(peerIp, limit)

      // Update messages list if filter matches
      if (filters?.senderIp === peerIp && isMountedRef.current) {
        setMessages(peerMessages)
      }

      return peerMessages
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setMessagesError(error)
      throw error
    }
  }, [filters?.senderIp, setMessages, setMessagesError])

  /**
   * Setup event listeners for real-time updates
   */
  useEffect(() => {
    if (!subscribeToEvents || !enabled) {
      return
    }

    const subscriptions: ReturnType<typeof onEvent>[] = []

    // Listen for new messages
    subscriptions.push(
      onEvent<MessageReceivedEvent>('message_received', (event) => {
        const messageDto = event.message as any
        const newMessage = toFrontendMessage(messageDto)

        // Check if message matches our filters
        if (filters?.senderIp && newMessage.senderIp !== filters.senderIp) {
          return
        }
        if (filters?.receiverIp && newMessage.receiverIp !== filters.receiverIp) {
          return
        }

        addMessage(newMessage)
      })
    )

    // Listen for sent messages
    subscriptions.push(
      onEvent<MessageSentEvent>('message_sent', (event) => {
        // Update message status if it's in our list
        updateMessage(event.message_id, { status: event.status as any })
      })
    )

    // Start events manager if not already started
    eventsManager.start().catch(console.error)

    return () => {
      subscriptions.forEach((sub) => sub.remove())
    }
  }, [subscribeToEvents, enabled, filters, addMessage, updateMessage])

  /**
   * Fetch messages on mount
   */
  useEffect(() => {
    fetchMessages()
  }, [fetchMessages])

  /**
   * Setup polling for periodic refresh
   */
  useEffect(() => {
    if (!enabled || refreshInterval <= 0) {
      return
    }

    pollingIntervalRef.current = setInterval(() => {
      fetchMessages()
    }, refreshInterval)

    return () => {
      if (pollingIntervalRef.current) {
        clearInterval(pollingIntervalRef.current)
        pollingIntervalRef.current = null
      }
    }
  }, [enabled, refreshInterval, fetchMessages])

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
    messages,
    isLoading,
    error,
    refresh,
    sendMessage,
    getMessagesByPeer,
  }
}

/**
 * Hook for getting messages for a specific peer
 * Convenience wrapper around useMessages
 *
 * @param peerIp - The IP address of the peer
 * @param options - Additional hook options
 * @returns Message data and management functions
 */
export function usePeerMessages(
  peerIp: string,
  options?: Omit<UseMessagesOptions, 'filters'>
): UseMessagesResult {
  return useMessages({
    ...options,
    filters: { senderIp: peerIp },
  })
}

/**
 * Hook for getting recent messages
 * Convenience wrapper around useMessages
 *
 * @param limit - Number of recent messages to retrieve
 * @param options - Additional hook options
 * @returns Message data and management functions
 */
export function useRecentMessages(
  limit: number = 50,
  options?: Omit<UseMessagesOptions, 'filters'>
): UseMessagesResult {
  return useMessages({
    ...options,
    filters: { limit },
  })
}

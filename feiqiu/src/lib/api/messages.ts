/**
 * Messages API
 *
 * Provides type-safe wrappers for all message-related IPC commands.
 */

import { invokeCommand } from './base'
import { toFrontendMessages } from '../converters'
import type { Message } from '../converters'

/**
 * Message filters for getMessages
 */
export interface MessageFilters {
  /** Filter by sender IP address */
  senderIp?: string
  /** Filter by receiver IP address */
  receiverIp?: string
  /** Filter by minimum timestamp (i64 milliseconds) */
  after?: number
  /** Filter by maximum timestamp (i64 milliseconds) */
  before?: number
  /** Limit number of results */
  limit?: number
}

/**
 * Send a message to a peer
 *
 * @param content - The message content
 * @param receiverIp - The IP address of the receiver
 * @returns The sent message
 */
export async function sendMessage(
  content: string,
  receiverIp: string
): Promise<Message> {
  const result = await invokeCommand<any>('send_message', {
    content,
    receiverIp,
  })
  return toFrontendMessages([result])[0]
}

/**
 * Send a text message (alias for sendMessage)
 *
 * @param content - The message content
 * @param receiverIp - The IP address of the receiver
 * @returns The sent message
 */
export async function sendTextMessage(
  content: string,
  receiverIp: string
): Promise<Message> {
  return sendMessage(content, receiverIp)
}

/**
 * Gets message history with optional filters
 *
 * @param filters - Optional filters to apply
 * @returns Array of messages
 */
export async function getMessages(filters?: MessageFilters): Promise<Message[]> {
  const result = await invokeCommand<any[]>('get_messages', filters || {})
  return toFrontendMessages(result)
}

/**
 * Gets messages for a specific peer
 *
 * @param peerIp - The IP address of the peer
 * @param limit - Optional limit on number of messages
 * @returns Array of messages with this peer
 */
export async function getMessagesByPeer(
  peerIp: string,
  limit?: number
): Promise<Message[]> {
  return getMessages({ senderIp: peerIp, limit })
}

/**
 * Gets recent messages (for chat history)
 *
 * @param limit - Number of recent messages to retrieve
 * @returns Array of recent messages
 */
export async function getRecentMessages(limit: number = 50): Promise<Message[]> {
  return getMessages({ limit })
}

/**
 * Messages API object
 * Provides all message-related API methods in a single object
 */
export const messagesApi = {
  sendMessage,
  sendTextMessage,
  getMessages,
  getMessagesByPeer,
  getRecentMessages,
}

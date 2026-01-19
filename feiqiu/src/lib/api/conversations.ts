// src/lib/api/conversations.ts
/**
 * Conversation API
 *
 * Provides functions for managing conversations via Tauri IPC commands.
 */

import { invoke } from "@tauri-apps/api/core";
import type { ConversationDto, UpdateConversationInput } from "@/lib/types";

export interface UpdateConversationParams {
  id: number;
  isPinned?: boolean;
  isArchived?: boolean;
  isMuted?: boolean;
}

/**
 * Get all conversations
 *
 * @returns Promise<ConversationDto[]> - List of all conversations
 */
export async function getConversations(): Promise<ConversationDto[]> {
  return await invoke<ConversationDto[]>("get_conversations");
}

/**
 * Get or create a conversation with a peer
 *
 * @param peerIp - The IP address of the peer
 * @returns Promise<ConversationDto> - The conversation
 */
export async function getOrCreateConversation(
  peerIp: string,
): Promise<ConversationDto> {
  return await invoke<ConversationDto>("get_or_create_conversation", {
    peerIp,
  });
}

/**
 * Update conversation metadata
 *
 * @param params - Update parameters
 * @returns Promise<ConversationDto> - The updated conversation
 */
export async function updateConversation(
  params: UpdateConversationParams,
): Promise<ConversationDto> {
  return await invoke<ConversationDto>("update_conversation", {
    input: {
      id: params.id,
      isPinned: params.isPinned,
      isArchived: params.isArchived,
      isMuted: params.isMuted,
    },
  });
}

/**
 * Mark conversation as read (clear unread count)
 *
 * @param conversationId - The ID of the conversation
 */
export async function markConversationRead(
  conversationId: number,
): Promise<void> {
  await invoke("mark_conversation_read", { conversationId });
}

/**
 * Delete a conversation
 *
 * @param conversationId - The ID of the conversation
 * @param deleteMessages - Whether to also delete associated messages
 */
export async function deleteConversation(
  conversationId: number,
  deleteMessages: boolean = false,
): Promise<void> {
  await invoke("delete_conversation", {
    conversationId,
    deleteMessages,
  });
}

// Export as a default object for convenience
export const conversationsApi = {
  getConversations,
  getOrCreateConversation,
  updateConversation,
  markConversationRead,
  deleteConversation,
};

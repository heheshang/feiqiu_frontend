/**
 * Conversation Types
 *
 * Types for conversations and conversation participants.
 */

import type { ConversationType } from "./messaging";

/**
 * Conversation Participant (from backend)
 */
export interface ConversationParticipant {
  id: number;
  conversationId: number;
  peerIp: string;
  joinedAt: number;
  leftAt: number | null;
  role: string;
}

/**
 * Conversation DTO (from backend)
 */
export interface ConversationDto {
  id: number;
  type: ConversationType;
  createdAt: number;
  updatedAt: number;
  isPinned: boolean;
  isArchived: boolean;
  isMuted: boolean;
  unreadCount: number;
  lastMessageId: number | null;
  lastMessageAt: number | null;
  lastMessageContent: string | null;
  lastMessageType: string | null;
  participants: ConversationParticipant[];
}

/**
 * Input for updating a conversation
 */
export interface UpdateConversationInput {
  id: number;
  isPinned?: boolean;
  isArchived?: boolean;
  isMuted?: boolean;
}

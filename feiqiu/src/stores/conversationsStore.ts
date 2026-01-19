// src/stores/conversationsStore.ts
import { create } from "zustand";
import type { ConversationDto } from "@/lib/types";

export interface ConversationsState {
  conversations: ConversationDto[];
  activeConversationId: number | null;
  isLoading: boolean;
  error: Error | null;

  // Actions
  setConversations: (conversations: ConversationDto[]) => void;
  setActiveConversation: (conversationId: number | null) => void;
  addConversation: (conversation: ConversationDto) => void;
  updateConversation: (id: number, updates: Partial<ConversationDto>) => void;
  removeConversation: (id: number) => void;
  setConversationsLoading: (isLoading: boolean) => void;
  setConversationsError: (error: Error | null) => void;
  resetConversations: () => void;

  // Helper actions
  markAsRead: (conversationId: number) => void;
  incrementUnread: (conversationId: number) => void;
  updateLastMessage: (
    conversationId: number,
    lastMessageContent: string,
    lastMessageType: string,
    lastMessageAt: number,
  ) => void;
}

export const useConversationsStore = create<ConversationsState>((set) => ({
  conversations: [],
  activeConversationId: null,
  isLoading: false,
  error: null,

  setConversations: (conversations) => set({ conversations }),

  setActiveConversation: (conversationId) =>
    set({ activeConversationId: conversationId }),

  addConversation: (conversation) =>
    set((state) => {
      if (state.conversations.some((c) => c.id === conversation.id)) {
        return state;
      }
      return { conversations: [...state.conversations, conversation] };
    }),

  updateConversation: (id, updates) =>
    set((state) => ({
      conversations: state.conversations.map((conversation) =>
        conversation.id === id ? { ...conversation, ...updates } : conversation,
      ),
    })),

  removeConversation: (id) =>
    set((state) => ({
      conversations: state.conversations.filter(
        (conversation) => conversation.id !== id,
      ),
      activeConversationId:
        state.activeConversationId === id ? null : state.activeConversationId,
    })),

  setConversationsLoading: (isLoading) => set({ isLoading }),

  setConversationsError: (error) => set({ error }),

  resetConversations: () =>
    set({
      conversations: [],
      activeConversationId: null,
      isLoading: false,
      error: null,
    }),

  // Helper actions
  markAsRead: (conversationId) =>
    set((state) => ({
      conversations: state.conversations.map((conversation) =>
        conversation.id === conversationId
          ? { ...conversation, unreadCount: 0 }
          : conversation,
      ),
    })),

  incrementUnread: (conversationId) =>
    set((state) => ({
      conversations: state.conversations.map((conversation) =>
        conversation.id === conversationId
          ? { ...conversation, unreadCount: conversation.unreadCount + 1 }
          : conversation,
      ),
    })),

  updateLastMessage: (
    conversationId,
    lastMessageContent,
    lastMessageType,
    lastMessageAt,
  ) =>
    set((state) => ({
      conversations: state.conversations.map((conversation) =>
        conversation.id === conversationId
          ? {
              ...conversation,
              lastMessageContent,
              lastMessageType,
              lastMessageAt,
            }
          : conversation,
      ),
    })),
}));

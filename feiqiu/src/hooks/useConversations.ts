/**
 * useConversations Hook
 *
 * Custom hook for fetching and managing conversation data from backend.
 * Provides real-time updates when conversations are created or updated.
 */

import { useEffect, useCallback, useRef } from "react";
import { conversationsApi } from "@/lib/api";
import { eventsManager, onEvent } from "@/lib/events";
import type { ConversationDto } from "@/lib/types";
import type {
  ConversationCreatedEvent,
  ConversationUpdatedEvent,
} from "@/lib/events";
import { useConversationsStore } from "@/stores/conversationsStore";

/**
 * Hook return value
 */
export interface UseConversationsResult {
  /** Array of all conversations */
  conversations: ConversationDto[];
  /** Currently active conversation ID */
  activeConversationId: number | null;
  /** Loading state */
  isLoading: boolean;
  /** Error state */
  error: Error | null;
  /** Function to manually refresh conversations */
  refresh: () => Promise<void>;
  /** Function to set active conversation */
  setActiveConversation: (conversationId: number | null) => void;
  /** Function to get or create a conversation by peer IP */
  getOrCreateConversation: (peerIp: string) => Promise<ConversationDto>;
  /** Function to update conversation metadata */
  updateConversation: (
    id: number,
    updates: { isPinned?: boolean; isArchived?: boolean; isMuted?: boolean },
  ) => Promise<ConversationDto>;
  /** Function to mark conversation as read */
  markAsRead: (conversationId: number) => Promise<void>;
  /** Function to delete a conversation */
  deleteConversation: (
    conversationId: number,
    deleteMessages?: boolean,
  ) => Promise<void>;
}

/**
 * Options for the useConversations hook
 */
export interface UseConversationsOptions {
  /** Whether to fetch conversations on mount (default: true) */
  enabled?: boolean;
  /** Whether to subscribe to real-time events (default: true) */
  subscribeToEvents?: boolean;
}

/**
 * Hook for fetching and managing conversation data
 *
 * @param options - Hook options
 * @returns Conversation data and management functions
 *
 * @example
 * ```tsx
 * function ConversationList() {
 *   const { conversations, isLoading, error, refresh } = useConversations()
 *
 *   if (isLoading) return <div>Loading...</div>
 *   if (error) return <div>Error: {error.message}</div>
 *
 *   return (
 *     <ul>
 *       {conversations.map(conv => (
 *         <li key={conv.id}>
 *           {conv.participants[0]?.peerIp} - {conv.unreadCount}
 *         </li>
 *       ))}
 *     </ul>
 *   )
 * }
 * ```
 */
export function useConversations(
  options: UseConversationsOptions = {},
): UseConversationsResult {
  const { enabled = true, subscribeToEvents = true } = options;

  const conversations = useConversationsStore((state) => state.conversations);
  const activeConversationId = useConversationsStore(
    (state) => state.activeConversationId,
  );
  const isLoading = useConversationsStore((state) => state.isLoading);
  const error = useConversationsStore((state) => state.error);

  const setConversations = useConversationsStore(
    (state) => state.setConversations,
  );
  const setConversationsLoading = useConversationsStore(
    (state) => state.setConversationsLoading,
  );
  const setConversationsError = useConversationsStore(
    (state) => state.setConversationsError,
  );
  const addConversation = useConversationsStore(
    (state) => state.addConversation,
  );
  const updateConversationStore = useConversationsStore(
    (state) => state.updateConversation,
  );
  const removeConversation = useConversationsStore(
    (state) => state.removeConversation,
  );
  const setActiveConversation = useConversationsStore(
    (state) => state.setActiveConversation,
  );
  const markAsReadStore = useConversationsStore((state) => state.markAsRead);

  // Use ref to track if component is mounted
  const isMountedRef = useRef(true);

  /**
   * Fetch conversations from backend
   */
  const fetchConversations = useCallback(async () => {
    if (!enabled || !isMountedRef.current) {
      return;
    }

    setConversationsLoading(true);
    setConversationsError(null);

    try {
      const conversationsData = await conversationsApi.getConversations();

      if (isMountedRef.current) {
        setConversations(conversationsData);
      }
    } catch (err) {
      if (isMountedRef.current) {
        const error = err instanceof Error ? err : new Error(String(err));
        setConversationsError(error);
        console.error("[useConversations] Failed to fetch conversations:", err);
      }
    } finally {
      if (isMountedRef.current) {
        setConversationsLoading(false);
      }
    }
  }, [
    enabled,
    setConversationsLoading,
    setConversationsError,
    setConversations,
  ]);

  /**
   * Manually refresh conversations
   */
  const refresh = useCallback(async () => {
    await fetchConversations();
  }, [fetchConversations]);

  /**
   * Get or create a conversation by peer IP
   */
  const getOrCreateConversation = useCallback(
    async (peerIp: string): Promise<ConversationDto> => {
      try {
        const conversation =
          await conversationsApi.getOrCreateConversation(peerIp);

        if (isMountedRef.current) {
          addConversation(conversation);
        }

        return conversation;
      } catch (err) {
        const error = err instanceof Error ? err : new Error(String(err));
        setConversationsError(error);
        throw error;
      }
    },
    [addConversation, setConversationsError],
  );

  /**
   * Update conversation metadata
   */
  const updateConversation = useCallback(
    async (
      id: number,
      updates: { isPinned?: boolean; isArchived?: boolean; isMuted?: boolean },
    ): Promise<ConversationDto> => {
      try {
        const updated = await conversationsApi.updateConversation({
          id,
          ...updates,
        });

        if (isMountedRef.current) {
          updateConversationStore(id, updated);
        }

        return updated;
      } catch (err) {
        const error = err instanceof Error ? err : new Error(String(err));
        setConversationsError(error);
        throw error;
      }
    },
    [updateConversationStore, setConversationsError],
  );

  /**
   * Mark conversation as read
   */
  const markAsRead = useCallback(
    async (conversationId: number): Promise<void> => {
      try {
        await conversationsApi.markConversationRead(conversationId);

        if (isMountedRef.current) {
          markAsReadStore(conversationId);
        }
      } catch (err) {
        const error = err instanceof Error ? err : new Error(String(err));
        setConversationsError(error);
        throw error;
      }
    },
    [markAsReadStore, setConversationsError],
  );

  /**
   * Delete a conversation
   */
  const deleteConversation = useCallback(
    async (conversationId: number, deleteMessages = false): Promise<void> => {
      try {
        await conversationsApi.deleteConversation(
          conversationId,
          deleteMessages,
        );

        if (isMountedRef.current) {
          removeConversation(conversationId);
        }
      } catch (err) {
        const error = err instanceof Error ? err : new Error(String(err));
        setConversationsError(error);
        throw error;
      }
    },
    [removeConversation, setConversationsError],
  );

  /**
   * Setup event listeners for real-time updates
   */
  useEffect(() => {
    if (!subscribeToEvents || !enabled) {
      return;
    }

    const subscriptions: ReturnType<typeof onEvent>[] = [];

    // Listen for conversation created events
    subscriptions.push(
      onEvent<ConversationCreatedEvent>(
        "conversation-created",
        async (event) => {
          console.log("[useConversations] Conversation created:", event);

          // Fetch the newly created conversation
          try {
            // Find the conversation by ID
            const existing = conversations.find(
              (c) => c.id === event.conversationId,
            );
            if (!existing) {
              // Fetch all conversations to get the new one
              const updated = await conversationsApi.getConversations();
              if (isMountedRef.current) {
                setConversations(updated);
              }
            }
          } catch (err) {
            console.error(
              "[useConversations] Failed to fetch conversation after creation:",
              err,
            );
          }
        },
      ),
    );

    // Listen for conversation updated events
    subscriptions.push(
      onEvent<ConversationUpdatedEvent>(
        "conversation-updated",
        async (event) => {
          console.log("[useConversations] Conversation updated:", event);

          // Fetch the updated conversation
          try {
            const updated = await conversationsApi.getConversations();
            if (isMountedRef.current) {
              setConversations(updated);
            }
          } catch (err) {
            console.error(
              "[useConversations] Failed to fetch conversations after update:",
              err,
            );
          }
        },
      ),
    );

    // Start events manager if not already started
    eventsManager.start().catch(console.error);

    return () => {
      subscriptions.forEach((sub) => sub.remove());
    };
  }, [subscribeToEvents, enabled, conversations, setConversations]);

  /**
   * Fetch conversations on mount
   */
  useEffect(() => {
    fetchConversations();
  }, [fetchConversations]);

  /**
   * Cleanup on unmount
   */
  useEffect(() => {
    isMountedRef.current = true;

    return () => {
      isMountedRef.current = false;
    };
  }, []);

  return {
    conversations,
    activeConversationId,
    isLoading,
    error,
    refresh,
    setActiveConversation,
    getOrCreateConversation,
    updateConversation,
    markAsRead,
    deleteConversation,
  };
}

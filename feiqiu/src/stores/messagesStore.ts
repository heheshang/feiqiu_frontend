import { create } from 'zustand'
import type { Message } from '@/lib/converters'

export interface MessagesState {
  messages: Message[]
  isLoading: boolean
  error: Error | null

  setMessages: (messages: Message[]) => void
  setMessagesLoading: (isLoading: boolean) => void
  setMessagesError: (error: Error | null) => void
  addMessage: (message: Message) => void
  updateMessage: (id: string, updates: Partial<Message>) => void
  removeMessage: (id: string) => void
  resetMessages: () => void
}

export const useMessagesStore = create<MessagesState>((set) => ({
  messages: [],
  isLoading: false,
  error: null,

  setMessages: (messages) => set({ messages }),

  setMessagesLoading: (isLoading) => set({ isLoading }),

  setMessagesError: (error) => set({ error }),

  addMessage: (message) =>
    set((state) => {
      if (state.messages.some((m) => m.id === message.id)) {
        return state
      }
      return { messages: [...state.messages, message] }
    }),

  updateMessage: (id, updates) =>
    set((state) => ({
      messages: state.messages.map((message) =>
        message.id === id ? { ...message, ...updates } : message
      ),
    })),

  removeMessage: (id) =>
    set((state) => ({
      messages: state.messages.filter((message) => message.id !== id),
    })),

  resetMessages: () =>
    set({
      messages: [],
      isLoading: false,
      error: null,
    }),
}))

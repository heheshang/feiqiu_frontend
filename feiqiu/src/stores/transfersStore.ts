import { create } from 'zustand'
import type { FileTransfer } from '@/lib/converters'

export interface TransfersState {
  transfers: FileTransfer[]
  isLoading: boolean
  error: Error | null
  stats: {
    totalTransfers: number
    activeTransfers: number
    pendingTransfers: number
    completedTransfers: number
    failedTransfers: number
    cancelledTransfers: number
  } | null

  setTransfers: (transfers: FileTransfer[]) => void
  setTransfersLoading: (isLoading: boolean) => void
  setTransfersError: (error: Error | null) => void
  setTransfersStats: (stats: TransfersState['stats']) => void
  addTransfer: (transfer: FileTransfer) => void
  updateTransfer: (id: string, updates: Partial<FileTransfer>) => void
  removeTransfer: (id: string) => void
  resetTransfers: () => void
}

export const useTransfersStore = create<TransfersState>((set) => ({
  transfers: [],
  isLoading: false,
  error: null,
  stats: null,

  setTransfers: (transfers) => set({ transfers }),

  setTransfersLoading: (isLoading) => set({ isLoading }),

  setTransfersError: (error) => set({ error }),

  setTransfersStats: (stats) => set({ stats }),

  addTransfer: (transfer) =>
    set((state) => {
      if (state.transfers.some((t) => t.id === transfer.id)) {
        return state
      }
      return { transfers: [...state.transfers, transfer] }
    }),

  updateTransfer: (id, updates) =>
    set((state) => ({
      transfers: state.transfers.map((transfer) =>
        transfer.id === id ? { ...transfer, ...updates } : transfer
      ),
    })),

  removeTransfer: (id) =>
    set((state) => ({
      transfers: state.transfers.filter((transfer) => transfer.id !== id),
    })),

  resetTransfers: () =>
    set({
      transfers: [],
      isLoading: false,
      error: null,
      stats: null,
    }),
}))

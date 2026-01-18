import { create } from 'zustand'
import type { Peer } from '@/lib/converters'

export interface PeersState {
  peers: Peer[]
  isLoading: boolean
  error: Error | null
  stats: {
    totalPeers: number
    onlinePeers: number
    offlinePeers: number
    awayPeers: number
  } | null

  setPeers: (peers: Peer[]) => void
  setPeersLoading: (isLoading: boolean) => void
  setPeersError: (error: Error | null) => void
  setPeersStats: (stats: PeersState['stats']) => void
  addPeer: (peer: Peer) => void
  updatePeer: (ip: string, updates: Partial<Peer>) => void
  removePeer: (ip: string) => void
  resetPeers: () => void
}

export const usePeersStore = create<PeersState>((set) => ({
  peers: [],
  isLoading: false,
  error: null,
  stats: null,

  setPeers: (peers) => set({ peers }),

  setPeersLoading: (isLoading) => set({ isLoading }),

  setPeersError: (error) => set({ error }),

  setPeersStats: (stats) => set({ stats }),

  addPeer: (peer) =>
    set((state) => {
      if (state.peers.some((p) => p.ip === peer.ip)) {
        return state
      }
      return { peers: [...state.peers, peer] }
    }),

  updatePeer: (ip, updates) =>
    set((state) => ({
      peers: state.peers.map((peer) =>
        peer.ip === ip ? { ...peer, ...updates } : peer
      ),
    })),

  removePeer: (ip) =>
    set((state) => ({
      peers: state.peers.filter((peer) => peer.ip !== ip),
    })),

  resetPeers: () =>
    set({
      peers: [],
      isLoading: false,
      error: null,
      stats: null,
    }),
}))

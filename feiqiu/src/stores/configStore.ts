import { create } from 'zustand'
import type { Config } from '@/lib/converters'

export interface ConfigState {
  config: Config | null
  isLoading: boolean
  error: Error | null

  setConfig: (config: Config) => void
  setConfigLoading: (isLoading: boolean) => void
  setConfigError: (error: Error | null) => void
  updateConfig: (updates: Partial<Config>) => void
  resetConfig: () => void
}

export const useConfigStore = create<ConfigState>((set) => ({
  config: null,
  isLoading: false,
  error: null,

  setConfig: (config) => set({ config }),

  setConfigLoading: (isLoading) => set({ isLoading }),

  setConfigError: (error) => set({ error }),

  updateConfig: (updates) =>
    set((state) => ({
      config: state.config ? { ...state.config, ...updates } : null,
    })),

  resetConfig: () =>
    set({
      config: null,
      isLoading: false,
      error: null,
    }),
}))

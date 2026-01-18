/**
 * useConfig Hook
 *
 * Custom hook for fetching and managing application configuration from backend.
 * Provides real-time updates when configuration changes.
 */

import { useEffect, useCallback, useRef } from 'react'
import { configApi } from '@/lib/api'
import { eventsManager, onEvent } from '@/lib/events'
import { toFrontendConfig } from '@/lib/converters'
import type { Config } from '@/lib/converters'
import type { ConfigChangedEvent } from '@/lib/events'
import { useConfigStore } from '@/stores/configStore'

/**
 * Hook return value
 */
export interface UseConfigResult {
  /** Current configuration */
  config: Config | null
  /** Loading state */
  isLoading: boolean
  /** Error state */
  error: Error | null
  /** Function to manually refresh config */
  refresh: () => Promise<void>
  /** Function to update configuration (partial update) */
  updateConfig: (config: Partial<Config>) => Promise<void>
  /** Function to reset configuration to defaults */
  resetConfig: () => Promise<Config>
  /** Function to get a single config value */
  getConfigValue: <T = any>(key: string) => Promise<T>
  /** Function to set a single config value */
  setConfigValue: (key: string, value: any) => Promise<void>
}

/**
 * Options for the useConfig hook
 */
export interface UseConfigOptions {
  /** Whether to fetch config on mount (default: true) */
  enabled?: boolean
  /** Whether to subscribe to real-time events (default: true) */
  subscribeToEvents?: boolean
}

/**
 * Hook for fetching and managing application configuration
 *
 * @param options - Hook options
 * @returns Config data and management functions
 *
 * @example
 * ```tsx
 * function Settings() {
 *   const { config, updateConfig, isLoading } = useConfig()
 *
 *   const handleUsernameChange = async (username: string) => {
 *     await updateConfig({ username })
 *   }
 *
 *   if (isLoading) return <div>Loading...</div>
 *
 *   return (
 *     <div>
 *       <input
 *         value={config?.username || ''}
 *         onChange={(e) => handleUsernameChange(e.target.value)}
 *       />
 *     </div>
 *   )
 * }
 * ```
 */
export function useConfig(options: UseConfigOptions = {}): UseConfigResult {
  const {
    enabled = true,
    subscribeToEvents = true,
  } = options

  const config = useConfigStore((state) => state.config)
  const isLoading = useConfigStore((state) => state.isLoading)
  const error = useConfigStore((state) => state.error)

  const setConfig = useConfigStore((state) => state.setConfig)
  const setConfigLoading = useConfigStore((state) => state.setConfigLoading)
  const setConfigError = useConfigStore((state) => state.setConfigError)
  const updateConfigStore = useConfigStore((state) => state.updateConfig)

  // Use ref to track if component is mounted
  const isMountedRef = useRef(true)

  /**
   * Fetch config from backend
   */
  const fetchConfig = useCallback(async () => {
    if (!enabled || !isMountedRef.current) {
      return
    }

    setConfigLoading(true)
    setConfigError(null)

    try {
      const configData = await configApi.getConfig()

      if (isMountedRef.current) {
        setConfig(configData)
      }
    } catch (err) {
      if (isMountedRef.current) {
        const error = err instanceof Error ? err : new Error(String(err))
        setConfigError(error)
        console.error('[useConfig] Failed to fetch config:', err)
      }
    } finally {
      if (isMountedRef.current) {
        setConfigLoading(false)
      }
    }
  }, [enabled, setConfigLoading, setConfigError, setConfig])

  /**
   * Manually refresh config
   */
  const refresh = useCallback(async () => {
    await fetchConfig()
  }, [fetchConfig])

  /**
   * Update configuration (partial update)
   */
  const updateConfig = useCallback(async (
    partialConfig: Partial<Config>
  ): Promise<void> => {
    try {
      await configApi.setConfig(partialConfig)
      updateConfigStore(partialConfig)
      await fetchConfig()
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setConfigError(error)
      throw error
    }
  }, [updateConfigStore, fetchConfig, setConfigError])

  /**
   * Reset configuration to defaults
   */
  const resetConfig = useCallback(async (): Promise<Config> => {
    try {
      const defaultConfig = await configApi.resetConfig()

      if (isMountedRef.current) {
        setConfig(defaultConfig)
      }

      return defaultConfig
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setConfigError(error)
      throw error
    }
  }, [setConfig, setConfigError])

  /**
   * Get a single config value
   */
  const getConfigValue = useCallback(async <T = any>(key: string): Promise<T> => {
    try {
      return await configApi.getConfigValue<T>(key)
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setConfigError(error)
      throw error
    }
  }, [setConfigError])

  /**
   * Set a single config value
   */
  const setConfigValue = useCallback(async (key: string, value: any): Promise<void> => {
    try {
      await configApi.setConfigValue(key, value)
      await fetchConfig()
    } catch (err) {
      const error = err instanceof Error ? err : new Error(String(err))
      setConfigError(error)
      throw error
    }
  }, [fetchConfig, setConfigError])

  /**
   * Setup event listeners for real-time updates
   */
  useEffect(() => {
    if (!subscribeToEvents || !enabled) {
      return
    }

    const subscriptions: ReturnType<typeof onEvent>[] = []

    // Listen for config changes
    subscriptions.push(
      onEvent<ConfigChangedEvent>('config_changed', (event) => {
        const configDto = event.config as any
        const newConfig = toFrontendConfig(configDto)
        setConfig(newConfig)
      })
    )

    // Start events manager if not already started
    eventsManager.start().catch(console.error)

    return () => {
      subscriptions.forEach((sub) => sub.remove())
    }
  }, [subscribeToEvents, enabled, setConfig])

  /**
   * Fetch config on mount
   */
  useEffect(() => {
    fetchConfig()
  }, [fetchConfig])

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
    config,
    isLoading,
    error,
    refresh,
    updateConfig,
    resetConfig,
    getConfigValue,
    setConfigValue,
  }
}

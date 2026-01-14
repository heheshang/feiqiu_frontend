/**
 * Config API
 *
 * Provides type-safe wrappers for all configuration-related IPC commands.
 */

import { invokeCommand } from './base'
import { toFrontendConfig } from '../converters'
import type { Config } from '../converters'

/**
 * Gets the current application configuration
 *
 * @returns Current configuration
 */
export async function getConfig(): Promise<Config> {
  const result = await invokeCommand<any>('get_config')
  return toFrontendConfig(result)
}

/**
 * Updates the application configuration
 * Only provided fields will be updated (partial update)
 *
 * @param config - Partial configuration object with fields to update
 * @returns void (backend returns no data on success)
 */
export async function setConfig(config: Partial<Config>): Promise<void> {
  await invokeCommand<void>('set_config', { config })
}

/**
 * Resets configuration to default values
 *
 * @returns Default configuration
 */
export async function resetConfig(): Promise<Config> {
  const result = await invokeCommand<any>('reset_config')
  return toFrontendConfig(result)
}

/**
 * Gets a single configuration value by key
 *
 * @param key - The configuration key
 * @returns The configuration value
 */
export async function getConfigValue<T = any>(key: string): Promise<T> {
  return await invokeCommand<T>('get_config_value', { key })
}

/**
 * Sets a single configuration value by key
 *
 * @param key - The configuration key
 * @param value - The value to set
 * @returns Updated configuration (optional, backend may return void)
 */
export async function setConfigValue(key: string, value: any): Promise<void> {
  await invokeCommand<void>('set_config_value', { key, value })
}

/**
 * Config API object
 * Provides all config-related API methods in a single object
 */
export const configApi = {
  getConfig,
  setConfig,
  resetConfig,
  getConfigValue,
  setConfigValue,
}

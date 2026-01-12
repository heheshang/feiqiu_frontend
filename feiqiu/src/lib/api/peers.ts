/**
 * Peers API
 *
 * Provides type-safe wrappers for all peer-related IPC commands.
 */

import { invokeCommand } from './base'
import { toFrontendPeers, toFrontendPeer } from '../converters'
import type { Peer } from '../converters'

/**
 * Gets all peers (both online and offline)
 *
 * @returns Array of all discovered peers
 */
export async function getPeers(): Promise<Peer[]> {
  const result = await invokeCommand<any[]>('get_peers')
  return toFrontendPeers(result)
}

/**
 * Gets only online peers
 *
 * @returns Array of peers with status 'online'
 */
export async function getOnlinePeers(): Promise<Peer[]> {
  const result = await invokeCommand<any[]>('get_online_peers')
  const peers = toFrontendPeers(result)
  return peers.filter(p => p.status === 'online')
}

/**
 * Gets a specific peer by IP address
 *
 * @param ip - The IP address of the peer
 * @returns The peer with the given IP
 * @throws Error if peer not found
 */
export async function getPeerByIp(ip: string): Promise<Peer> {
  const result = await invokeCommand<any>('get_peer_by_ip', { ip })
  return toFrontendPeer(result)
}

/**
 * Gets peer statistics
 *
 * @returns Statistics about peer counts by status
 */
export async function getPeerStats(): Promise<{
  totalPeers: number
  onlinePeers: number
  offlinePeers: number
  awayPeers: number
}> {
  return await invokeCommand('get_peer_stats')
}

/**
 * Peers API object
 * Provides all peer-related API methods in a single object
 */
export const peersApi = {
  getPeers,
  getOnlinePeers,
  getPeerByIp,
  getPeerStats,
}

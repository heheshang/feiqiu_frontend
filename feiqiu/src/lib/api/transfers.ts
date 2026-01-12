/**
 * File Transfers API
 *
 * Provides type-safe wrappers for all file transfer-related IPC commands.
 */

import { invokeCommand } from './base'
import { toFrontendTransfers } from '../converters'
import type { FileTransfer } from '../converters'

/**
 * File transfer filters for getFileTransfers
 */
export interface TransferFilters {
  /** Filter by peer IP address */
  peerIp?: string
  /** Filter by transfer direction */
  direction?: 'incoming' | 'outgoing'
  /** Filter by transfer status */
  status?: 'pending' | 'active' | 'completed' | 'failed' | 'cancelled'
  /** Filter by minimum timestamp (i64 milliseconds) */
  after?: number
  /** Filter by maximum timestamp (i64 milliseconds) */
  before?: number
  /** Limit number of results */
  limit?: number
}

/**
 * Accepts a file transfer
 *
 * @param transferId - The ID of the transfer to accept
 * @returns Promise that resolves when the transfer is accepted
 */
export async function acceptFileTransfer(transferId: string): Promise<void> {
  await invokeCommand<void>('accept_file_transfer', { transferId })
}

/**
 * Rejects a file transfer
 *
 * @param transferId - The ID of the transfer to reject
 * @returns Promise that resolves when the transfer is rejected
 */
export async function rejectFileTransfer(transferId: string): Promise<void> {
  await invokeCommand<void>('reject_file_transfer', { transferId })
}

/**
 * Gets file transfers with optional filters
 *
 * @param filters - Optional filters to apply
 * @returns Array of file transfers
 */
export async function getFileTransfers(filters?: TransferFilters): Promise<FileTransfer[]> {
  const result = await invokeCommand<any[]>('get_file_transfers', filters || {})
  return toFrontendTransfers(result)
}

/**
 * Gets transfers for a specific peer
 *
 * @param peerIp - The IP address of the peer
 * @param limit - Optional limit on number of transfers
 * @returns Array of transfers with this peer
 */
export async function getTransfersByPeer(
  peerIp: string,
  limit?: number
): Promise<FileTransfer[]> {
  return getFileTransfers({ peerIp, limit })
}

/**
 * Gets active transfers (currently in progress)
 *
 * @param limit - Optional limit on number of transfers
 * @returns Array of active transfers
 */
export async function getActiveTransfers(limit?: number): Promise<FileTransfer[]> {
  const all = await getFileTransfers({ limit })
  return all.filter(t => t.status === 'transferring')
}

/**
 * Gets pending transfers (waiting for acceptance)
 *
 * @param limit - Optional limit on number of transfers
 * @returns Array of pending transfers
 */
export async function getPendingTransfers(limit?: number): Promise<FileTransfer[]> {
  const all = await getFileTransfers({ limit })
  return all.filter(t => t.status === 'waiting')
}

/**
 * Cancels a file transfer
 *
 * @param transferId - The ID of the transfer to cancel
 * @returns Promise that resolves when the transfer is cancelled
 */
export async function cancelFileTransfer(transferId: string): Promise<void> {
  await invokeCommand<void>('cancel_file_transfer', { transferId })
}

/**
 * Gets file transfer statistics
 *
 * @returns Statistics about transfer counts by status
 */
export async function getTransferStats(): Promise<{
  totalTransfers: number
  activeTransfers: number
  pendingTransfers: number
  completedTransfers: number
  failedTransfers: number
  cancelledTransfers: number
}> {
  const all = await getFileTransfers()

  return {
    totalTransfers: all.length,
    activeTransfers: all.filter(t => t.status === 'transferring').length,
    pendingTransfers: all.filter(t => t.status === 'waiting').length,
    completedTransfers: all.filter(t => t.status === 'completed').length,
    failedTransfers: all.filter(t => t.status === 'failed').length,
    cancelledTransfers: all.filter(t => t.status === 'cancelled').length,
  }
}

/**
 * File Transfers API object
 * Provides all transfer-related API methods in a single object
 */
export const transfersApi = {
  acceptFileTransfer,
  rejectFileTransfer,
  getFileTransfers,
  getTransfersByPeer,
  getActiveTransfers,
  getPendingTransfers,
  cancelFileTransfer,
  getTransferStats,
}

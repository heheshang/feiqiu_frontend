/**
 * File Transfers API
 *
 * Provides type-safe wrappers for all file transfer-related IPC commands.
 */

import { invokeCommand } from './base'
import { toFrontendTransfers } from '../converters'
import type { FileTransfer } from '../converters'
import type { TaskDto } from './types'

/**
 * File transfer filters for getFileTransfers
 */
export interface TransferFilters extends Record<string, unknown> {
  /** Filter by peer IP address */
  peerIp?: string
  /** Filter by transfer direction */
  direction?: 'incoming' | 'outgoing'
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
  const result = await invokeCommand<TaskDto[]>('get_file_transfers', filters || {})
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
  const transfers = await getFileTransfers({ limit })
  return transfers.filter(t => t.status === 'transferring')
}

/**
 * Gets pending transfers (waiting for acceptance)
 *
 * @param limit - Optional limit on number of transfers
 * @returns Array of pending transfers
 */
export async function getPendingTransfers(limit?: number): Promise<FileTransfer[]> {
  const transfers = await getFileTransfers({ limit })
  return transfers.filter(t => t.status === 'waiting')
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
 * File transfer statistics
 */
export interface TransferStats {
  totalTransfers: number
  activeTransfers: number
  pendingTransfers: number
  completedTransfers: number
  failedTransfers: number
  cancelledTransfers: number
}

/**
 * Gets file transfer statistics
 *
 * @returns Statistics about transfer counts by status
 */
export async function getTransferStats(): Promise<TransferStats> {
  const transfers = await getFileTransfers()

  const stats: TransferStats = {
    totalTransfers: transfers.length,
    activeTransfers: 0,
    pendingTransfers: 0,
    completedTransfers: 0,
    failedTransfers: 0,
    cancelledTransfers: 0,
  }

  for (const transfer of transfers) {
    switch (transfer.status) {
      case 'transferring':
        stats.activeTransfers++
        break
      case 'waiting':
        stats.pendingTransfers++
        break
      case 'completed':
        stats.completedTransfers++
        break
      case 'failed':
        stats.failedTransfers++
        break
      case 'cancelled':
        stats.cancelledTransfers++
        break
    }
  }

  return stats
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

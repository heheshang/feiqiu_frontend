/**
 * Status mapping utilities
 *
 * Converts between backend and frontend status enums.
 */

/**
 * Backend peer status values (from Rust)
 */
export type BackendPeerStatus = 'online' | 'offline' | 'away'

/**
 * Frontend peer status values (extended)
 */
export type FrontendPeerStatus = 'online' | 'offline' | 'away' | 'busy'

/**
 * Backend file transfer status values
 */
export type BackendTransferStatus = 'pending' | 'active' | 'paused' | 'completed' | 'failed' | 'cancelled'

/**
 * Frontend file transfer status values
 */
export type FrontendTransferStatus = 'waiting' | 'transferring' | 'paused' | 'completed' | 'cancelled' | 'failed'

/**
 * Maps backend peer status to frontend peer status
 * @param status - Backend status
 * @returns Frontend status (busy status must be set by UI)
 */
export function mapPeerStatus(status: BackendPeerStatus): FrontendPeerStatus {
  return status as FrontendPeerStatus
}

/**
 * Maps frontend peer status to backend peer status
 * Note: 'busy' status is frontend-only, mapped to 'away' for backend
 * @param status - Frontend status
 * @returns Backend status
 */
export function toBackendPeerStatus(status: FrontendPeerStatus): BackendPeerStatus {
  if (status === 'busy') {
    return 'away'
  }
  return status as BackendPeerStatus
}

/**
 * Maps backend transfer status to frontend transfer status
 * @param status - Backend transfer status
 * @returns Frontend transfer status
 */
export function mapTransferStatus(status: BackendTransferStatus): FrontendTransferStatus {
  switch (status) {
    case 'pending':
      return 'waiting'
    case 'active':
      return 'transferring'
    case 'paused':
      return 'paused'
    case 'completed':
      return 'completed'
    case 'failed':
      return 'failed'
    case 'cancelled':
      return 'cancelled'
    default:
      return 'waiting'
  }
}

/**
 * Maps frontend transfer status to backend transfer status
 * @param status - Frontend transfer status
 * @returns Backend transfer status
 */
export function toBackendTransferStatus(status: FrontendTransferStatus): BackendTransferStatus {
  switch (status) {
    case 'waiting':
      return 'pending'
    case 'transferring':
      return 'active'
    case 'paused':
      return 'paused'
    case 'completed':
      return 'completed'
    case 'failed':
      return 'failed'
    case 'cancelled':
      return 'cancelled'
    default:
      return 'pending'
  }
}

/**
 * Checks if a peer status is considered "online"
 * @param status - Peer status to check
 * @returns true if the status is online or away (both are available states)
 */
export function isPeerAvailable(status: FrontendPeerStatus): boolean {
  return status === 'online' || status === 'away'
}

/**
 * Checks if a transfer is in progress
 * @param status - Transfer status to check
 * @returns true if the transfer is active
 */
export function isTransferActive(status: FrontendTransferStatus): boolean {
  return status === 'waiting' || status === 'transferring' || status === 'paused'
}

/**
 * Checks if a transfer has completed (successfully or failed)
 * @param status - Transfer status to check
 * @returns true if the transfer is in a terminal state
 */
export function isTransferTerminal(status: FrontendTransferStatus): boolean {
  return status === 'completed' || status === 'failed' || status === 'cancelled'
}

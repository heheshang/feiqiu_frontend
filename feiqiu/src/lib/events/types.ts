/**
 * Events System Types
 *
 * Defines all event types that can be emitted by the backend and consumed by the frontend.
 */

/**
 * Base event interface
 */
export interface BackendEvent {
  type: string;
  timestamp: number; // i64 milliseconds
}

/**
 * Peer discovered event
 * Emitted when a new peer is found on the network
 */
export interface PeerDiscoveredEvent extends BackendEvent {
  type: "peer_discovered";
  peer: {
    ip: string;
    port: number;
    username: string | null;
    hostname: string | null;
    nickname: string | null;
    avatar: string | null;
    groups: string[];
    status: string;
    display_name: string;
    last_seen: number;
  };
}

/**
 * Peer status changed event
 * Emitted when a peer's status changes (online/offline/away)
 */
export interface PeerStatusChangedEvent extends BackendEvent {
  type: "peer_status_changed";
  peer_ip: string;
  old_status: string;
  new_status: string;
}

/**
 * Peer lost event
 * Emitted when a peer goes offline or is no longer reachable
 */
export interface PeerLostEvent extends BackendEvent {
  type: "peer_lost";
  peer_ip: string;
  reason?: string;
}

/**
 * Message received event
 * Emitted when a new message is received from a peer
 */
export interface MessageReceivedEvent extends BackendEvent {
  type: "message_received";
  message: {
    id: string;
    msg_id: string;
    sender_ip: string;
    sender_name: string;
    receiver_ip: string;
    msg_type: number;
    content: string;
    is_encrypted: boolean;
    is_offline: boolean;
    sent_at: number;
    received_at: number;
    created_at: number;
  };
}

/**
 * Message sent event
 * Emitted when a message is successfully sent
 */
export interface MessageSentEvent extends BackendEvent {
  type: "message_sent";
  message_id: string;
  receiver_ip: string;
  status: "sent" | "pending" | "failed";
}

/**
 * File transfer request event
 * Emitted when a peer sends a file transfer request
 */
export interface FileTransferRequestedEvent extends BackendEvent {
  type: "file_transfer_requested";
  transfer: {
    id: string;
    direction: "incoming" | "outgoing";
    peer_ip: string;
    peer_name: string;
    file_name: string;
    file_size: number;
    file_path: string;
    status: string;
    progress: number;
    created_at: number;
    updated_at: number;
  };
}

/**
 * File transfer progress event
 * Emitted during file transfer to report progress
 */
export interface FileTransferProgressEvent extends BackendEvent {
  type: "file_transfer_progress";
  transfer_id: string;
  progress: number;
  bytes_transferred: number;
  total_bytes: number;
}

/**
 * File transfer completed event
 * Emitted when a file transfer completes successfully
 */
export interface FileTransferCompletedEvent extends BackendEvent {
  type: "file_transfer_completed";
  transfer_id: string;
  file_path: string;
}

/**
 * File transfer failed event
 * Emitted when a file transfer fails
 */
export interface FileTransferFailedEvent extends BackendEvent {
  type: "file_transfer_failed";
  transfer_id: string;
  error: string;
}

/**
 * Config changed event
 * Emitted when application configuration is updated
 */
export interface ConfigChangedEvent extends BackendEvent {
  type: "config_changed";
  config: {
    username?: string;
    hostname?: string;
    avatar?: string | null;
    status?: string;
    bind_ip?: string;
    udp_port?: number;
    tcp_port_start?: number;
    tcp_port_end?: number;
    heartbeat_interval?: number;
    peer_timeout?: number;
    encryption_enabled?: boolean;
    offline_message_retention_days?: number;
    auto_accept_files?: boolean;
    file_save_dir?: string;
    log_level?: string;
  };
}

/**
 * Network status changed event
 * Emitted when network status changes
 */
export interface NetworkStatusChangedEvent extends BackendEvent {
  type: "network_status_changed";
  status: {
    is_connected: boolean;
    bind_ip: string;
    udp_port: number;
    peers_count: number;
    active_transfers: number;
  };
}

/**
 * Error event
 * Emitted when a backend error occurs
 */
export interface ErrorEvent extends BackendEvent {
  type: "error";
  error: {
    code: string;
    message: string;
    context?: Record<string, unknown>;
  };
}

/**
 * Conversation created event
 * Emitted when a new conversation is created
 */
export interface ConversationCreatedEvent extends BackendEvent {
  type: "conversation-created";
  conversationId: number;
  conversationType: string;
}

/**
 * Conversation updated event
 * Emitted when a conversation is updated
 */
export interface ConversationUpdatedEvent extends BackendEvent {
  type: "conversation-updated";
  conversationId: number;
}

/**
 * Union type of all possible backend events
 */
export type Event =
  | PeerDiscoveredEvent
  | PeerStatusChangedEvent
  | PeerLostEvent
  | MessageReceivedEvent
  | MessageSentEvent
  | FileTransferRequestedEvent
  | FileTransferProgressEvent
  | FileTransferCompletedEvent
  | FileTransferFailedEvent
  | ConfigChangedEvent
  | NetworkStatusChangedEvent
  | ErrorEvent
  | ConversationCreatedEvent
  | ConversationUpdatedEvent;

/**
 * Event listener function type
 */
export type EventListener<T extends Event = Event> = (event: T) => void;

/**
 * Event filter function type
 * Used to selectively listen to specific events
 */
export type EventFilter<T extends Event = Event> = (event: T) => boolean;

/**
 * Event listener options
 */
export interface EventListenerOptions {
  /**
   * If true, the listener is automatically removed after being called once
   */
  once?: boolean;
  /**
   * Optional filter function to selectively process events
   */
  filter?: EventFilter;
}

/**
 * Event subscription (returned when adding a listener)
 */
export interface EventSubscription {
  /**
   * Removes the event listener
   */
  remove: () => void;
}

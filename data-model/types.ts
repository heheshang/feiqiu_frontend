// =============================================================================
// Global Data Model Types
// =============================================================================

// User Status
export type UserStatus = "online" | "away" | "busy" | "offline";

// Message Types
export type MessageType = "text" | "emoji" | "image" | "file" | "system";

// Message Status
export type MessageStatus = "sending" | "sent" | "read" | "unread" | "failed";

// Conversation Type
export type ConversationType = "single" | "group";

// Transfer Direction
export type TransferDirection = "send" | "receive";

// Transfer Status
export type TransferStatus =
  | "waiting"
  | "transferring"
  | "paused"
  | "completed"
  | "cancelled"
  | "failed";

// Connection Status
export type ConnectionStatus =
  | "connected"
  | "disconnected"
  | "connecting"
  | "error";

// Screenshot Type
export type ScreenshotType = "fullscreen" | "region" | "window";

// Screenshot Status
export type ScreenshotStatus = "draft" | "saved" | "sent";

// Annotation Type
export type AnnotationType = "arrow" | "rectangle" | "text" | "brush";

// Point for brush paths
export interface Point {
  x: number;
  y: number;
}

// Message Reaction
export interface MessageReaction {
  emoji: string;
  users: Array<{ id: string; name: string }>;
}

// Message Quote
export interface MessageQuote {
  messageId: string;
  content: string;
  senderName: string;
}

// Annotation Element
export interface Annotation {
  id: string;
  type: AnnotationType;
  color: string;
  // Arrow properties
  startX?: number;
  startY?: number;
  endX?: number;
  endY?: number;
  // Rectangle properties
  x?: number;
  y?: number;
  width?: number;
  height?: number;
  // Text properties
  content?: string;
  fontSize?: number;
  // Brush properties
  points?: Point[];
  lineWidth?: number;
}

// User (Global)
export interface User {
  id: string;
  name: string;
  avatar: string;
  status?: UserStatus;
  department?: string;
  departmentId?: string;
  position?: string;
  email?: string;
  phone?: string;
  pinyin?: string;
  signature?: string;
}

// Department
export interface Department {
  id: string;
  name: string;
  parentId: string | null;
  level: number;
  memberCount: number;
}

// Message
export interface Message {
  id: string;
  type: MessageType;
  content: string;
  timestamp: string;
  senderId: string;
  senderName: string;
  status: MessageStatus;
  reactions: MessageReaction[];
  quote?: MessageQuote;
  imageThumbnailUrl?: string;
  imageUrl?: string;
  fileUrl?: string;
  fileName?: string;
  fileSize?: number;
}

// Group
export interface Group {
  id: string;
  name: string;
  avatar: string;
  memberCount: number;
  members: User[];
}

// Conversation Participant (from backend)
export interface ConversationParticipant {
  id: number;
  conversationId: number;
  peerIp: string;
  joinedAt: number;
  leftAt: number | null;
  role: string;
}

// Conversation DTO (from backend)
export interface ConversationDto {
  id: number;
  type: ConversationType;
  createdAt: number;
  updatedAt: number;
  isPinned: boolean;
  isArchived: boolean;
  isMuted: boolean;
  unreadCount: number;
  lastMessageId: number | null;
  lastMessageAt: number | null;
  lastMessageContent: string | null;
  lastMessageType: string | null;
  participants: ConversationParticipant[];
}

// Conversation (Frontend UI Model)
export interface Conversation {
  id: string;
  type: ConversationType;
  pinned: boolean;
  unreadCount: number;
  lastMessage: {
    id: string;
    content: string;
    type: MessageType;
    timestamp: string;
    senderId: string;
    senderName: string;
  };
  participant?: User;
  group?: Group;
  messages?: Message[];
}

// File Transfer
export interface FileTransfer {
  id: string;
  direction: TransferDirection;
  senderId: string;
  receiverId: string;
  fileName: string;
  fileSize: number;
  fileType: string;
  status: TransferStatus;
  progress: number;
  transferSpeed: number;
  remainingTime: number;
  startTime: string;
  endTime: string | null;
  errorMessage?: string;
}

// Screenshot
export interface Screenshot {
  id: string;
  type: ScreenshotType;
  title: string;
  imageUrl: string;
  thumbnailUrl: string;
  createdAt: string;
  createdBy: string;
  annotations: Annotation[];
  status: ScreenshotStatus;
  sentTo?: string;
}

// Network Configuration
export interface NetworkConfig {
  id: string;
  udpPort: number;
  bindAddress: string;
  broadcastAddress: string;
  maxRetries: number;
  timeout: number;
}

// Network Status
export interface NetworkStatus {
  ipAddress: string;
  macAddress: string;
  connectionStatus: ConnectionStatus;
  listeningPort: number;
  lastSeen: string;
  onlineUsers: number;
}

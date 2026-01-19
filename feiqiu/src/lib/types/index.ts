// Types will be added in each milestone
export * from "./shell";
export * from "./contacts";
export * from "./conversations";

// Messaging types - export specific types to avoid conflicts with shell.User
export type { MessageType, MessageStatus, ConversationType } from "./messaging";
export type {
  Message,
  MessageReaction,
  MessageQuote,
  Group,
  Conversation as MessagingConversation,
} from "./messaging";

// export * from './basic-settings';
// export * from './file-transfer';
// export * from './collaboration';
// export * from './organization';

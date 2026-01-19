"use client";

import { useState, useRef, useEffect } from "react";
import { Conversation, Message, User } from "../../lib/types/messaging";
import { ConversationItem } from "./ConversationItem";
import { MessageBubble } from "./MessageBubble";
import { MessageInput } from "./MessageInput";
import { cn } from "../../lib/utils";
import {
  Search,
  MoreVertical,
  MessageSquare,
  AlertTriangle,
} from "lucide-react";
import { useConversations } from "../../hooks/useConversations";
import type { ConversationDto } from "../../lib/types/conversations";
import { toMessagingUser, type Peer } from "../../lib/converters";

interface MessagingProps {
  currentUser: User;
  peers: Peer[];
  activeConversationId: string | null;
  onConversationSelect: (id: string) => void;
  onSendMessage?: (conversationId: string, content: string) => void;
  onSendImage?: (conversationId: string, file: File) => void;
  onMessageReply?: (messageId: string) => void;
  onMessageReact?: (messageId: string, emoji: string) => void;
  onMessageRetract?: (messageId: string) => void;
}

// Convert ConversationDto to frontend Conversation type for Messaging component
function toFrontendConversation(
  dto: ConversationDto,
  peers: Peer[],
): Conversation | null {
  const participant = dto.participants[0];
  if (!participant) return null;

  const peer = peers.find((p) => p.ip === participant.peerIp);
  if (!peer) return null;

  const lastMessageTimestamp = dto.lastMessageAt
    ? new Date(dto.lastMessageAt).toISOString()
    : new Date().toISOString();

  return {
    id: participant.peerIp,
    type: dto.type,
    pinned: dto.isPinned,
    unreadCount: dto.unreadCount,
    lastMessage:
      dto.lastMessageContent && dto.lastMessageAt
        ? {
            id: dto.lastMessageId?.toString() || "",
            content: dto.lastMessageContent,
            type: (dto.lastMessageType as any) || "text",
            timestamp: lastMessageTimestamp,
            senderId: participant.peerIp,
            senderName: peer.name || peer.ip,
          }
        : undefined,
    participant: toMessagingUser(peer),
  };
}

// Convert peer IP to conversation ID for backend operations
// This is a reverse mapping that looks up the DTO by participant IP
function findConversationDtoId(
  peerIp: string,
  conversationDtos: ConversationDto[],
): number | null {
  for (const dto of conversationDtos) {
    if (dto.participants.some((p) => p.peerIp === peerIp)) {
      return dto.id;
    }
  }
  return null;
}

export function Messaging({
  currentUser,
  peers,
  activeConversationId,
  onConversationSelect,
  onSendMessage,
  onSendImage,
  onMessageReply,
  onMessageReact,
  onMessageRetract,
}: MessagingProps) {
  const [searchQuery, setSearchQuery] = useState("");
  const [showDeleteDialog, setShowDeleteDialog] = useState(false);
  const [conversationToDelete, setConversationToDelete] = useState<
    string | null
  >(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // Use conversations hook to get conversation data and management functions
  const {
    conversations: conversationDtos,
    updateConversation,
    deleteConversation,
  } = useConversations({ enabled: true, subscribeToEvents: true });

  // Convert DTOs to frontend Conversation type
  const conversations: Conversation[] = (() => {
    return conversationDtos
      .map((dto) => toFrontendConversation(dto, peers))
      .filter((c: Conversation | null): c is Conversation => c !== null);
  })();

  const activeConversation = conversations.find(
    (c) => c.id === activeConversationId,
  );

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [activeConversation?.messages]);

  const pinnedConversations = conversations.filter((c) => c.pinned);
  const regularConversations = conversations.filter((c) => !c.pinned);

  const filterConversations = (convs: Conversation[]) => {
    if (!searchQuery.trim()) return convs;
    return convs.filter((c) => {
      const name = c.type === "group" ? c.group?.name : c.participant?.name;
      return name?.toLowerCase().includes(searchQuery.toLowerCase());
    });
  };

  const displayName =
    activeConversation?.type === "group"
      ? activeConversation.group?.name
      : activeConversation?.participant?.name;

  const memberCount =
    activeConversation?.type === "group"
      ? activeConversation.group?.memberCount
      : undefined;

  const handleSendMessage = (content: string) => {
    if (activeConversationId && onSendMessage) {
      onSendMessage(activeConversationId, content);
    }
  };

  const handleSendImage = (file: File) => {
    if (activeConversationId && onSendImage) {
      onSendImage(activeConversationId, file);
    }
  };

  const handleReply = (messageId: string) => {
    onMessageReply?.(messageId);
  };

  const handleReact = (messageId: string, emoji: string) => {
    onMessageReact?.(messageId, emoji);
  };

  const handleRetract = (messageId: string) => {
    onMessageRetract?.(messageId);
  };

  // Handle toggle pin
  const handleTogglePin = async (conversationId: string) => {
    const dtoId = findConversationDtoId(conversationId, conversationDtos);
    if (dtoId === null) return;

    const conversation = conversationDtos.find((c) => c.id === dtoId);
    if (!conversation) return;

    try {
      await updateConversation(dtoId, { isPinned: !conversation.isPinned });
    } catch (error) {
      console.error("Failed to toggle pin:", error);
    }
  };

  // Handle delete - show confirmation dialog
  const handleDeleteClick = (conversationId: string) => {
    setConversationToDelete(conversationId);
    setShowDeleteDialog(true);
  };

  // Confirm deletion
  const handleConfirmDelete = async () => {
    if (!conversationToDelete) return;

    const dtoId = findConversationDtoId(conversationToDelete, conversationDtos);
    if (dtoId === null) return;

    try {
      await deleteConversation(dtoId, false); // Don't delete messages, just remove from conversation list
      setShowDeleteDialog(false);
      setConversationToDelete(null);

      // If the deleted conversation was active, clear the selection
      if (activeConversationId === conversationToDelete) {
        onConversationSelect("");
      }
    } catch (error) {
      console.error("Failed to delete conversation:", error);
    }
  };

  // Cancel deletion
  const handleCancelDelete = () => {
    setShowDeleteDialog(false);
    setConversationToDelete(null);
  };

  return (
    <>
      <div className="flex h-full bg-white/50 dark:bg-slate-900/50 backdrop-blur-sm">
        <div className="w-[280px] border-r border-slate-200/80 dark:border-slate-700/60 flex flex-col flex-shrink-0 bg-slate-50/80 dark:bg-slate-900/80">
          <div className="p-4 border-b border-slate-200/70 dark:border-slate-700/60 bg-white/40 dark:bg-slate-900/40">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400 dark:text-slate-500" />
              <input
                type="text"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="搜索会话..."
                aria-label="搜索会话"
                className="w-full pl-10 pr-4 py-2.5 bg-white dark:bg-slate-800 border border-slate-200/80 dark:border-slate-700 rounded-xl text-sm focus:outline-none focus:ring-2 focus:ring-emerald-500/50 focus:border-emerald-500 dark:text-white placeholder-slate-400 dark:placeholder-slate-500 transition-all duration-200 shadow-sm dark:shadow-[0_1px_2px_rgba(0,0,0,0.2)]"
              />
            </div>
          </div>

          <div className="flex-1 overflow-y-auto">
            {filterConversations(pinnedConversations).length > 0 && (
              <div className="px-4 py-2.5 text-xs font-bold text-slate-500 dark:text-slate-400 bg-slate-100/70 dark:bg-slate-800/40 uppercase tracking-wide">
                置顶会话
              </div>
            )}
            {filterConversations(pinnedConversations).map((conv) => (
              <ConversationItem
                key={conv.id}
                conversation={conv}
                isActive={conv.id === activeConversationId}
                onClick={() => onConversationSelect(conv.id)}
                onTogglePin={handleTogglePin}
                onDelete={handleDeleteClick}
              />
            ))}

            {filterConversations(regularConversations).map((conv) => (
              <ConversationItem
                key={conv.id}
                conversation={conv}
                isActive={conv.id === activeConversationId}
                onClick={() => onConversationSelect(conv.id)}
                onTogglePin={handleTogglePin}
                onDelete={handleDeleteClick}
              />
            ))}

            {filterConversations(conversations).length === 0 && (
              <div className="px-4 py-12 text-center">
                <div className="w-12 h-12 mx-auto mb-3 rounded-full bg-slate-100 dark:bg-slate-800 flex items-center justify-center shadow-inner">
                  <Search className="w-5 h-5 text-slate-400 dark:text-slate-500" />
                </div>
                <p className="text-sm text-slate-400 dark:text-slate-500 font-medium">
                  未找到匹配的会话
                </p>
              </div>
            )}
          </div>
        </div>

        <div className="flex-1 flex flex-col min-w-0 bg-white/70 dark:bg-slate-900/70">
          {activeConversation ? (
            <>
              <div className="px-6 py-4 border-b border-slate-200/80 dark:border-slate-700/60 flex items-center justify-between flex-shrink-0 bg-white/90 dark:bg-slate-900/90 backdrop-blur-sm shadow-sm dark:shadow-[0_1px_2px_rgba(0,0,0,0.2)]">
                <div className="flex items-center gap-3.5">
                  <div>
                    <h2 className="font-bold text-slate-900 dark:text-slate-100 text-lg tracking-tight">
                      {displayName}
                    </h2>
                    {memberCount && (
                      <p className="text-xs text-slate-500 dark:text-slate-400 mt-0.5 font-medium">
                        {memberCount} 位成员
                      </p>
                    )}
                  </div>
                </div>

                <button
                  className="p-2.5 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-xl transition-colors"
                  title="更多"
                >
                  <MoreVertical className="w-5 h-5 text-slate-500 dark:text-slate-400" />
                </button>
              </div>

              <div className="flex-1 overflow-y-auto px-6 py-5">
                {activeConversation.messages &&
                activeConversation.messages.length > 0 ? (
                  <>
                    {activeConversation.messages.map((message, index) => {
                      const isSent = message.senderId === currentUser.id;
                      const showAvatar =
                        !isSent &&
                        (index === 0 ||
                          activeConversation.messages![index - 1].senderId !==
                            message.senderId);

                      const avatarUrl = isSent
                        ? currentUser.avatar
                        : activeConversation.type === "group"
                          ? activeConversation.group?.members.find(
                              (m) => m.id === message.senderId,
                            )?.avatar
                          : activeConversation.participant?.avatar;

                      return (
                        <MessageBubble
                          key={message.id}
                          message={message}
                          isSent={isSent}
                          showAvatar={showAvatar}
                          avatarUrl={avatarUrl}
                          onReply={handleReply}
                          onReact={handleReact}
                          onRetract={handleRetract}
                        />
                      );
                    })}
                    <div ref={messagesEndRef} />
                  </>
                ) : (
                  <div className="h-full flex items-center justify-center">
                    <div className="text-center">
                      <div className="w-16 h-16 mx-auto mb-4 rounded-2xl bg-emerald-50 dark:bg-emerald-900/20 flex items-center justify-center ring-2 ring-emerald-500/10">
                        <MessageSquare className="w-8 h-8 text-emerald-500 dark:text-emerald-400" />
                      </div>
                      <p className="text-slate-400 dark:text-slate-500 text-sm font-medium">
                        暂无消息，开始聊天吧
                      </p>
                    </div>
                  </div>
                )}
              </div>

              <MessageInput
                onSendMessage={handleSendMessage}
                onImageUpload={handleSendImage}
                placeholder={`发给 ${displayName}...`}
              />
            </>
          ) : (
            <div className="flex-1 flex items-center justify-center">
              <div className="text-center">
                <div className="w-20 h-20 mx-auto mb-5 rounded-2xl bg-slate-100 dark:bg-slate-800 flex items-center justify-center ring-2 ring-slate-200/50 dark:ring-slate-700/50">
                  <MessageSquare className="w-10 h-10 text-slate-400 dark:text-slate-500" />
                </div>
                <p className="text-slate-400 dark:text-slate-500 text-base font-medium">
                  选择一个会话开始聊天
                </p>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Delete Confirmation Dialog */}
      {showDeleteDialog && (
        <div className="fixed inset-0 z-50 flex items-center justify-center">
          {/* Backdrop */}
          <div
            className="absolute inset-0 bg-black/50 backdrop-blur-sm"
            onClick={handleCancelDelete}
          />

          {/* Dialog */}
          <div className="relative bg-white dark:bg-slate-800 rounded-2xl shadow-2xl border border-slate-200 dark:border-slate-700 max-w-md w-full mx-4 overflow-hidden">
            <div className="p-6">
              <div className="flex items-start gap-4">
                <div className="flex-shrink-0 w-12 h-12 rounded-full bg-red-100 dark:bg-red-900/20 flex items-center justify-center">
                  <AlertTriangle className="w-6 h-6 text-red-600 dark:text-red-400" />
                </div>
                <div className="flex-1">
                  <h3 className="text-lg font-bold text-slate-900 dark:text-slate-100 mb-2">
                    删除会话
                  </h3>
                  <p className="text-sm text-slate-600 dark:text-slate-400">
                    确定要删除此会话吗？此操作不会删除聊天记录，只会从会话列表中移除。
                  </p>
                </div>
              </div>
            </div>

            <div className="px-6 pb-6 flex gap-3 justify-end">
              <button
                onClick={handleCancelDelete}
                className="px-4 py-2.5 rounded-xl text-sm font-bold text-slate-700 dark:text-slate-200 hover:bg-slate-100 dark:hover:bg-slate-700 transition-colors"
              >
                取消
              </button>
              <button
                onClick={handleConfirmDelete}
                className="px-4 py-2.5 rounded-xl text-sm font-bold text-white bg-red-600 hover:bg-red-700 dark:bg-red-500 dark:hover:bg-red-600 transition-colors shadow-lg shadow-red-500/20"
              >
                删除
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
}

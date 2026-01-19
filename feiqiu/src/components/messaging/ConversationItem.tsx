import { useState, useRef, useEffect } from "react";
import { Conversation } from "../../lib/types/messaging";
import { cn } from "../../lib/utils";
import { Pin, PinOff, Trash2, MoreVertical } from "lucide-react";

interface ConversationItemProps {
  conversation: Conversation;
  isActive: boolean;
  onClick: () => void;
  onTogglePin?: (conversationId: string) => void;
  onDelete?: (conversationId: string) => void;
}

function formatTime(timestamp: string): string {
  const date = new Date(timestamp);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / 60000);
  const diffHours = Math.floor(diffMs / 3600000);
  const diffDays = Math.floor(diffMs / 86400000);

  if (diffMins < 1) return "刚刚";
  if (diffMins < 60) return `${diffMins}分钟前`;
  if (diffHours < 24) return `${diffHours}小时前`;
  if (diffDays < 7) return `${diffDays}天前`;

  const month = date.getMonth() + 1;
  const day = date.getDate();
  return `${month}/${day}`;
}

export function ConversationItem({
  conversation,
  isActive,
  onClick,
  onTogglePin,
  onDelete,
}: ConversationItemProps) {
  const [showMenu, setShowMenu] = useState(false);
  const [menuPosition, setMenuPosition] = useState({ x: 0, y: 0 });
  const menuRef = useRef<HTMLDivElement>(null);
  const itemRef = useRef<HTMLDivElement>(null);

  const isGroup = conversation.type === "group";
  const displayName = isGroup
    ? conversation.group?.name
    : conversation.participant?.name;
  const displayAvatar = isGroup
    ? conversation.group?.avatar
    : conversation.participant?.avatar;
  const lastMessageTime = conversation.lastMessage
    ? formatTime(conversation.lastMessage.timestamp)
    : "";
  const userStatus = !isGroup ? conversation.participant?.status : undefined;

  const containerClass = cn(
    "flex items-center gap-3 px-4 py-3.5 cursor-pointer transition-all duration-200 border-l-2 border-transparent relative",
    isActive
      ? "bg-emerald-500 dark:bg-emerald-600 border-l-emerald-600 dark:border-l-emerald-500 shadow-lg shadow-emerald-500/10 dark:shadow-emerald-500/20"
      : "hover:bg-slate-100/80 dark:hover:bg-slate-800/80 border-l-transparent",
  );

  const nameClass = cn(
    "font-bold text-sm truncate pr-2",
    isActive
      ? "text-white dark:text-white"
      : "text-slate-900 dark:text-slate-100",
  );

  const timeClass = cn(
    "text-xs flex-shrink-0 font-medium",
    isActive
      ? "text-emerald-100 dark:text-emerald-200"
      : "text-slate-500 dark:text-slate-400",
  );

  const messageClass = cn(
    "text-xs truncate font-medium",
    isActive
      ? "text-emerald-100 dark:text-emerald-200 opacity-90"
      : "text-slate-500 dark:text-slate-400",
  );

  const badgeClass = cn(
    "flex-shrink-0 px-2 py-0.5 rounded-full text-xs font-bold",
    isActive
      ? "bg-white text-emerald-600 dark:bg-slate-200 dark:text-emerald-700 shadow-sm"
      : "bg-emerald-500 text-white dark:bg-emerald-600 shadow-md shadow-emerald-500/20",
  );

  const statusColor = cn({
    "bg-emerald-500 ring-2 ring-emerald-500/30": userStatus === "online",
    "bg-amber-500 ring-2 ring-amber-500/30": userStatus === "away",
    "bg-slate-400 ring-2 ring-slate-400/30": userStatus === "offline",
  });

  // Handle right-click to show context menu
  const handleContextMenu = (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setMenuPosition({ x: e.clientX, y: e.clientY });
    setShowMenu(true);
  };

  // Handle click on the more button
  const handleMoreClick = (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
    setMenuPosition({ x: rect.left, y: rect.bottom + 4 });
    setShowMenu(true);
  };

  // Close menu when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        menuRef.current &&
        !menuRef.current.contains(event.target as Node) &&
        itemRef.current &&
        !itemRef.current.contains(event.target as Node)
      ) {
        setShowMenu(false);
      }
    };

    if (showMenu) {
      document.addEventListener("mousedown", handleClickOutside);
    }

    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, [showMenu]);

  // Handle menu actions
  const handleTogglePin = () => {
    setShowMenu(false);
    onTogglePin?.(conversation.id);
  };

  const handleDelete = () => {
    setShowMenu(false);
    onDelete?.(conversation.id);
  };

  return (
    <div
      ref={itemRef}
      onContextMenu={handleContextMenu}
      className={containerClass}
    >
      <div className="relative flex-shrink-0">
        <img
          src={displayAvatar}
          alt={displayName}
          className="w-12 h-12 rounded-xl object-cover ring-2 ring-white dark:ring-slate-900/50 shadow-sm"
        />
        {userStatus && (
          <span
            className={cn(
              "absolute -bottom-0.5 -right-0.5 w-3.5 h-3.5 border-2 border-white dark:border-slate-900 rounded-full",
              statusColor,
            )}
          />
        )}
      </div>

      <div className="flex-1 min-w-0" onClick={onClick}>
        <div className="flex items-center justify-between mb-1">
          <h4 className={nameClass}>
            {conversation.pinned && <span className="mr-1 text-xs">📌</span>}
            {displayName}
          </h4>
          <span className={timeClass}>{lastMessageTime}</span>
        </div>

        <div className="flex items-center justify-between gap-2">
          {conversation.lastMessage ? (
            <p className={messageClass}>
              {conversation.lastMessage.type === "image" && (
                <span className="mr-1">📷</span>
              )}
              {conversation.lastMessage.senderId !== "user-1" && (
                <span className="font-bold mr-1">
                  {isGroup ? `${conversation.lastMessage.senderName}: ` : ""}
                </span>
              )}
              {conversation.lastMessage.content}
            </p>
          ) : (
            <p className={messageClass}>暂无消息</p>
          )}

          {conversation.unreadCount > 0 && (
            <span className={badgeClass}>
              {conversation.unreadCount > 99 ? "99+" : conversation.unreadCount}
            </span>
          )}
        </div>
      </div>

      {/* More button for mobile/alternative access */}
      <button
        onClick={handleMoreClick}
        className={cn(
          "p-1.5 rounded-lg transition-colors flex-shrink-0",
          isActive
            ? "hover:bg-emerald-400 dark:hover:bg-emerald-500 text-white"
            : "hover:bg-slate-200 dark:hover:bg-slate-700 text-slate-400 dark:text-slate-500",
        )}
      >
        <MoreVertical className="w-4 h-4" />
      </button>

      {/* Context Menu */}
      {showMenu && (
        <div
          ref={menuRef}
          className="fixed z-50 min-w-[160px] bg-white dark:bg-slate-800 rounded-lg shadow-xl border border-slate-200 dark:border-slate-700 py-1"
          style={{
            left: `${menuPosition.x}px`,
            top: `${menuPosition.y}px`,
          }}
        >
          <button
            onClick={handleTogglePin}
            className={cn(
              "w-full px-4 py-2.5 text-left text-sm flex items-center gap-3 transition-colors",
              "hover:bg-slate-100 dark:hover:bg-slate-700",
              "text-slate-700 dark:text-slate-200",
            )}
          >
            {conversation.pinned ? (
              <>
                <PinOff className="w-4 h-4" />
                <span>取消置顶</span>
              </>
            ) : (
              <>
                <Pin className="w-4 h-4" />
                <span>置顶会话</span>
              </>
            )}
          </button>
          <button
            onClick={handleDelete}
            className={cn(
              "w-full px-4 py-2.5 text-left text-sm flex items-center gap-3 transition-colors",
              "hover:bg-red-50 dark:hover:bg-red-900/20",
              "text-red-600 dark:text-red-400",
            )}
          >
            <Trash2 className="w-4 h-4" />
            <span>删除会话</span>
          </button>
        </div>
      )}
    </div>
  );
}

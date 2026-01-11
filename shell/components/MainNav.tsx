import { MessageSquare, Users, Building2 } from 'lucide-react'

export interface MainNavProps {
  items: Array<{
    id: string
    label: string
    icon: string
    href: string
    isActive?: boolean
  }>
  activeId?: string
  onSelect?: (id: string) => void
}

const iconMap: Record<string, React.ComponentType<{ className?: string }>> = {
  chat: MessageSquare,
  contacts: Users,
  organization: Building2,
}

export function MainNav({ items, activeId, onSelect }: MainNavProps) {
  return (
    <nav className="flex flex-col gap-2 w-full">
      {items.map((item) => {
        const Icon = iconMap[item.icon] || MessageSquare
        const isActive = item.id === activeId

        return (
          <button
            key={item.id}
            onClick={() => onSelect?.(item.id)}
            className={`
              relative w-12 h-12 mx-auto rounded-xl flex items-center justify-center
              transition-all duration-200
              ${isActive
                ? 'bg-emerald-500 text-white shadow-md'
                : 'text-slate-600 dark:text-slate-400 hover:bg-slate-200 dark:hover:bg-slate-700'
              }
            `}
            title={item.label}
          >
            <Icon className="w-5 h-5" />
            {isActive && (
              <div className="absolute left-0 top-1/2 -translate-y-1/2 w-1 h-6 bg-white rounded-r-full" />
            )}
          </button>
        )
      })}
    </nav>
  )
}

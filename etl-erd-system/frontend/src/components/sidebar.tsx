import { useState } from "react"
import { 
  Upload, 
  Database, 
  GitBranch, 
  Share2, 
  FileText, 
  Settings,
  Info
} from "lucide-react"
import { Button } from "./ui/button"
import { cn } from "../lib/utils"

interface SidebarProps {
  onMenuSelect: (menu: string) => void
  activeMenu: string
}

const menuItems = [
  { id: "upload", icon: Upload, label: "Upload Files", description: "Upload CSV/XLSX files" },
  { id: "database", icon: Database, label: "Database", description: "View database operations" },
  { id: "etl", icon: GitBranch, label: "ETL Pipeline", description: "Build ETL workflows" },
  { id: "normalization", icon: Share2, label: "Normalization", description: "Data normalization (1NF, 2NF, 3NF)" },
  { id: "erd", icon: FileText, label: "ERD Viewer", description: "View Entity Relationship Diagrams" },
  { id: "settings", icon: Settings, label: "Settings", description: "Application settings" },
  { id: "help", icon: Info, label: "Help", description: "Documentation and help" },
]

export function Sidebar({ onMenuSelect, activeMenu }: SidebarProps) {
  const [hoveredItem, setHoveredItem] = useState<string | null>(null)

  return (
    <div className="fixed left-0 top-0 h-full w-16 bg-card border-r border-border flex flex-col items-center py-4 z-40">
      <div className="mb-8">
        <div className="w-10 h-10 bg-primary rounded-lg flex items-center justify-center">
          <Database className="w-6 h-6 text-primary-foreground" />
        </div>
      </div>

      <div className="flex flex-col gap-2 flex-1">
        {menuItems.map((item) => (
          <div
            key={item.id}
            className="relative group"
            onMouseEnter={() => setHoveredItem(item.id)}
            onMouseLeave={() => setHoveredItem(null)}
          >
            <Button
              variant={activeMenu === item.id ? "default" : "ghost"}
              size="icon"
              onClick={() => onMenuSelect(item.id)}
              className={cn(
                "w-12 h-12 relative",
                activeMenu === item.id && "bg-primary text-primary-foreground"
              )}
            >
              <item.icon className="w-5 h-5" />
            </Button>

            {/* Tooltip */}
            {hoveredItem === item.id && (
              <div className="absolute left-16 top-1/2 -translate-y-1/2 ml-2 px-3 py-2 bg-popover text-popover-foreground rounded-md shadow-lg border border-border z-50 whitespace-nowrap">
                <div className="font-medium">{item.label}</div>
                <div className="text-xs text-muted-foreground">{item.description}</div>
                {/* Arrow */}
                <div className="absolute left-0 top-1/2 -translate-y-1/2 -translate-x-1 w-2 h-2 bg-popover border-l border-b border-border rotate-45"></div>
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  )
}
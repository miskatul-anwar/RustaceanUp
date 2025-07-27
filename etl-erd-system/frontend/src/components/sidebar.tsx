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
    <div className="fixed left-0 top-0 h-full w-16 bg-card/80 backdrop-blur-xl border-r border-border/50 flex flex-col items-center py-4 z-40 shadow-modern">
      <div className="mb-8">
        <div className="w-10 h-10 bg-gradient-primary rounded-xl flex items-center justify-center shadow-glow-sm relative">
          <Database className="w-6 h-6 text-primary-foreground" />
          <div className="absolute inset-0 bg-gradient-primary opacity-20 rounded-xl blur"></div>
        </div>
      </div>

      <div className="flex flex-col gap-3 flex-1">
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
                "w-12 h-12 relative transition-all duration-300 hover:scale-105",
                activeMenu === item.id 
                  ? "bg-gradient-primary text-primary-foreground shadow-glow-sm border-0" 
                  : "hover:bg-gradient-primary/10 hover:shadow-modern border border-border/30"
              )}
            >
              <item.icon className="w-5 h-5" />
              {activeMenu === item.id && (
                <div className="absolute inset-0 bg-gradient-primary opacity-20 rounded-md blur"></div>
              )}
            </Button>

            {/* Enhanced Tooltip */}
            {hoveredItem === item.id && (
              <div className="absolute left-16 top-1/2 -translate-y-1/2 ml-2 px-4 py-3 bg-card/95 backdrop-blur-xl text-card-foreground rounded-lg shadow-modern-lg border border-border/50 z-50 whitespace-nowrap">
                <div className="font-medium text-sm">{item.label}</div>
                <div className="text-xs text-muted-foreground mt-1">{item.description}</div>
                {/* Enhanced Arrow */}
                <div className="absolute left-0 top-1/2 -translate-y-1/2 -translate-x-1 w-2 h-2 bg-card border-l border-b border-border/50 rotate-45 backdrop-blur-xl"></div>
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  )
}
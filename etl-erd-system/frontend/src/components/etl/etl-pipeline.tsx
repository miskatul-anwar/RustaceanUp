import { useCallback } from 'react'
import ReactFlow, {
  type Node,
  type Edge,
  addEdge,
  type Connection,
  useNodesState,
  useEdgesState,
  Controls,
  MiniMap,
  Background,
  BackgroundVariant,
} from 'reactflow'

import 'reactflow/dist/style.css'

import { Button } from '../ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '../ui/card'
import { Database, Filter, BarChart3, FileText } from 'lucide-react'

// Custom node types with modern colors
const nodeTypes = {
  source: {
    icon: Database,
    color: 'from-accent-blue to-accent-purple',
    bgClass: 'bg-gradient-to-r from-accent-blue to-accent-purple',
    category: 'input'
  },
  transform: {
    icon: Filter,
    color: 'from-green-500 to-emerald-500',
    bgClass: 'bg-gradient-to-r from-green-500 to-emerald-500',
    category: 'process'
  },
  normalize1nf: {
    icon: BarChart3,
    color: 'from-yellow-500 to-orange-500',
    bgClass: 'bg-gradient-to-r from-yellow-500 to-orange-500',
    category: 'normalization'
  },
  normalize2nf: {
    icon: BarChart3,
    color: 'from-orange-500 to-red-500',
    bgClass: 'bg-gradient-to-r from-orange-500 to-red-500',
    category: 'normalization'
  },
  normalize3nf: {
    icon: BarChart3,
    color: 'from-red-500 to-pink-500',
    bgClass: 'bg-gradient-to-r from-red-500 to-pink-500',
    category: 'normalization'
  },
  output: {
    icon: FileText,
    color: 'from-accent-purple to-accent-pink',
    bgClass: 'bg-gradient-to-r from-accent-purple to-accent-pink',
    category: 'output'
  }
}

const initialNodes: Node[] = [
  {
    id: '1',
    type: 'input',
    position: { x: 100, y: 100 },
    data: { 
      label: 'Data Source',
      nodeType: 'source',
      description: 'CSV/XLSX Input'
    },
  },
]

const initialEdges: Edge[] = []

// Compatibility rules
const compatibilityRules: Record<string, string[]> = {
  source: ['transform', 'normalize1nf'],
  transform: ['normalize1nf', 'output'],
  normalize1nf: ['normalize2nf', 'output'],
  normalize2nf: ['normalize3nf', 'output'],
  normalize3nf: ['output'],
  output: []
}

interface ETLPipelineProps {
  onPipelineChange: (nodes: Node[], edges: Edge[]) => void
}

export function ETLPipeline({ onPipelineChange }: ETLPipelineProps) {
  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes)
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges)

  const onConnect = useCallback(
    (params: Connection) => {
      const sourceNode = nodes.find(n => n.id === params.source)
      const targetNode = nodes.find(n => n.id === params.target)
      
      if (sourceNode && targetNode) {
        const sourceType = sourceNode.data.nodeType
        const targetType = targetNode.data.nodeType
        
        // Check compatibility
        if (compatibilityRules[sourceType]?.includes(targetType)) {
          setEdges((eds) => addEdge(params, eds))
          onPipelineChange(nodes, [...edges, { ...params, id: `${params.source}-${params.target}` } as Edge])
        } else {
          // Show toast notification for incompatible connection
          console.warn(`Cannot connect ${sourceType} to ${targetType}`)
        }
      }
    },
    [nodes, edges, onPipelineChange]
  )

  const addNode = useCallback((type: string) => {
    const newNode: Node = {
      id: `${Date.now()}`,
      type: 'default',
      position: { x: Math.random() * 400 + 200, y: Math.random() * 400 + 200 },
      data: {
        label: getNodeLabel(type),
        nodeType: type,
        description: getNodeDescription(type)
      },
    }
    setNodes((nds) => [...nds, newNode])
  }, [setNodes])

  const getNodeLabel = (type: string) => {
    const labels: Record<string, string> = {
      source: 'Data Source',
      transform: 'Transform',
      normalize1nf: '1NF Normalize',
      normalize2nf: '2NF Normalize',
      normalize3nf: '3NF Normalize',
      output: 'Output'
    }
    return labels[type] || 'Unknown'
  }

  const getNodeDescription = (type: string) => {
    const descriptions: Record<string, string> = {
      source: 'CSV/XLSX Input',
      transform: 'Data Cleaning & Transform',
      normalize1nf: 'First Normal Form',
      normalize2nf: 'Second Normal Form',
      normalize3nf: 'Third Normal Form',
      output: 'Final Output'
    }
    return descriptions[type] || ''
  }

  return (
    <div className="h-full flex bg-gradient-to-br from-background via-background to-accent-purple/5">
      {/* Node Palette */}
      <div className="w-72 bg-card/80 backdrop-blur-xl border-r border-border/50 p-6 shadow-modern">
        <h3 className="font-serif font-semibold text-lg mb-6 bg-gradient-primary bg-clip-text text-transparent">
          Pipeline Components
        </h3>
        <div className="space-y-3">
          {Object.entries(nodeTypes).map(([type, config]) => (
            <Button
              key={type}
              variant="outline"
              className="w-full justify-start gap-3 p-4 h-auto bg-card/50 border-border/30 hover:bg-gradient-primary/10 hover:border-accent-purple/50 transition-all duration-300 group"
              onClick={() => addNode(type)}
            >
              <div className={`w-8 h-8 rounded-lg flex items-center justify-center ${config.bgClass} shadow-glow-sm group-hover:scale-110 transition-transform duration-300`}>
                <config.icon className="w-4 h-4 text-white" />
              </div>
              <div className="text-left">
                <div className="font-medium">{getNodeLabel(type)}</div>
                <div className="text-xs text-muted-foreground">{getNodeDescription(type)}</div>
              </div>
            </Button>
          ))}
        </div>
        
        <Card className="mt-8 shadow-modern border-border/50 bg-card/50 backdrop-blur-xl">
          <CardHeader className="pb-3">
            <CardTitle className="text-sm font-serif bg-gradient-primary bg-clip-text text-transparent">
              Compatibility Rules
            </CardTitle>
          </CardHeader>
          <CardContent className="text-xs space-y-2 text-muted-foreground">
            <div className="flex items-center gap-2">
              <div className="w-2 h-2 bg-accent-blue rounded-full"></div>
              <span>Source → Transform, 1NF</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-2 h-2 bg-green-500 rounded-full"></div>
              <span>Transform → 1NF, Output</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-2 h-2 bg-yellow-500 rounded-full"></div>
              <span>1NF → 2NF, Output</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-2 h-2 bg-orange-500 rounded-full"></div>
              <span>2NF → 3NF, Output</span>
            </div>
            <div className="flex items-center gap-2">
              <div className="w-2 h-2 bg-red-500 rounded-full"></div>
              <span>3NF → Output</span>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Flow Canvas */}
      <div className="flex-1 relative">
        {/* Background decoration */}
        <div className="absolute inset-0 overflow-hidden pointer-events-none">
          <div className="absolute top-1/4 right-1/4 w-64 h-64 bg-gradient-radial from-accent-purple/10 to-transparent opacity-50 rounded-full blur-3xl"></div>
          <div className="absolute bottom-1/4 left-1/4 w-64 h-64 bg-gradient-radial from-accent-blue/10 to-transparent opacity-50 rounded-full blur-3xl"></div>
        </div>
        
        <ReactFlow
          nodes={nodes}
          edges={edges}
          onNodesChange={onNodesChange}
          onEdgesChange={onEdgesChange}
          onConnect={onConnect}
          connectionLineStyle={{ 
            stroke: 'hsl(var(--accent-purple))', 
            strokeWidth: 3,
            strokeDasharray: '5,5'
          }}
          defaultEdgeOptions={{
            style: { 
              stroke: 'hsl(var(--accent-purple))', 
              strokeWidth: 2 
            },
            type: 'smoothstep'
          }}
          snapToGrid={true}
          snapGrid={[20, 20]}
          className="relative z-10"
        >
          <Controls className="bg-card/80 backdrop-blur-xl border border-border/50 shadow-modern rounded-lg" />
          <MiniMap 
            nodeColor={(node) => {
              const nodeType = node.data.nodeType
              return `hsl(var(--accent-${nodeType === 'source' ? 'blue' : nodeType === 'output' ? 'purple' : 'pink'}))`
            }}
            className="bg-card/80 backdrop-blur-xl border border-border/50 shadow-modern rounded-lg"
            pannable
            zoomable
          />
          <Background 
            variant={BackgroundVariant.Dots} 
            gap={20} 
            size={1.5} 
            color="hsl(var(--border) / 0.3)"
          />
        </ReactFlow>
      </div>
    </div>
  )
}
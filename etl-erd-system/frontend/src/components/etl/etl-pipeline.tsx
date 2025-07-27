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

// Custom node types
const nodeTypes = {
  source: {
    icon: Database,
    color: 'bg-blue-500',
    category: 'input'
  },
  transform: {
    icon: Filter,
    color: 'bg-green-500',
    category: 'process'
  },
  normalize1nf: {
    icon: BarChart3,
    color: 'bg-yellow-500',
    category: 'normalization'
  },
  normalize2nf: {
    icon: BarChart3,
    color: 'bg-orange-500',
    category: 'normalization'
  },
  normalize3nf: {
    icon: BarChart3,
    color: 'bg-red-500',
    category: 'normalization'
  },
  output: {
    icon: FileText,
    color: 'bg-purple-500',
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
    <div className="h-full flex">
      {/* Node Palette */}
      <div className="w-64 bg-card border-r border-border p-4">
        <h3 className="font-semibold mb-4">Pipeline Components</h3>
        <div className="space-y-2">
          {Object.entries(nodeTypes).map(([type, config]) => (
            <Button
              key={type}
              variant="outline"
              className="w-full justify-start gap-2"
              onClick={() => addNode(type)}
            >
              <config.icon className="w-4 h-4" />
              {getNodeLabel(type)}
            </Button>
          ))}
        </div>
        
        <Card className="mt-6">
          <CardHeader className="pb-3">
            <CardTitle className="text-sm">Compatibility Rules</CardTitle>
          </CardHeader>
          <CardContent className="text-xs space-y-1">
            <div>• Source → Transform, 1NF</div>
            <div>• Transform → 1NF, Output</div>
            <div>• 1NF → 2NF, Output</div>
            <div>• 2NF → 3NF, Output</div>
            <div>• 3NF → Output</div>
          </CardContent>
        </Card>
      </div>

      {/* Flow Canvas */}
      <div className="flex-1">
        <ReactFlow
          nodes={nodes}
          edges={edges}
          onNodesChange={onNodesChange}
          onEdgesChange={onEdgesChange}
          onConnect={onConnect}
          connectionLineStyle={{ stroke: '#374151', strokeWidth: 2 }}
          snapToGrid={true}
          snapGrid={[15, 15]}
        >
          <Controls />
          <MiniMap 
            nodeColor={(node) => {
              const nodeType = node.data.nodeType
              return nodeTypes[nodeType as keyof typeof nodeTypes]?.color.replace('bg-', '#') || '#6b7280'
            }}
            className="bg-background border border-border"
          />
          <Background variant={BackgroundVariant.Dots} gap={12} size={1} />
        </ReactFlow>
      </div>
    </div>
  )
}
import { useState } from "react"
import { Card, CardContent, CardHeader, CardTitle } from "../ui/card"
import { Button } from "../ui/button"
import { Download, RefreshCw, Zap } from "lucide-react"

interface ERDViewerProps {
  erdData?: string
  onGenerateERD: () => void
  isLoading?: boolean
}

export function ERDViewer({ erdData, onGenerateERD, isLoading = false }: ERDViewerProps) {
  const [zoomLevel, setZoomLevel] = useState(100)

  const handleZoomIn = () => {
    setZoomLevel(prev => Math.min(prev + 25, 200))
  }

  const handleZoomOut = () => {
    setZoomLevel(prev => Math.max(prev - 25, 25))
  }

  const handleDownload = () => {
    if (erdData) {
      const blob = new Blob([erdData], { type: 'image/svg+xml' })
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = 'erd-diagram.svg'
      document.body.appendChild(a)
      a.click()
      document.body.removeChild(a)
      URL.revokeObjectURL(url)
    }
  }

  return (
    <Card className="h-full flex flex-col">
      <CardHeader className="flex-shrink-0">
        <div className="flex items-center justify-between">
          <CardTitle className="flex items-center gap-2">
            <Zap className="w-5 h-5" />
            Entity Relationship Diagram
          </CardTitle>
          <div className="flex items-center gap-2">
            <Button
              variant="outline"
              size="sm"
              onClick={onGenerateERD}
              disabled={isLoading}
            >
              <RefreshCw className={`w-4 h-4 mr-2 ${isLoading ? 'animate-spin' : ''}`} />
              Generate ERD
            </Button>
            {erdData && (
              <Button
                variant="outline"
                size="sm"
                onClick={handleDownload}
              >
                <Download className="w-4 h-4 mr-2" />
                Download
              </Button>
            )}
          </div>
        </div>
      </CardHeader>
      
      <CardContent className="flex-1 flex flex-col">
        {/* Controls */}
        <div className="flex items-center justify-between mb-4 pb-4 border-b border-border">
          <div className="flex items-center gap-2">
            <Button variant="outline" size="sm" onClick={handleZoomOut}>
              -
            </Button>
            <span className="text-sm font-mono w-12 text-center">{zoomLevel}%</span>
            <Button variant="outline" size="sm" onClick={handleZoomIn}>
              +
            </Button>
          </div>
          <Button 
            variant="outline" 
            size="sm" 
            onClick={() => setZoomLevel(100)}
          >
            Reset Zoom
          </Button>
        </div>

        {/* ERD Display Area */}
        <div className="flex-1 border border-border rounded-lg bg-background overflow-auto">
          {isLoading ? (
            <div className="h-full flex items-center justify-center">
              <div className="text-center">
                <RefreshCw className="w-8 h-8 mx-auto mb-4 animate-spin text-muted-foreground" />
                <p className="text-muted-foreground">Generating ERD...</p>
              </div>
            </div>
          ) : erdData ? (
            <div 
              className="p-4 transition-transform origin-top-left"
              style={{ transform: `scale(${zoomLevel / 100})` }}
              dangerouslySetInnerHTML={{ __html: erdData }}
            />
          ) : (
            <div className="h-full flex items-center justify-center">
              <div className="text-center max-w-md">
                <Zap className="w-16 h-16 mx-auto mb-4 text-muted-foreground" />
                <h3 className="text-lg font-medium mb-2">No ERD Generated</h3>
                <p className="text-muted-foreground mb-4">
                  Upload data and process it through the ETL pipeline to generate an Entity Relationship Diagram.
                </p>
                <Button onClick={onGenerateERD}>
                  <Zap className="w-4 h-4 mr-2" />
                  Generate ERD
                </Button>
              </div>
            </div>
          )}
        </div>

        {/* ERD Info Panel */}
        {erdData && (
          <div className="mt-4 p-4 bg-muted rounded-lg">
            <h4 className="font-medium mb-2">Diagram Information</h4>
            <div className="text-sm text-muted-foreground space-y-1">
              <div>Format: SVG</div>
              <div>Generated: {new Date().toLocaleString()}</div>
              <div>Zoom: {zoomLevel}%</div>
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  )
}
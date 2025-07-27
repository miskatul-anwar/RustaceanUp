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
    <Card className="h-full flex flex-col shadow-modern-lg border-border/50 bg-card/80 backdrop-blur-xl">
      <CardHeader className="flex-shrink-0 border-b border-border/30 bg-gradient-primary/5">
        <div className="flex items-center justify-between">
          <CardTitle className="flex items-center gap-3 text-xl">
            <div className="w-8 h-8 bg-gradient-primary rounded-lg flex items-center justify-center shadow-glow-sm">
              <Zap className="w-4 h-4 text-primary-foreground" />
            </div>
            <span className="bg-gradient-primary bg-clip-text text-transparent">Entity Relationship Diagram</span>
          </CardTitle>
          <div className="flex items-center gap-3">
            <Button
              variant="outline"
              size="sm"
              onClick={onGenerateERD}
              disabled={isLoading}
              className="bg-card/50 backdrop-blur-sm border-border/50 hover:bg-gradient-primary/10 hover:border-accent-purple/50 transition-all duration-300 shadow-modern"
            >
              <RefreshCw className={`w-4 h-4 mr-2 ${isLoading ? 'animate-spin' : ''}`} />
              Generate ERD
            </Button>
            {erdData && (
              <Button
                variant="outline"
                size="sm"
                onClick={handleDownload}
                className="bg-card/50 backdrop-blur-sm border-border/50 hover:bg-gradient-blue-purple/10 hover:border-accent-blue/50 transition-all duration-300 shadow-modern"
              >
                <Download className="w-4 h-4 mr-2" />
                Download
              </Button>
            )}
          </div>
        </div>
      </CardHeader>
      
      <CardContent className="flex-1 flex flex-col p-6">
        {/* Controls */}
        <div className="flex items-center justify-between mb-6 pb-4 border-b border-border/30">
          <div className="flex items-center gap-3 p-2 bg-gradient-primary/5 rounded-lg border border-border/30">
            <Button 
              variant="outline" 
              size="sm" 
              onClick={handleZoomOut}
              className="w-8 h-8 p-0 bg-card/50 hover:bg-gradient-primary/10 transition-all duration-300"
            >
              -
            </Button>
            <span className="text-sm font-mono w-12 text-center font-medium">{zoomLevel}%</span>
            <Button 
              variant="outline" 
              size="sm" 
              onClick={handleZoomIn}
              className="w-8 h-8 p-0 bg-card/50 hover:bg-gradient-primary/10 transition-all duration-300"
            >
              +
            </Button>
          </div>
          <Button 
            variant="outline" 
            size="sm" 
            onClick={() => setZoomLevel(100)}
            className="bg-card/50 backdrop-blur-sm border-border/50 hover:bg-gradient-primary/10 transition-all duration-300"
          >
            Reset Zoom
          </Button>
        </div>

        {/* ERD Display Area */}
        <div className="flex-1 border border-border/50 rounded-xl bg-gradient-radial from-background via-background to-accent-purple/5 overflow-auto shadow-modern relative">
          {/* Decorative corner gradients */}
          <div className="absolute top-0 left-0 w-32 h-32 bg-gradient-radial from-accent-purple/10 to-transparent opacity-50"></div>
          <div className="absolute bottom-0 right-0 w-32 h-32 bg-gradient-radial from-accent-blue/10 to-transparent opacity-50"></div>
          
          {isLoading ? (
            <div className="h-full flex items-center justify-center relative z-10">
              <div className="text-center p-8 bg-card/50 backdrop-blur-xl rounded-xl border border-border/30 shadow-modern">
                <div className="w-16 h-16 mx-auto mb-4 bg-gradient-primary/10 rounded-full flex items-center justify-center">
                  <RefreshCw className="w-8 h-8 animate-spin text-accent-purple" />
                </div>
                <p className="text-muted-foreground font-medium">Generating ERD...</p>
              </div>
            </div>
          ) : erdData ? (
            <div 
              className="p-6 transition-transform origin-top-left relative z-10"
              style={{ transform: `scale(${zoomLevel / 100})` }}
              dangerouslySetInnerHTML={{ __html: erdData }}
            />
          ) : (
            <div className="h-full flex items-center justify-center relative z-10">
              <div className="text-center max-w-md p-8 bg-card/50 backdrop-blur-xl rounded-xl border border-border/30 shadow-modern">
                <div className="w-20 h-20 mx-auto mb-6 bg-gradient-primary/10 rounded-2xl flex items-center justify-center">
                  <Zap className="w-10 h-10 text-accent-purple" />
                </div>
                <h3 className="text-xl font-serif font-medium mb-3 text-foreground">No ERD Generated</h3>
                <p className="text-muted-foreground mb-6 leading-relaxed">
                  Upload data and process it through the ETL pipeline to generate an Entity Relationship Diagram.
                </p>
                <Button 
                  onClick={onGenerateERD}
                  className="bg-gradient-primary hover:bg-gradient-primary/90 shadow-glow transition-all duration-300"
                >
                  <Zap className="w-4 h-4 mr-2" />
                  Generate ERD
                </Button>
              </div>
            </div>
          )}
        </div>

        {/* ERD Info Panel */}
        {erdData && (
          <div className="mt-6 p-5 bg-gradient-primary/5 rounded-xl border border-border/30 shadow-modern">
            <h4 className="font-serif font-medium mb-3 text-foreground">Diagram Information</h4>
            <div className="text-sm text-muted-foreground space-y-2 grid grid-cols-3 gap-4">
              <div className="flex items-center gap-2">
                <div className="w-2 h-2 bg-accent-purple rounded-full"></div>
                <span>Format: SVG</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-2 h-2 bg-accent-blue rounded-full"></div>
                <span>Generated: {new Date().toLocaleDateString()}</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-2 h-2 bg-accent-pink rounded-full"></div>
                <span>Zoom: {zoomLevel}%</span>
              </div>
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  )
}
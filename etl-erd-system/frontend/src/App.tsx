import { useState } from 'react'
import { ThemeProvider } from './components/theme-provider'
import { ThemeToggle } from './components/theme-toggle'
import { Sidebar } from './components/sidebar'
import { FileUpload } from './components/etl/file-upload'
import { ETLPipeline } from './components/etl/etl-pipeline'
import { ERDViewer } from './components/erd/erd-viewer'
import { Card, CardContent, CardHeader, CardTitle } from './components/ui/card'
import { Database, Settings, Info } from 'lucide-react'
import type { Node, Edge } from 'reactflow'

import './App.css'

function App() {
  const [activeMenu, setActiveMenu] = useState('upload')
  const [uploadedFile, setUploadedFile] = useState<File | null>(null)
  const [erdData, setErdData] = useState<string>('')
  const [isLoadingERD, setIsLoadingERD] = useState(false)

  const handleFileUpload = (file: File) => {
    setUploadedFile(file)
    // Auto-switch to ETL pipeline after upload
    setActiveMenu('etl')
  }

  const handlePipelineChange = (nodes: Node[], edges: Edge[]) => {
    console.log('Pipeline changed:', { nodes, edges })
  }

  const handleGenerateERD = async () => {
    setIsLoadingERD(true)
    // Simulate ERD generation
    setTimeout(() => {
      setErdData(`<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 400 300">
        <rect x="50" y="50" width="120" height="80" fill="#e5e7eb" stroke="#374151" stroke-width="2" rx="5"/>
        <text x="110" y="75" text-anchor="middle" font-family="Inter" font-size="14" font-weight="bold">Users</text>
        <text x="60" y="95" font-family="Inter" font-size="12">id (PK)</text>
        <text x="60" y="110" font-family="Inter" font-size="12">name</text>
        <text x="60" y="125" font-family="Inter" font-size="12">email</text>
        
        <rect x="230" y="50" width="120" height="80" fill="#e5e7eb" stroke="#374151" stroke-width="2" rx="5"/>
        <text x="290" y="75" text-anchor="middle" font-family="Inter" font-size="14" font-weight="bold">Orders</text>
        <text x="240" y="95" font-family="Inter" font-size="12">id (PK)</text>
        <text x="240" y="110" font-family="Inter" font-size="12">user_id (FK)</text>
        <text x="240" y="125" font-family="Inter" font-size="12">total</text>
        
        <line x1="170" y1="90" x2="230" y2="90" stroke="#374151" stroke-width="2"/>
        <text x="200" y="85" text-anchor="middle" font-family="Inter" font-size="10">1:N</text>
      </svg>`)
      setIsLoadingERD(false)
    }, 2000)
  }

  const renderContent = () => {
    switch (activeMenu) {
      case 'upload':
        return (
          <div className="p-8">
            <FileUpload onFileUpload={handleFileUpload} />
          </div>
        )
      
      case 'database':
        return (
          <div className="p-8">
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Database className="w-5 h-5" />
                  Database Operations
                </CardTitle>
              </CardHeader>
              <CardContent>
                <p className="text-muted-foreground">
                  Database operations and SQLite management will be displayed here.
                </p>
                {uploadedFile && (
                  <div className="mt-4 p-4 bg-muted rounded-lg">
                    <h4 className="font-medium">Current File</h4>
                    <p className="text-sm text-muted-foreground">{uploadedFile.name}</p>
                  </div>
                )}
              </CardContent>
            </Card>
          </div>
        )
      
      case 'etl':
        return (
          <div className="h-full">
            <ETLPipeline onPipelineChange={handlePipelineChange} />
          </div>
        )
      
      case 'normalization':
        return (
          <div className="p-8">
            <Card>
              <CardHeader>
                <CardTitle>Data Normalization</CardTitle>
              </CardHeader>
              <CardContent>
                <p className="text-muted-foreground">
                  Data normalization (1NF, 2NF, 3NF) and functional dependency analysis will be displayed here.
                </p>
              </CardContent>
            </Card>
          </div>
        )
      
      case 'erd':
        return (
          <div className="p-8 h-full">
            <ERDViewer 
              erdData={erdData} 
              onGenerateERD={handleGenerateERD}
              isLoading={isLoadingERD}
            />
          </div>
        )
      
      case 'settings':
        return (
          <div className="p-8">
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Settings className="w-5 h-5" />
                  Settings
                </CardTitle>
              </CardHeader>
              <CardContent>
                <p className="text-muted-foreground">
                  Application settings and preferences will be displayed here.
                </p>
              </CardContent>
            </Card>
          </div>
        )
      
      case 'help':
        return (
          <div className="p-8">
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Info className="w-5 h-5" />
                  Help & Documentation
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-4">
                  <div>
                    <h4 className="font-medium">Getting Started</h4>
                    <p className="text-sm text-muted-foreground">
                      1. Upload a CSV or XLSX file<br/>
                      2. Build your ETL pipeline<br/>
                      3. Generate ERD diagrams<br/>
                      4. Export results
                    </p>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>
        )
      
      default:
        return <div>Select a menu item</div>
    }
  }

  return (
    <ThemeProvider defaultTheme="system" storageKey="etl-erd-theme">
      <div className="h-screen bg-background text-foreground">
        {/* Header */}
        <header className="fixed top-0 left-16 right-0 h-16 bg-card border-b border-border flex items-center justify-between px-6 z-30">
          <div>
            <h1 className="text-xl font-serif font-bold">ETL & ERD System</h1>
            <p className="text-sm text-muted-foreground">
              Extract, Transform, Load & Entity Relationship Diagrams
            </p>
          </div>
          <ThemeToggle />
        </header>

        {/* Sidebar */}
        <Sidebar onMenuSelect={setActiveMenu} activeMenu={activeMenu} />

        {/* Main Content */}
        <main className="ml-16 pt-16 h-full">
          {renderContent()}
        </main>
      </div>
    </ThemeProvider>
  )
}

export default App

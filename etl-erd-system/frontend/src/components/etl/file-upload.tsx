import { useState, useCallback } from "react"
import { Upload, FileText, X } from "lucide-react"
import { Button } from "../ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "../ui/card"
import { cn } from "../../lib/utils"

interface FileUploadProps {
  onFileUpload: (file: File) => void
}

export function FileUpload({ onFileUpload }: FileUploadProps) {
  const [dragActive, setDragActive] = useState(false)
  const [uploadedFile, setUploadedFile] = useState<File | null>(null)

  const handleDrag = useCallback((e: React.DragEvent) => {
    e.preventDefault()
    e.stopPropagation()
    if (e.type === "dragenter" || e.type === "dragover") {
      setDragActive(true)
    } else if (e.type === "dragleave") {
      setDragActive(false)
    }
  }, [])

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault()
    e.stopPropagation()
    setDragActive(false)

    if (e.dataTransfer.files && e.dataTransfer.files[0]) {
      const file = e.dataTransfer.files[0]
      if (file.type === "text/csv" || file.type === "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet") {
        setUploadedFile(file)
        onFileUpload(file)
      }
    }
  }, [onFileUpload])

  const handleChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    e.preventDefault()
    if (e.target.files && e.target.files[0]) {
      const file = e.target.files[0]
      setUploadedFile(file)
      onFileUpload(file)
    }
  }, [onFileUpload])

  const removeFile = () => {
    setUploadedFile(null)
  }

  return (
    <Card className="w-full max-w-2xl mx-auto shadow-modern-lg border-border/50 bg-card/80 backdrop-blur-xl">
      <CardHeader className="space-y-1 pb-4">
        <CardTitle className="flex items-center gap-3 text-xl">
          <div className="w-8 h-8 bg-gradient-primary rounded-lg flex items-center justify-center shadow-glow-sm">
            <Upload className="w-4 h-4 text-primary-foreground" />
          </div>
          <span className="bg-gradient-primary bg-clip-text text-transparent">Upload Data File</span>
        </CardTitle>
        <CardDescription className="text-base">
          Upload a CSV or XLSX file to start building your ETL pipeline
        </CardDescription>
      </CardHeader>
      <CardContent>
        {!uploadedFile ? (
          <div
            className={cn(
              "border-2 border-dashed rounded-xl p-12 text-center cursor-pointer transition-all duration-300 relative overflow-hidden group",
              dragActive 
                ? "border-accent-purple bg-gradient-primary/5 shadow-glow" 
                : "border-border/50 hover:border-accent-purple/50 hover:bg-gradient-primary/5"
            )}
            onDragEnter={handleDrag}
            onDragLeave={handleDrag}
            onDragOver={handleDrag}
            onDrop={handleDrop}
            onClick={() => document.getElementById("file-input")?.click()}
          >
            {/* Background gradient effect */}
            <div className="absolute inset-0 bg-gradient-radial from-accent-purple/10 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
            
            <div className="relative z-10">
              <div className="w-16 h-16 mx-auto mb-6 bg-gradient-primary/10 rounded-2xl flex items-center justify-center group-hover:scale-110 transition-transform duration-300">
                <Upload className="w-8 h-8 text-accent-purple" />
              </div>
              <h3 className="text-xl font-serif font-medium mb-3 text-foreground">Drop your file here</h3>
              <p className="text-muted-foreground mb-6 text-sm leading-relaxed">
                Supports CSV and XLSX files up to 10MB<br/>
                Drag and drop or click to browse
              </p>
              <Button variant="outline" className="bg-card/50 backdrop-blur-sm border-border/50 hover:bg-gradient-primary/10 hover:border-accent-purple/50 transition-all duration-300">
                <Upload className="w-4 h-4 mr-2" />
                Browse Files
              </Button>
            </div>
            <input
              id="file-input"
              type="file"
              accept=".csv,.xlsx"
              onChange={handleChange}
              className="hidden"
            />
          </div>
        ) : (
          <div className="border border-border/50 rounded-xl p-6 bg-gradient-primary/5 shadow-modern">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-4">
                <div className="w-12 h-12 bg-gradient-primary/20 rounded-xl flex items-center justify-center shadow-glow-sm">
                  <FileText className="w-6 h-6 text-accent-purple" />
                </div>
                <div>
                  <h4 className="font-medium text-foreground">{uploadedFile.name}</h4>
                  <p className="text-sm text-muted-foreground">
                    {(uploadedFile.size / 1024 / 1024).toFixed(2)} MB • Ready for processing
                  </p>
                </div>
              </div>
              <Button variant="ghost" size="icon" onClick={removeFile} className="hover:bg-destructive/10 hover:text-destructive transition-colors duration-300">
                <X className="w-4 h-4" />
              </Button>
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  )
}
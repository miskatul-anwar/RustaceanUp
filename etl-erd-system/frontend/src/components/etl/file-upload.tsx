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
    <Card className="w-full max-w-2xl mx-auto">
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Upload className="w-5 h-5" />
          Upload Data File
        </CardTitle>
        <CardDescription>
          Upload a CSV or XLSX file to start building your ETL pipeline
        </CardDescription>
      </CardHeader>
      <CardContent>
        {!uploadedFile ? (
          <div
            className={cn(
              "border-2 border-dashed border-border rounded-lg p-8 text-center cursor-pointer transition-colors",
              dragActive && "border-primary bg-primary/5"
            )}
            onDragEnter={handleDrag}
            onDragLeave={handleDrag}
            onDragOver={handleDrag}
            onDrop={handleDrop}
            onClick={() => document.getElementById("file-input")?.click()}
          >
            <Upload className="w-12 h-12 mx-auto mb-4 text-muted-foreground" />
            <h3 className="text-lg font-medium mb-2">Drop your file here</h3>
            <p className="text-muted-foreground mb-4">
              Supports CSV and XLSX files up to 10MB
            </p>
            <Button variant="outline">
              Browse Files
            </Button>
            <input
              id="file-input"
              type="file"
              accept=".csv,.xlsx"
              onChange={handleChange}
              className="hidden"
            />
          </div>
        ) : (
          <div className="border border-border rounded-lg p-4">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-3">
                <div className="w-10 h-10 bg-primary/10 rounded-lg flex items-center justify-center">
                  <FileText className="w-5 h-5 text-primary" />
                </div>
                <div>
                  <h4 className="font-medium">{uploadedFile.name}</h4>
                  <p className="text-sm text-muted-foreground">
                    {(uploadedFile.size / 1024 / 1024).toFixed(2)} MB
                  </p>
                </div>
              </div>
              <Button variant="ghost" size="icon" onClick={removeFile}>
                <X className="w-4 h-4" />
              </Button>
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  )
}
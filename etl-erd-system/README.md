# ETL & ERD Generation System

A comprehensive ETL (Extract, Transform, Load) and ERD (Entity Relationship Diagram) generation software with modern web interface and powerful data processing capabilities.

## Features

- **Frontend (React + Vite + TypeScript)**
  - Modern UI with dark/light mode toggle
  - Sidebar with icon-only buttons (show text on hover)
  - React Flow interface for ETL pipeline building
  - File upload for CSV/XLSX files
  - Interactive, colorful nodes with icons
  - Node compatibility checking with toast notifications
  - Minimap component
  - ERD display area for generated diagrams
  - Built with shadcn/ui and Aceternity UI components

- **Backend (Python FastAPI)**
  - Accept CSV/XLSX uploads and process them
  - SQLite3 database operations (insert to 'main' table)
  - Data cleaning and normalization (1NF, 2NF, 3NF)
  - Functional dependency detection using TANE algorithm
  - ERD generation using eralchemy (SVG output)
  - RESTful API endpoints for frontend communication

## Key Features

- **File Processing**: Support CSV and XLSX input files
- **Database Operations**: Insert data into SQLite3 'main' table
- **Data Normalization**: 1NF, 2NF, 3NF with functional dependency analysis
- **ETL Pipeline**: Visual pipeline builder with node-based interface
- **ERD Generation**: Automatic ERD creation and display
- **Node Compatibility**: Prevent illogical connections (e.g., 1NF after 2NF)
- **Modern UI**: Sleek, responsive design with proper accessibility

## Project Structure

```
etl-erd-system/
├── frontend/           # React + Vite + TypeScript frontend
├── backend/           # Python FastAPI backend
├── docs/             # Documentation
└── README.md         # This file
```

## Getting Started

### Frontend
```bash
cd frontend
npm install
npm run dev
```

### Backend
```bash
cd backend
pip install -r requirements.txt
uvicorn main:app --reload
```

## Technology Stack

- **Frontend**: React 18+, Vite, TypeScript, React Flow, shadcn/ui, Aceternity UI
- **Backend**: FastAPI, SQLite3, eralchemy, TANE algorithm
- **UI Components**: shadcn/ui, Aceternity UI
- **Fonts**: DM Serif for headings, system fonts for body text
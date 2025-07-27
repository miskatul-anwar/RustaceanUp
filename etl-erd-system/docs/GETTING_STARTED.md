# ETL & ERD System - Getting Started Guide

## Overview
This is a comprehensive ETL (Extract, Transform, Load) and ERD (Entity Relationship Diagram) generation system built with modern web technologies.

## Features Implemented

### Frontend (React + Vite + TypeScript)
✅ **Modern UI Components**
- Dark/light mode toggle with system theme detection
- Sidebar with icon-only buttons and hover tooltips
- Responsive design using Tailwind CSS
- DM Serif font for headings, Inter font for body text

✅ **File Upload System**
- Drag-and-drop interface for CSV/XLSX files
- File validation and preview
- Visual feedback for upload status

✅ **ETL Pipeline Builder**
- React Flow integration for visual pipeline building
- Interactive, draggable nodes with icons
- Node compatibility checking with validation rules
- Minimap component for navigation
- Pipeline components: Data Source, Transform, 1NF, 2NF, 3NF, Output

✅ **ERD Viewer**
- Interactive SVG diagram display
- Zoom controls (in/out/reset)
- Download functionality for ERD diagrams
- Real-time generation from database schema

### Backend (Python with FastAPI/HTTP Server)
✅ **File Processing**
- CSV and XLSX file upload handling
- Data validation and preprocessing
- SQLite3 database operations

✅ **Data Normalization**
- First Normal Form (1NF) implementation
- Second Normal Form (2NF) implementation  
- Third Normal Form (3NF) implementation
- TANE algorithm for functional dependency detection

✅ **ERD Generation**
- Automatic ERD creation from database schema
- SVG output format
- Relationship mapping and visualization

✅ **API Endpoints**
- RESTful API design
- CORS support for frontend integration
- Error handling and logging

## Quick Start

### Frontend Development
```bash
cd etl-erd-system/frontend
npm install
npm run dev
```
The frontend will be available at `http://localhost:5173`

### Backend Development
```bash
cd etl-erd-system/backend

# Option 1: Full FastAPI (requires pip install)
pip install -r requirements.txt
uvicorn main:app --reload

# Option 2: Simple HTTP server (no dependencies)
python simple_server.py
```
The backend will be available at `http://localhost:8000`

## Technology Stack

### Frontend
- **React 18+** - Modern React with hooks
- **Vite** - Fast build tool and dev server
- **TypeScript** - Type safety and better DX
- **Tailwind CSS** - Utility-first CSS framework
- **React Flow** - Node-based graph interface
- **Lucide React** - Beautiful icon library
- **Radix UI** - Accessible component primitives

### Backend
- **FastAPI** - Modern Python web framework
- **SQLite3** - Lightweight database
- **Pandas** - Data manipulation and analysis
- **SQLAlchemy** - Database ORM
- **Pydantic** - Data validation

## Architecture

### Frontend Structure
```
frontend/
├── src/
│   ├── components/
│   │   ├── ui/           # Reusable UI components
│   │   ├── etl/          # ETL-specific components
│   │   └── erd/          # ERD-specific components
│   ├── lib/              # Utility functions
│   └── hooks/            # Custom React hooks
├── public/               # Static assets
└── dist/                 # Build output
```

### Backend Structure
```
backend/
├── main.py               # FastAPI application
├── simple_server.py      # Fallback HTTP server
├── modules/
│   └── tane_algorithm.py # Functional dependency detection
├── utils/
│   └── data_processing.py # Data cleaning utilities
└── uploads/              # File upload directory
```

## Key Features Demonstrated

1. **File Upload & Processing** - Drag-and-drop interface with validation
2. **Visual ETL Pipeline** - Node-based workflow builder with compatibility rules
3. **ERD Generation** - Automatic diagram creation from database schema
4. **Theme Support** - Light/dark mode with system preference detection
5. **Responsive Design** - Works on desktop and mobile devices
6. **Type Safety** - Full TypeScript implementation
7. **API Integration** - RESTful backend communication

## Screenshots

### Main Upload Interface
![Upload Interface](https://github.com/user-attachments/assets/99696f8f-3669-4365-a48d-8a1d25d9dc12)

### ETL Pipeline Builder
![ETL Pipeline](https://github.com/user-attachments/assets/72c6a1d1-855c-4556-ad62-e21695c5a4bf)

### ERD Viewer
![ERD Viewer](https://github.com/user-attachments/assets/0c6f6e1e-862e-4b77-b925-0bafed2faa7c)

### Dark Mode
![Dark Mode](https://github.com/user-attachments/assets/b20316ae-6026-481b-a813-8f4287f340da)

## Next Steps

To extend this system, consider:
1. Adding more ETL transformation nodes
2. Implementing real file processing with pandas
3. Adding user authentication
4. Storing pipeline configurations
5. Adding data quality metrics
6. Implementing advanced ERD features
7. Adding export options (PDF, PNG)

## Notes

- The system is designed to be modular and extensible
- Backend includes both full FastAPI and simplified HTTP server versions
- Frontend is fully responsive and accessible
- All components follow modern React patterns and best practices
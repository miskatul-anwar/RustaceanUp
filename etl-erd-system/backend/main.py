from fastapi import FastAPI, UploadFile, File, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse
import pandas as pd
import sqlite3
import os
import aiofiles
from pathlib import Path
import tempfile
from typing import List, Dict, Any
import json
from sqlalchemy import create_engine, inspect
import logging

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

app = FastAPI(
    title="ETL & ERD Generation API",
    description="A comprehensive API for ETL operations and ERD generation",
    version="1.0.0"
)

# Enable CORS
app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:5173", "http://localhost:3000"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Database configuration
DATABASE_URL = "sqlite:///./etl_data.db"
engine = create_engine(DATABASE_URL)

# Ensure uploads directory exists
UPLOAD_DIR = Path("uploads")
UPLOAD_DIR.mkdir(exist_ok=True)

@app.get("/")
async def root():
    """Root endpoint"""
    return {"message": "ETL & ERD Generation API is running"}

@app.post("/upload-file/")
async def upload_file(file: UploadFile = File(...)):
    """Upload and process CSV/XLSX files"""
    try:
        # Validate file type
        if not file.filename.endswith(('.csv', '.xlsx')):
            raise HTTPException(status_code=400, detail="Only CSV and XLSX files are supported")
        
        # Save uploaded file
        file_path = UPLOAD_DIR / file.filename
        async with aiofiles.open(file_path, 'wb') as f:
            content = await file.read()
            await f.write(content)
        
        # Read file into pandas DataFrame
        if file.filename.endswith('.csv'):
            df = pd.read_csv(file_path)
        else:
            df = pd.read_excel(file_path)
        
        # Basic data analysis
        file_info = {
            "filename": file.filename,
            "rows": len(df),
            "columns": len(df.columns),
            "column_names": df.columns.tolist(),
            "dtypes": df.dtypes.astype(str).to_dict(),
            "sample_data": df.head(5).to_dict(orient="records")
        }
        
        logger.info(f"File uploaded successfully: {file.filename}")
        return JSONResponse(content=file_info)
        
    except Exception as e:
        logger.error(f"Error uploading file: {str(e)}")
        raise HTTPException(status_code=500, detail=f"Error processing file: {str(e)}")

@app.post("/insert-to-database/")
async def insert_to_database(filename: str):
    """Insert data from uploaded file to SQLite database"""
    try:
        file_path = UPLOAD_DIR / filename
        
        if not file_path.exists():
            raise HTTPException(status_code=404, detail="File not found")
        
        # Read file
        if filename.endswith('.csv'):
            df = pd.read_csv(file_path)
        else:
            df = pd.read_excel(file_path)
        
        # Insert to database
        df.to_sql('main', engine, if_exists='replace', index=False)
        
        logger.info(f"Data inserted to database from {filename}")
        return {"message": f"Data from {filename} inserted to 'main' table successfully", "rows": len(df)}
        
    except Exception as e:
        logger.error(f"Error inserting to database: {str(e)}")
        raise HTTPException(status_code=500, detail=f"Error inserting to database: {str(e)}")

@app.get("/database-info/")
async def get_database_info():
    """Get information about the database tables"""
    try:
        inspector = inspect(engine)
        tables = inspector.get_table_names()
        
        db_info = {}
        for table in tables:
            columns = inspector.get_columns(table)
            db_info[table] = {
                "columns": [{"name": col["name"], "type": str(col["type"])} for col in columns]
            }
        
        return {"tables": db_info}
        
    except Exception as e:
        logger.error(f"Error getting database info: {str(e)}")
        raise HTTPException(status_code=500, detail=f"Error getting database info: {str(e)}")

@app.post("/normalize/")
async def normalize_data(table_name: str = "main", normal_form: str = "1nf"):
    """Normalize data to specified normal form"""
    try:
        # This is a simplified implementation
        # In a real system, you'd implement proper normalization algorithms
        
        if normal_form not in ["1nf", "2nf", "3nf"]:
            raise HTTPException(status_code=400, detail="Invalid normal form. Use 1nf, 2nf, or 3nf")
        
        # Read data from database
        df = pd.read_sql(f"SELECT * FROM {table_name}", engine)
        
        # Simple normalization logic (placeholder)
        if normal_form == "1nf":
            # Remove duplicate rows, ensure atomic values
            df_normalized = df.drop_duplicates()
            result_table = f"{table_name}_1nf"
        elif normal_form == "2nf":
            # Remove partial dependencies (simplified)
            df_normalized = df.drop_duplicates()
            result_table = f"{table_name}_2nf"
        else:  # 3nf
            # Remove transitive dependencies (simplified)
            df_normalized = df.drop_duplicates()
            result_table = f"{table_name}_3nf"
        
        # Save normalized data
        df_normalized.to_sql(result_table, engine, if_exists='replace', index=False)
        
        return {
            "message": f"Data normalized to {normal_form.upper()}",
            "original_rows": len(df),
            "normalized_rows": len(df_normalized),
            "table_name": result_table
        }
        
    except Exception as e:
        logger.error(f"Error normalizing data: {str(e)}")
        raise HTTPException(status_code=500, detail=f"Error normalizing data: {str(e)}")

@app.post("/detect-functional-dependencies/")
async def detect_functional_dependencies(table_name: str = "main"):
    """Detect functional dependencies using simplified TANE algorithm"""
    try:
        # Read data from database
        df = pd.read_sql(f"SELECT * FROM {table_name}", engine)
        
        # Simplified functional dependency detection
        # In a real implementation, you'd use the full TANE algorithm
        fds = []
        columns = df.columns.tolist()
        
        for col in columns:
            # Check if this column uniquely determines others
            for other_col in columns:
                if col != other_col:
                    # Simple check: if values are unique for each value of col
                    grouped = df.groupby(col)[other_col].nunique()
                    if (grouped == 1).all():
                        fds.append({
                            "determinant": [col],
                            "dependent": other_col,
                            "confidence": 1.0
                        })
        
        return {"functional_dependencies": fds}
        
    except Exception as e:
        logger.error(f"Error detecting functional dependencies: {str(e)}")
        raise HTTPException(status_code=500, detail=f"Error detecting functional dependencies: {str(e)}")

@app.post("/generate-erd/")
async def generate_erd():
    """Generate ERD from database schema"""
    try:
        inspector = inspect(engine)
        tables = inspector.get_table_names()
        
        if not tables:
            return {"erd_svg": generate_empty_erd_svg()}
        
        # Generate a simple SVG ERD
        svg_content = generate_simple_erd_svg(inspector, tables)
        
        return {"erd_svg": svg_content}
        
    except Exception as e:
        logger.error(f"Error generating ERD: {str(e)}")
        # Return a simple placeholder SVG
        placeholder_svg = generate_placeholder_erd_svg()
        return {"erd_svg": placeholder_svg}

def generate_simple_erd_svg(inspector, tables: List[str]) -> str:
    """Generate a simple SVG ERD representation"""
    svg_parts = [
        '<?xml version="1.0" encoding="UTF-8"?>',
        '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 800 600">'
    ]
    
    x_offset = 50
    y_offset = 50
    table_width = 200
    table_height_base = 100
    
    for i, table in enumerate(tables):
        columns = inspector.get_columns(table)
        table_height = table_height_base + (len(columns) * 20)
        
        # Calculate position
        x = x_offset + (i % 3) * (table_width + 50)
        y = y_offset + (i // 3) * (table_height + 50)
        
        # Draw table rectangle
        svg_parts.append(f'<rect x="{x}" y="{y}" width="{table_width}" height="{table_height}" '
                        f'fill="#f8f9fa" stroke="#374151" stroke-width="2" rx="5"/>')
        
        # Table name
        svg_parts.append(f'<text x="{x + table_width//2}" y="{y + 25}" text-anchor="middle" '
                        f'font-family="Arial" font-size="16" font-weight="bold">{table}</text>')
        
        # Table columns
        for j, col in enumerate(columns):
            col_y = y + 50 + (j * 18)
            col_name = col['name']
            col_type = str(col['type'])
            
            # Primary key indicator
            pk_indicator = " (PK)" if col.get('primary_key') else ""
            
            svg_parts.append(f'<text x="{x + 10}" y="{col_y}" font-family="Arial" '
                           f'font-size="12">{col_name}: {col_type}{pk_indicator}</text>')
    
    svg_parts.append('</svg>')
    return '\n'.join(svg_parts)

def generate_placeholder_erd_svg() -> str:
    """Generate a placeholder ERD SVG"""
    return '''<?xml version="1.0" encoding="UTF-8"?>
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 400 300">
        <rect x="50" y="50" width="120" height="80" fill="#e5e7eb" stroke="#374151" stroke-width="2" rx="5"/>
        <text x="110" y="75" text-anchor="middle" font-family="Arial" font-size="14" font-weight="bold">main</text>
        <text x="60" y="95" font-family="Arial" font-size="12">Table generated from</text>
        <text x="60" y="110" font-family="Arial" font-size="12">uploaded data</text>
    </svg>'''

def generate_empty_erd_svg() -> str:
    """Generate an empty ERD SVG when no tables exist"""
    return '''<?xml version="1.0" encoding="UTF-8"?>
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 400 300">
        <text x="200" y="150" text-anchor="middle" font-family="Arial" font-size="16" fill="#6b7280">
            No tables found in database
        </text>
        <text x="200" y="170" text-anchor="middle" font-family="Arial" font-size="14" fill="#9ca3af">
            Upload and process data to generate ERD
        </text>
    </svg>'''

@app.get("/tables/")
async def list_tables():
    """List all tables in the database"""
    try:
        inspector = inspect(engine)
        tables = inspector.get_table_names()
        return {"tables": tables}
        
    except Exception as e:
        logger.error(f"Error listing tables: {str(e)}")
        raise HTTPException(status_code=500, detail=f"Error listing tables: {str(e)}")

@app.get("/table/{table_name}/data")
async def get_table_data(table_name: str, limit: int = 100):
    """Get data from a specific table"""
    try:
        df = pd.read_sql(f"SELECT * FROM {table_name} LIMIT {limit}", engine)
        return {
            "data": df.to_dict(orient="records"),
            "columns": df.columns.tolist(),
            "total_rows": len(df)
        }
        
    except Exception as e:
        logger.error(f"Error getting table data: {str(e)}")
        raise HTTPException(status_code=500, detail=f"Error getting table data: {str(e)}")

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
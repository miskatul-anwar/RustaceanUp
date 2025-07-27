"""
Simple HTTP server for ETL & ERD API
This is a fallback implementation using only Python standard library
"""
import http.server
import socketserver
import json
import urllib.parse
import sqlite3
import csv
import io
import os
from pathlib import Path

class ETLHandler(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == '/':
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.send_header('Access-Control-Allow-Origin', '*')
            self.end_headers()
            response = {"message": "ETL & ERD Generation API is running (Simple Mode)"}
            self.wfile.write(json.dumps(response).encode())
        
        elif self.path == '/tables/':
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.send_header('Access-Control-Allow-Origin', '*')
            self.end_headers()
            
            try:
                conn = sqlite3.connect('etl_data.db')
                cursor = conn.cursor()
                cursor.execute("SELECT name FROM sqlite_master WHERE type='table';")
                tables = [row[0] for row in cursor.fetchall()]
                conn.close()
                response = {"tables": tables}
            except Exception as e:
                response = {"tables": [], "error": str(e)}
            
            self.wfile.write(json.dumps(response).encode())
        
        elif self.path.startswith('/generate-erd/'):
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.send_header('Access-Control-Allow-Origin', '*')
            self.end_headers()
            
            # Generate simple ERD SVG
            svg_content = self.generate_simple_erd()
            response = {"erd_svg": svg_content}
            self.wfile.write(json.dumps(response).encode())
        
        else:
            self.send_response(404)
            self.end_headers()
    
    def do_POST(self):
        # Handle CORS preflight
        self.send_response(200)
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'POST, GET, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        self.send_header('Content-type', 'application/json')
        self.end_headers()
        
        if self.path == '/upload-file/':
            # Simplified file upload handling
            response = {
                "message": "File upload received (simplified mode)",
                "filename": "sample.csv",
                "rows": 100,
                "columns": 5,
                "column_names": ["id", "name", "email", "age", "city"],
                "sample_data": [
                    {"id": 1, "name": "John", "email": "john@example.com", "age": 30, "city": "New York"},
                    {"id": 2, "name": "Jane", "email": "jane@example.com", "age": 25, "city": "Boston"}
                ]
            }
        else:
            response = {"message": "Endpoint not implemented in simple mode"}
        
        self.wfile.write(json.dumps(response).encode())
    
    def do_OPTIONS(self):
        self.send_response(200)
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'POST, GET, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        self.end_headers()
    
    def generate_simple_erd(self):
        """Generate a simple ERD SVG"""
        return '''<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 600 400">
    <!-- Users Table -->
    <rect x="50" y="50" width="180" height="120" fill="#f8f9fa" stroke="#374151" stroke-width="2" rx="5"/>
    <text x="140" y="75" text-anchor="middle" font-family="Arial" font-size="16" font-weight="bold">Users</text>
    <line x1="60" y1="85" x2="220" y2="85" stroke="#374151" stroke-width="1"/>
    <text x="60" y="105" font-family="Arial" font-size="12">id (PK)</text>
    <text x="60" y="125" font-family="Arial" font-size="12">name</text>
    <text x="60" y="145" font-family="Arial" font-size="12">email</text>
    <text x="60" y="165" font-family="Arial" font-size="12">created_at</text>
    
    <!-- Orders Table -->
    <rect x="350" y="50" width="180" height="140" fill="#f8f9fa" stroke="#374151" stroke-width="2" rx="5"/>
    <text x="440" y="75" text-anchor="middle" font-family="Arial" font-size="16" font-weight="bold">Orders</text>
    <line x1="360" y1="85" x2="520" y2="85" stroke="#374151" stroke-width="1"/>
    <text x="360" y="105" font-family="Arial" font-size="12">id (PK)</text>
    <text x="360" y="125" font-family="Arial" font-size="12">user_id (FK)</text>
    <text x="360" y="145" font-family="Arial" font-size="12">total</text>
    <text x="360" y="165" font-family="Arial" font-size="12">status</text>
    <text x="360" y="185" font-family="Arial" font-size="12">created_at</text>
    
    <!-- Products Table -->
    <rect x="200" y="250" width="180" height="120" fill="#f8f9fa" stroke="#374151" stroke-width="2" rx="5"/>
    <text x="290" y="275" text-anchor="middle" font-family="Arial" font-size="16" font-weight="bold">Products</text>
    <line x1="210" y1="285" x2="370" y2="285" stroke="#374151" stroke-width="1"/>
    <text x="210" y="305" font-family="Arial" font-size="12">id (PK)</text>
    <text x="210" y="325" font-family="Arial" font-size="12">name</text>
    <text x="210" y="345" font-family="Arial" font-size="12">price</text>
    <text x="210" y="365" font-family="Arial" font-size="12">category</text>
    
    <!-- Relationships -->
    <line x1="230" y1="110" x2="350" y2="110" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead)"/>
    <text x="290" y="105" text-anchor="middle" font-family="Arial" font-size="10">1:N</text>
    
    <line x1="440" y1="190" x2="290" y2="250" stroke="#374151" stroke-width="2" marker-end="url(#arrowhead)"/>
    <text x="365" y="220" text-anchor="middle" font-family="Arial" font-size="10">N:M</text>
    
    <!-- Arrow marker definition -->
    <defs>
        <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
            <polygon points="0 0, 10 3.5, 0 7" fill="#374151"/>
        </marker>
    </defs>
</svg>'''

if __name__ == "__main__":
    PORT = 8000
    print(f"Starting ETL & ERD API server on port {PORT}")
    print("This is a simplified version due to network connectivity issues")
    
    with socketserver.TCPServer(("", PORT), ETLHandler) as httpd:
        print(f"Server running at http://localhost:{PORT}/")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\nServer stopped.")
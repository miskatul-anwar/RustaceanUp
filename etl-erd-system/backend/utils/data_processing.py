"""
Data processing utilities for ETL operations
"""
import pandas as pd
import numpy as np
from typing import Dict, List, Any, Optional
import logging

logger = logging.getLogger(__name__)

def clean_dataframe(df: pd.DataFrame) -> pd.DataFrame:
    """Clean and preprocess DataFrame"""
    try:
        # Remove completely empty rows and columns
        df = df.dropna(how='all').dropna(axis=1, how='all')
        
        # Handle missing values
        for col in df.columns:
            if df[col].dtype == 'object':
                # Fill string columns with 'Unknown'
                df[col] = df[col].fillna('Unknown')
            else:
                # Fill numeric columns with median
                df[col] = df[col].fillna(df[col].median())
        
        # Remove special characters from column names
        df.columns = [col.strip().replace(' ', '_').replace('-', '_') for col in df.columns]
        
        # Convert data types
        for col in df.columns:
            # Try to convert to numeric if possible
            if df[col].dtype == 'object':
                try:
                    df[col] = pd.to_numeric(df[col], errors='ignore')
                except:
                    pass
        
        logger.info(f"DataFrame cleaned: {df.shape[0]} rows, {df.shape[1]} columns")
        return df
        
    except Exception as e:
        logger.error(f"Error cleaning DataFrame: {str(e)}")
        return df

def analyze_data_quality(df: pd.DataFrame) -> Dict[str, Any]:
    """Analyze data quality metrics"""
    try:
        quality_report = {
            "total_rows": len(df),
            "total_columns": len(df.columns),
            "missing_values": {},
            "duplicate_rows": df.duplicated().sum(),
            "data_types": {},
            "column_stats": {}
        }
        
        for col in df.columns:
            missing_count = df[col].isna().sum()
            missing_percentage = (missing_count / len(df)) * 100
            
            quality_report["missing_values"][col] = {
                "count": int(missing_count),
                "percentage": round(missing_percentage, 2)
            }
            
            quality_report["data_types"][col] = str(df[col].dtype)
            
            # Column statistics
            if df[col].dtype in ['int64', 'float64']:
                quality_report["column_stats"][col] = {
                    "min": float(df[col].min()),
                    "max": float(df[col].max()),
                    "mean": float(df[col].mean()),
                    "std": float(df[col].std())
                }
            else:
                quality_report["column_stats"][col] = {
                    "unique_values": int(df[col].nunique()),
                    "most_common": str(df[col].mode().iloc[0]) if len(df[col].mode()) > 0 else "N/A"
                }
        
        return quality_report
        
    except Exception as e:
        logger.error(f"Error analyzing data quality: {str(e)}")
        return {"error": str(e)}

def detect_primary_key_candidates(df: pd.DataFrame) -> List[Dict[str, Any]]:
    """Detect potential primary key candidates"""
    try:
        candidates = []
        
        # Single column candidates
        for col in df.columns:
            unique_count = df[col].nunique()
            if unique_count == len(df) and df[col].notna().all():
                candidates.append({
                    "columns": [col],
                    "uniqueness": 1.0,
                    "completeness": 1.0,
                    "type": "single"
                })
        
        # Two column combinations
        for i, col1 in enumerate(df.columns):
            for col2 in df.columns[i+1:]:
                combined = df[[col1, col2]].drop_duplicates()
                if len(combined) == len(df):
                    candidates.append({
                        "columns": [col1, col2],
                        "uniqueness": 1.0,
                        "completeness": 1.0,
                        "type": "composite"
                    })
        
        return candidates
        
    except Exception as e:
        logger.error(f"Error detecting primary key candidates: {str(e)}")
        return []

def suggest_data_types(df: pd.DataFrame) -> Dict[str, str]:
    """Suggest optimal data types for columns"""
    suggestions = {}
    
    for col in df.columns:
        current_type = str(df[col].dtype)
        
        # Check if numeric column can be integer
        if df[col].dtype == 'float64' and df[col].notna().all():
            if (df[col] % 1 == 0).all():
                suggestions[col] = "int64"
            else:
                suggestions[col] = "float64"
        
        # Check if string column is actually categorical
        elif df[col].dtype == 'object':
            unique_ratio = df[col].nunique() / len(df)
            if unique_ratio < 0.1:  # Less than 10% unique values
                suggestions[col] = "category"
            else:
                suggestions[col] = "string"
        
        # Check if string column could be datetime
        if df[col].dtype == 'object':
            try:
                pd.to_datetime(df[col].head(10))
                suggestions[col] = "datetime"
            except:
                pass
    
    return suggestions

def transform_data_types(df: pd.DataFrame, type_suggestions: Dict[str, str]) -> pd.DataFrame:
    """Transform DataFrame columns to suggested data types"""
    try:
        df_transformed = df.copy()
        
        for col, suggested_type in type_suggestions.items():
            if col in df_transformed.columns:
                try:
                    if suggested_type == "int64":
                        df_transformed[col] = df_transformed[col].astype('int64')
                    elif suggested_type == "float64":
                        df_transformed[col] = df_transformed[col].astype('float64')
                    elif suggested_type == "category":
                        df_transformed[col] = df_transformed[col].astype('category')
                    elif suggested_type == "datetime":
                        df_transformed[col] = pd.to_datetime(df_transformed[col])
                except Exception as e:
                    logger.warning(f"Could not convert {col} to {suggested_type}: {str(e)}")
        
        return df_transformed
        
    except Exception as e:
        logger.error(f"Error transforming data types: {str(e)}")
        return df
"""
TANE Algorithm Implementation for Functional Dependency Discovery
Simplified version for educational purposes
"""
import pandas as pd
from typing import List, Tuple, Set, Dict
from itertools import combinations

class TANEAlgorithm:
    def __init__(self, df: pd.DataFrame):
        self.df = df
        self.attributes = list(df.columns)
        self.n_rows = len(df)
    
    def discover_functional_dependencies(self) -> List[Dict]:
        """
        Discover functional dependencies using a simplified TANE algorithm
        Returns a list of functional dependencies
        """
        fds = []
        
        # Level 0: Check single attribute dependencies
        for attr in self.attributes:
            for target in self.attributes:
                if attr != target:
                    if self._is_functional_dependency([attr], target):
                        fds.append({
                            "determinant": [attr],
                            "dependent": target,
                            "confidence": self._calculate_confidence([attr], target)
                        })
        
        # Level 1: Check two-attribute combinations
        for combo in combinations(self.attributes, 2):
            for target in self.attributes:
                if target not in combo:
                    if self._is_functional_dependency(list(combo), target):
                        fds.append({
                            "determinant": list(combo),
                            "dependent": target,
                            "confidence": self._calculate_confidence(list(combo), target)
                        })
        
        return fds
    
    def _is_functional_dependency(self, determinant: List[str], dependent: str) -> bool:
        """Check if determinant -> dependent is a functional dependency"""
        try:
            # Group by determinant attributes
            grouped = self.df.groupby(determinant)[dependent].nunique()
            # If all groups have exactly one unique value, it's a FD
            return (grouped == 1).all()
        except:
            return False
    
    def _calculate_confidence(self, determinant: List[str], dependent: str) -> float:
        """Calculate confidence of the functional dependency"""
        try:
            grouped = self.df.groupby(determinant)[dependent].nunique()
            # Confidence = percentage of groups with single value
            single_value_groups = (grouped == 1).sum()
            total_groups = len(grouped)
            return single_value_groups / total_groups if total_groups > 0 else 0.0
        except:
            return 0.0

def normalize_to_1nf(df: pd.DataFrame) -> pd.DataFrame:
    """Normalize data to First Normal Form (1NF)"""
    # Remove duplicate rows
    df_1nf = df.drop_duplicates()
    
    # Ensure atomic values (basic check for string columns)
    for col in df_1nf.select_dtypes(include=['object']).columns:
        # Split comma-separated values into separate rows
        if df_1nf[col].str.contains(',', na=False).any():
            df_expanded = df_1nf.assign(**{col: df_1nf[col].str.split(',')}).explode(col)
            df_1nf = df_expanded.reset_index(drop=True)
    
    return df_1nf

def normalize_to_2nf(df: pd.DataFrame, primary_key: List[str]) -> Dict[str, pd.DataFrame]:
    """Normalize data to Second Normal Form (2NF)"""
    # First ensure 1NF
    df_1nf = normalize_to_1nf(df)
    
    # Simplified 2NF: Remove partial dependencies
    tables = {"main": df_1nf}
    
    if len(primary_key) > 1:
        # Find attributes dependent on part of the primary key
        tane = TANEAlgorithm(df_1nf)
        fds = tane.discover_functional_dependencies()
        
        for fd in fds:
            # If determinant is subset of primary key and confidence > 0.9
            if (set(fd["determinant"]).issubset(set(primary_key)) and 
                len(fd["determinant"]) < len(primary_key) and 
                fd["confidence"] > 0.9):
                
                # Create separate table for this dependency
                table_name = f"table_{fd['determinant'][0]}"
                cols = fd["determinant"] + [fd["dependent"]]
                tables[table_name] = df_1nf[cols].drop_duplicates()
    
    return tables

def normalize_to_3nf(df: pd.DataFrame, primary_key: List[str]) -> Dict[str, pd.DataFrame]:
    """Normalize data to Third Normal Form (3NF)"""
    # First ensure 2NF
    tables_2nf = normalize_to_2nf(df, primary_key)
    
    # Simplified 3NF: Remove transitive dependencies
    tane = TANEAlgorithm(df)
    fds = tane.discover_functional_dependencies()
    
    for fd in fds:
        # If determinant is not primary key and confidence > 0.9
        if (not set(fd["determinant"]).issubset(set(primary_key)) and 
            fd["confidence"] > 0.9):
            
            # Create separate table for transitive dependency
            table_name = f"table_{fd['determinant'][0]}_transitive"
            cols = fd["determinant"] + [fd["dependent"]]
            tables_2nf[table_name] = df[cols].drop_duplicates()
    
    return tables_2nf
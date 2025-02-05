#!/usr/bin/env python3

import json
import os
import subprocess
import sys
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Tuple

class MetricsCollector:
    def __init__(self, root_dir: str):
        self.root_dir = Path(root_dir)
        self.metrics: Dict[str, Dict] = {}
        
    def collect_rust_metrics(self, path: Path) -> Dict:
        """Collect Rust-specific metrics."""
        metrics = {}
        try:
            # Collect code coverage
            result = subprocess.run(
                ["cargo", "tarpaulin", "--out", "Json"],
                cwd=path,
                capture_output=True,
                text=True
            )
            coverage_data = json.loads(result.stdout)
            metrics["coverage"] = coverage_data.get("coverage", 0)
            
            # Collect clippy warnings
            result = subprocess.run(
                ["cargo", "clippy", "--message-format=json"],
                cwd=path,
                capture_output=True,
                text=True
            )
            clippy_warnings = len([line for line in result.stdout.split("\n") if "warning" in line])
            metrics["clippy_warnings"] = clippy_warnings
            
        except Exception as e:
            print(f"Error collecting Rust metrics: {e}", file=sys.stderr)
        return metrics
    
    def collect_python_metrics(self, path: Path) -> Dict:
        """Collect Python-specific metrics."""
        metrics = {}
        try:
            # Collect test coverage
            result = subprocess.run(
                ["pytest", "--cov=src", "--cov-report=json"],
                cwd=path,
                capture_output=True,
                text=True
            )
            with open(path / "coverage.json") as f:
                coverage_data = json.load(f)
                metrics["coverage"] = coverage_data.get("totals", {}).get("percent_covered", 0)
            
            # Collect type hint coverage
            result = subprocess.run(
                ["mypy", "--strict", "src"],
                cwd=path,
                capture_output=True,
                text=True
            )
            type_errors = len([line for line in result.stdout.split("\n") if "error:" in line])
            metrics["type_errors"] = type_errors
            
        except Exception as e:
            print(f"Error collecting Python metrics: {e}", file=sys.stderr)
        return metrics
    
    def collect_dart_metrics(self, path: Path) -> Dict:
        """Collect Dart/Flutter-specific metrics."""
        metrics = {}
        try:
            # Collect test coverage
            result = subprocess.run(
                ["flutter", "test", "--coverage"],
                cwd=path,
                capture_output=True,
                text=True
            )
            with open(path / "coverage/lcov.info") as f:
                coverage_lines = f.readlines()
                covered_lines = len([line for line in coverage_lines if line.startswith("DA:") and ",1" in line])
                total_lines = len([line for line in coverage_lines if line.startswith("DA:")])
                metrics["coverage"] = (covered_lines / total_lines * 100) if total_lines > 0 else 0
            
            # Collect analyzer warnings
            result = subprocess.run(
                ["flutter", "analyze"],
                cwd=path,
                capture_output=True,
                text=True
            )
            analyzer_warnings = len([line for line in result.stdout.split("\n") if "warning:" in line])
            metrics["analyzer_warnings"] = analyzer_warnings
            
        except Exception as e:
            print(f"Error collecting Dart metrics: {e}", file=sys.stderr)
        return metrics
    
    def collect_all_metrics(self):
        """Collect metrics for all components."""
        components = {
            "dash33": ("rust", "python"),
            "enterprise": ("rust",),
            "mobile": ("dart",),
            "web5-rs": ("rust",)
        }
        
        for component, languages in components.items():
            component_path = self.root_dir / "anya-core" / component
            if not component_path.exists():
                continue
                
            self.metrics[component] = {
                "timestamp": datetime.now().isoformat(),
                "metrics": {}
            }
            
            for lang in languages:
                if lang == "rust":
                    self.metrics[component]["metrics"]["rust"] = self.collect_rust_metrics(component_path)
                elif lang == "python":
                    self.metrics[component]["metrics"]["python"] = self.collect_python_metrics(component_path)
                elif lang == "dart":
                    self.metrics[component]["metrics"]["dart"] = self.collect_dart_metrics(component_path)
    
    def save_metrics(self, output_file: str):
        """Save collected metrics to a JSON file."""
        with open(output_file, 'w') as f:
            json.dump(self.metrics, f, indent=2)
    
    def print_summary(self):
        """Print a summary of collected metrics."""
        print("\nMetrics Summary:")
        print("=" * 50)
        for component, data in self.metrics.items():
            print(f"\n{component}:")
            print("-" * 30)
            for lang, metrics in data["metrics"].items():
                print(f"\n{lang.upper()}:")
                for metric, value in metrics.items():
                    print(f"  {metric}: {value}")

def main():
    root_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    collector = MetricsCollector(root_dir)
    collector.collect_all_metrics()
    
    # Save metrics to file
    metrics_dir = Path(root_dir) / "metrics"
    metrics_dir.mkdir(exist_ok=True)
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    metrics_file = metrics_dir / f"metrics_{timestamp}.json"
    collector.save_metrics(metrics_file)
    
    # Print summary
    collector.print_summary()

if __name__ == "__main__":
    main()

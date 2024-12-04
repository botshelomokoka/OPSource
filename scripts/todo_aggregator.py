#!/usr/bin/env python3
"""
Automatically aggregates TODOs from all repositories and their submodules.
Searches through code comments, issue trackers, and specific TODO files.
"""

import os
import re
import subprocess
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Set, Tuple

class TodoAggregator:
    def __init__(self, root_dir: str):
        self.root_dir = Path(root_dir)
        self.todo_pattern = re.compile(r'(?i)(?:TODO|FIXME|HACK|XXX|NOTE)[:|\s](.+?)(?:\n|$)')
        self.ignore_dirs = {'.git', 'node_modules', '__pycache__', 'venv', '.venv', 'dist', 'build'}
        self.code_extensions = {'.py', '.rs', '.js', '.ts', '.jsx', '.tsx', '.java', '.cpp', '.h', '.c', '.go'}

    def get_submodules(self) -> List[str]:
        """Get list of all submodule paths."""
        try:
            result = subprocess.run(
                ['git', 'submodule', 'status'],
                cwd=self.root_dir,
                capture_output=True,
                text=True,
                check=True
            )
            submodules = []
            for line in result.stdout.splitlines():
                # Extract submodule path from status line
                parts = line.strip().split()
                if len(parts) >= 2:
                    submodules.append(parts[1])
            return submodules
        except subprocess.CalledProcessError:
            print("Warning: Failed to get submodules")
            return []

    def scan_file(self, file_path: Path) -> List[Tuple[str, str, int]]:
        """Scan a single file for TODOs."""
        todos = []
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                for i, line in enumerate(f, 1):
                    matches = self.todo_pattern.findall(line)
                    for match in matches:
                        todos.append((str(file_path), match.strip(), i))
        except UnicodeDecodeError:
            print(f"Warning: Could not read {file_path} as text")
        except Exception as e:
            print(f"Error reading {file_path}: {e}")
        return todos

    def scan_directory(self, directory: Path) -> List[Tuple[str, str, int]]:
        """Recursively scan directory for TODOs."""
        todos = []
        try:
            for root, dirs, files in os.walk(directory):
                # Skip ignored directories
                dirs[:] = [d for d in dirs if d not in self.ignore_dirs]
                
                for file in files:
                    file_path = Path(root) / file
                    if file_path.suffix in self.code_extensions or file_path.name.lower() == 'todo.md':
                        todos.extend(self.scan_file(file_path))
        except Exception as e:
            print(f"Error scanning directory {directory}: {e}")
        return todos

    def format_markdown(self, todos: List[Tuple[str, str, int]]) -> str:
        """Format TODOs as markdown."""
        if not todos:
            return "No TODOs found."

        # Group TODOs by repository/submodule
        todos_by_repo: Dict[str, List[Tuple[str, str, int]]] = {}
        for file_path, todo, line in todos:
            repo = str(Path(file_path).relative_to(self.root_dir).parts[0])
            if repo not in todos_by_repo:
                todos_by_repo[repo] = []
            todos_by_repo[repo].append((file_path, todo, line))

        # Format markdown
        markdown = f"# Project TODOs\n\nLast updated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n\n"
        
        for repo, repo_todos in sorted(todos_by_repo.items()):
            markdown += f"## {repo}\n\n"
            for file_path, todo, line in sorted(repo_todos, key=lambda x: (x[0], x[2])):
                rel_path = Path(file_path).relative_to(self.root_dir)
                markdown += f"- [{rel_path}:{line}] {todo}\n"
            markdown += "\n"

        return markdown

    def aggregate_todos(self) -> str:
        """Aggregate TODOs from main repository and all submodules."""
        all_todos = []

        # Scan main repository
        print(f"Scanning main repository: {self.root_dir}")
        all_todos.extend(self.scan_directory(self.root_dir))

        # Scan submodules
        for submodule in self.get_submodules():
            submodule_path = self.root_dir / submodule
            if submodule_path.exists():
                print(f"Scanning submodule: {submodule}")
                all_todos.extend(self.scan_directory(submodule_path))

        return self.format_markdown(all_todos)

def main():
    """Main entry point."""
    # Get the repository root directory
    try:
        repo_root = subprocess.run(
            ['git', 'rev-parse', '--show-toplevel'],
            capture_output=True,
            text=True,
            check=True
        ).stdout.strip()
    except subprocess.CalledProcessError:
        print("Error: Not in a git repository")
        return

    # Create todo aggregator
    aggregator = TodoAggregator(repo_root)
    
    # Generate TODO markdown
    todo_markdown = aggregator.aggregate_todos()
    
    # Write to file
    todo_file = Path(repo_root) / '.github' / 'TODO.md'
    todo_file.parent.mkdir(exist_ok=True)
    todo_file.write_text(todo_markdown, encoding='utf-8')
    print(f"TODOs written to {todo_file}")

if __name__ == '__main__':
    main()

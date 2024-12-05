#!/usr/bin/env python3
"""
TODO Migration Script
Migrates and standardizes all TODO files across repositories to the new format.
"""

import os
import re
import yaml
import json
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional

class TodoMigration:
    """Handles migration of TODO files to standardized format."""
    
    def __init__(self, root_dir: str):
        self.root_dir = Path(root_dir)
        self.config = self._load_config()
        self.todo_files = []
        self.migrated_todos = []
    
    def _load_config(self) -> Dict:
        """Load configuration from git-config.yml."""
        config_path = self.root_dir / '.github' / 'git-config.yml'
        with open(config_path, 'r') as f:
            return yaml.safe_load(f)
    
    def find_todo_files(self):
        """Find all TODO files in the repository."""
        todo_patterns = ['TODO', 'todo', '@TODO', '@todo', 'TECHNICAL_TODO']
        
        for root, _, files in os.walk(self.root_dir):
            if any(excluded in root for excluded in ['.git', 'node_modules', 'venv']):
                continue
            
            for file in files:
                if any(pattern in file for pattern in todo_patterns):
                    file_path = Path(root) / file
                    if file_path.suffix in ['.md', '.txt', '']:
                        self.todo_files.append(file_path)
    
    def parse_todo_file(self, file_path: Path) -> List[Dict]:
        """Parse a TODO file and extract items."""
        todos = []
        current_section = None
        current_priority = 'normal'
        
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
            
            # Try to determine priority from path or content
            if 'critical' in content.lower() or 'p0' in content.lower():
                current_priority = 'critical'
            elif 'high' in content.lower() or 'p1' in content.lower():
                current_priority = 'high'
            elif 'low' in content.lower() or 'p4' in content.lower():
                current_priority = 'low'
            
            # Extract todos with existing format
            formatted_todos = re.findall(r'(?:TODO|FIXME|NOTE)\(([^)]+)\)\[([^]]+)\]:\s*(.+)', content)
            for assignee, priority, text in formatted_todos:
                todos.append({
                    'type': 'TODO',
                    'assignee': assignee,
                    'priority': priority,
                    'text': text.strip(),
                    'file': str(file_path.relative_to(self.root_dir)),
                    'section': current_section
                })
            
            # Extract traditional markdown todos
            markdown_todos = re.findall(r'-\s*\[([ x])\]\s*(.+)', content)
            for completed, text in markdown_todos:
                # Try to extract assignee from text
                assignee_match = re.search(r'\(@([^)]+)\)', text)
                assignee = assignee_match.group(1) if assignee_match else 'unassigned'
                
                todos.append({
                    'type': 'TODO',
                    'assignee': assignee,
                    'priority': current_priority,
                    'text': text.strip(),
                    'file': str(file_path.relative_to(self.root_dir)),
                    'section': current_section,
                    'completed': completed == 'x'
                })
            
            # Extract section headers
            sections = re.findall(r'^#+\s*(.+)$', content, re.MULTILINE)
            if sections:
                current_section = sections[0]
        
        return todos
    
    def migrate_todos(self):
        """Migrate all found TODOs to the new format."""
        self.find_todo_files()
        
        for file_path in self.todo_files:
            print(f"Migrating: {file_path}")
            todos = self.parse_todo_file(file_path)
            self.migrated_todos.extend(todos)
    
    def generate_new_todo_file(self):
        """Generate new standardized TODO file."""
        now = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        
        # Count priorities
        critical = sum(1 for todo in self.migrated_todos if todo['priority'] == 'critical')
        high = sum(1 for todo in self.migrated_todos if todo['priority'] == 'high')
        normal = sum(1 for todo in self.migrated_todos if todo['priority'] == 'normal')
        low = sum(1 for todo in self.migrated_todos if todo['priority'] == 'low')
        
        report = f"""# TODO Report
Generated on: {now}

## Summary
- Total TODOs: {len(self.migrated_todos)}
- Critical: {critical}
- High Priority: {high}
- Normal: {normal}
- Low Priority: {low}

## Critical TODOs
"""
        
        # Add todos by priority
        for todo in [t for t in self.migrated_todos if t['priority'] == 'critical']:
            report += f"""- [ ] **{todo['type']}** ({todo['assignee']}): {todo['text']}
  - File: `{todo['file']}`
  - Section: {todo['section']}
"""
        
        report += "\n## High Priority\n"
        for todo in [t for t in self.migrated_todos if t['priority'] == 'high']:
            report += f"""- [ ] **{todo['type']}** ({todo['assignee']}): {todo['text']}
  - File: `{todo['file']}`
  - Section: {todo['section']}
"""
        
        report += "\n## Normal Priority\n"
        for todo in [t for t in self.migrated_todos if t['priority'] == 'normal']:
            report += f"""- [ ] **{todo['type']}** ({todo['assignee']}): {todo['text']}
  - File: `{todo['file']}`
  - Section: {todo['section']}
"""
        
        report += "\n## Low Priority\n"
        for todo in [t for t in self.migrated_todos if t['priority'] == 'low']:
            report += f"""- [ ] **{todo['type']}** ({todo['assignee']}): {todo['text']}
  - File: `{todo['file']}`
  - Section: {todo['section']}
"""
        
        report += """
## Notes
- This report is automatically generated
- TODOs are extracted from code comments
- Format: TODO(assignee)[priority]: description
- Priorities: [critical, high, normal, low]
- Types: TODO, FIXME, XXX, HACK, BUG, OPTIMIZE, NOTE

## Recent Changes
"""
        
        # Add recent changes
        changes = [
            {'date': now, 'description': 'Initial TODO migration and consolidation'}
        ]
        for change in changes:
            report += f"- {change['date']}: {change['description']}\n"
        
        # Save new TODO file
        new_todo_path = self.root_dir / 'docs' / 'TODO.md'
        new_todo_path.parent.mkdir(exist_ok=True)
        with open(new_todo_path, 'w') as f:
            f.write(report)
        
        # Save JSON for project board integration
        with open(self.root_dir / 'todos.json', 'w') as f:
            json.dump(self.migrated_todos, f, indent=2)
    
    def archive_old_todos(self):
        """Archive old TODO files."""
        archive_dir = self.root_dir / 'docs' / 'archived_todos'
        archive_dir.mkdir(exist_ok=True)
        
        for file_path in self.todo_files:
            if file_path != self.root_dir / 'docs' / 'TODO.md':
                archive_path = archive_dir / f"{file_path.stem}_{datetime.now().strftime('%Y%m%d')}{file_path.suffix}"
                with open(file_path, 'r') as src, open(archive_path, 'w') as dst:
                    dst.write(f"# Archived from {file_path} on {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n\n")
                    dst.write(src.read())
                print(f"Archived: {file_path} -> {archive_path}")

def main():
    """Main function."""
    try:
        repo_root = Path(os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..')))
        migrator = TodoMigration(str(repo_root))
        
        print("Starting TODO migration...")
        migrator.migrate_todos()
        
        print("Generating new TODO file...")
        migrator.generate_new_todo_file()
        
        print("Archiving old TODO files...")
        migrator.archive_old_todos()
        
        print("Migration complete!")
        print(f"New TODO file: {repo_root}/docs/TODO.md")
        print(f"Archived TODOs: {repo_root}/docs/archived_todos/")
        
    except Exception as e:
        print(f"Error during migration: {e}")
        raise

if __name__ == '__main__':
    main()

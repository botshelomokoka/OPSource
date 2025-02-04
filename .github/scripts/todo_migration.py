#!/usr/bin/env python3
"""
TODO Migration Script
Migrates and standardizes all TODO files across repositories to the new format.
"""

import os
import re
import yaml
import json
import argparse
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional, Set
import fnmatch

class TodoMigrator:
    """Handles migration of TODO files to standardized format."""
    
    def __init__(self, root_dir: str, force: bool = False):
        self.root_dir = Path(root_dir)
        self.migrated_todos = []
        self.config = self._load_config()
        self.force = force
        self.known_teams = {
            'wallet-team', 'rgb-team', 'lightning-team', 'security-team',
            'infra-team', 'qa-team', 'dev-team', 'core-team'
        }
    
    def _load_config(self) -> Dict:
        """Load configuration from git-config.yml."""
        config_path = self.root_dir / '.github' / 'git-config.yml'
        try:
            with open(config_path, 'r') as f:
                config = yaml.safe_load(f)
                return config['documentation']['todo_tracking']
        except Exception as e:
            print(f"Error loading config from {config_path}: {e}")
            print("Using default configuration...")
            return {
                'format': {
                    'pattern': r'^(?:TODO|FIXME|BUG|HACK|NOTE|OPTIMIZE)\(([^)]+)\)\[([^]]+)\]:\s*(.+)$',
                    'priorities': ['p0-critical', 'p1-high', 'p2-normal', 'p3-low']
                },
                'tracking': {
                    'locations': ['**.{py,rs,go,js,ts,dart}'],
                    'report': {
                        'path': 'docs/TODO.md',
                        'sections': [
                            {'title': 'Critical (P0)', 'priority': 'p0-critical'},
                            {'title': 'High Priority (P1)', 'priority': 'p1-high'},
                            {'title': 'Normal Priority (P2)', 'priority': 'p2-normal'},
                            {'title': 'Low Priority (P3)', 'priority': 'p3-low'}
                        ]
                    }
                }
            }
    
    def _normalize_priority(self, priority: str) -> str:
        """Normalize priority to standardized format."""
        priority = priority.lower()
        if priority in ['critical', 'p0', 'p0-critical']:
            return 'p0-critical'
        elif priority in ['high', 'p1', 'p1-high']:
            return 'p1-high'
        elif priority in ['normal', 'p2', 'p2-normal']:
            return 'p2-normal'
        elif priority in ['low', 'p3', 'p3-low']:
            return 'p3-low'
        return 'p2-normal'
    
    def _normalize_assignee(self, assignee: str) -> str:
        """Normalize assignee to team name if possible."""
        assignee = assignee.lower()
        if any(team.startswith(assignee) for team in self.known_teams):
            return next(team for team in self.known_teams if team.startswith(assignee))
        return assignee
    
    def _parse_todo_line(self, line: str, file_path: Path, line_number: int) -> Optional[Dict]:
        """Parse a TODO line and extract relevant information."""
        # Try standard format first
        pattern = self.config['format']['pattern']
        try:
            # Remove leading comment characters and whitespace
            clean_line = re.sub(r'^[\s#/*-]*', '', line.strip())
            
            # Try standard format
            match = re.search(pattern, clean_line)
            if match:
                assignee = self._normalize_assignee(match.group(1))
                priority = self._normalize_priority(match.group(2))
                text = match.group(3).strip()
                return {
                    'type': 'TODO',
                    'assignee': assignee,
                    'priority': priority,
                    'text': text,
                    'file': str(file_path.relative_to(self.root_dir)),
                    'line': line_number,
                    'created': datetime.now().strftime('%Y-%m-%d')
                }
            
            # Try markdown task format
            md_match = re.match(r'-\s*\[\s*[xX ]?\s*\]\s*(?:\(@([^)]+)\))?\s*(.+)', clean_line)
            if md_match:
                assignee = self._normalize_assignee(md_match.group(1) or 'unassigned')
                text = md_match.group(2).strip()
                # Try to extract priority from text
                priority_match = re.search(r'\[(p[0-3]|critical|high|normal|low)\]', text)
                priority = self._normalize_priority(priority_match.group(1) if priority_match else 'normal')
                text = re.sub(r'\[(p[0-3]|critical|high|normal|low)\]', '', text).strip()
                return {
                    'type': 'TODO',
                    'assignee': assignee,
                    'priority': priority,
                    'text': text,
                    'file': str(file_path.relative_to(self.root_dir)),
                    'line': line_number,
                    'created': datetime.now().strftime('%Y-%m-%d')
                }
            
            # Try old format (TODO: description)
            old_match = re.match(r'^(?:TODO|FIXME|BUG|HACK|NOTE|OPTIMIZE):\s*(.+)', clean_line)
            if old_match:
                text = old_match.group(1).strip()
                return {
                    'type': 'TODO',
                    'assignee': 'unassigned',
                    'priority': 'p2-normal',
                    'text': text,
                    'file': str(file_path.relative_to(self.root_dir)),
                    'line': line_number,
                    'created': datetime.now().strftime('%Y-%m-%d')
                }
                
        except re.error as e:
            print(f"Error in regex pattern for line '{line}': {e}")
        return None

    def find_todos(self):
        """Find TODOs in both code and documentation files."""
        print("Finding TODOs in codebase...")
        
        # Process code files
        for root, dirs, files in os.walk(self.root_dir):
            # Skip excluded directories
            dirs[:] = [d for d in dirs if not any(excluded in d for excluded in [
                'vendor', 'node_modules', 'target', '.git', '__pycache__', 'dist', 'build'
            ])]
            
            for file in files:
                file_path = Path(root) / file
                rel_path = file_path.relative_to(self.root_dir)
                
                # Skip binary and generated files
                if file_path.suffix in ['.pyc', '.pyo', '.so', '.dll', '.class']:
                    continue
                
                try:
                    print(f"Checking file: {file_path}")
                    with open(file_path, 'r', encoding='utf-8') as f:
                        for i, line in enumerate(f, 1):
                            if any(marker in line for marker in ['TODO', 'FIXME', 'BUG', 'HACK', 'NOTE', 'OPTIMIZE']):
                                todo = self._parse_todo_line(line, file_path, i)
                                if todo:
                                    print(f"Found TODO in {file_path}:{i} - {todo['text']}")
                                    self.migrated_todos.append(todo)
                except (UnicodeDecodeError, IOError) as e:
                    print(f"Error reading {file_path}: {e}")
    
    def migrate_archived_todos(self):
        """Migrate TODOs from archived files."""
        print("Migrating archived TODOs...")
        archive_dir = self.root_dir / 'docs' / 'archived_todos'
        if not archive_dir.exists():
            return
        
        for file_path in archive_dir.glob('*.md'):
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    current_section = None
                    for i, line in enumerate(f, 1):
                        # Track sections for context
                        if line.startswith('#'):
                            current_section = line.strip('# ').strip()
                            continue
                        
                        # Look for TODOs
                        if any(marker in line for marker in ['TODO', 'FIXME', 'BUG', 'HACK', 'NOTE', 'OPTIMIZE', '- [ ]']):
                            todo = self._parse_todo_line(line, file_path, i)
                            if todo:
                                todo['section'] = current_section
                                print(f"Found archived TODO: {todo['text']}")
                                self.migrated_todos.append(todo)
            except Exception as e:
                print(f"Error processing archived file {file_path}: {e}")
    
    def generate_todo_report(self):
        """Generate a formatted TODO report."""
        print("Generating TODO report...")
        
        if not self.migrated_todos and not self.force:
            print("No TODOs found to migrate. Use --force to generate report anyway.")
            return
            
        # Create report directory if it doesn't exist
        report_path = self.root_dir / self.config['tracking']['report']['path']
        report_path.parent.mkdir(parents=True, exist_ok=True)
        
        # Group TODOs by section and priority
        todos_by_section = {}
        for todo in self.migrated_todos:
            section = todo.get('section', 'Uncategorized')
            if section not in todos_by_section:
                todos_by_section[section] = {
                    'p0-critical': [],
                    'p1-high': [],
                    'p2-normal': [],
                    'p3-low': []
                }
            todos_by_section[section][todo['priority']].append(todo)
        
        # Generate markdown report
        with open(report_path, 'w') as f:
            f.write(f"# TODO Report\nGenerated on: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n\n")
            
            # Write summary
            total = len(self.migrated_todos)
            critical = sum(1 for t in self.migrated_todos if t['priority'] == 'p0-critical')
            high = sum(1 for t in self.migrated_todos if t['priority'] == 'p1-high')
            normal = sum(1 for t in self.migrated_todos if t['priority'] == 'p2-normal')
            low = sum(1 for t in self.migrated_todos if t['priority'] == 'p3-low')
            
            f.write("## Summary\n")
            f.write(f"- Total TODOs: {total}\n")
            f.write(f"- Critical (P0): {critical}\n")
            f.write(f"- High Priority (P1): {high}\n")
            f.write(f"- Normal Priority (P2): {normal}\n")
            f.write(f"- Low Priority (P3): {low}\n\n")
            
            # Write TODOs by section
            for section, priorities in todos_by_section.items():
                if any(todos for todos in priorities.values()):
                    f.write(f"## {section}\n\n")
                    
                    # Write TODOs by priority
                    for priority, todos in priorities.items():
                        if todos:
                            title = priority.replace('p0-', 'Critical (').replace('p1-', 'High Priority (').replace('p2-', 'Normal Priority (').replace('p3-', 'Low Priority (')
                            f.write(f"### {title})\n\n")
                            for todo in sorted(todos, key=lambda x: x['text']):
                                rel_path = todo['file']
                                f.write(f"- [ ] TODO({todo['assignee']})[{todo['priority']}]: {todo['text']}\n")
                                if 'line' in todo:
                                    f.write(f"  - File: `{rel_path}:{todo['line']}`\n")
                            f.write("\n")
            
            # Write notes
            f.write("## Notes\n")
            f.write("- Format: TODO(assignee)[priority]: description\n")
            f.write("- Priorities: [p0-critical, p1-high, p2-normal, p3-low]\n")
            f.write("- Types: TODO, FIXME, BUG, HACK, NOTE, OPTIMIZE\n")
            f.write("- Assignees should be GitHub usernames or team names\n\n")
            
            # Write recent changes
            f.write("## Recent Changes\n")
            f.write(f"- {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}: Updated TODO report\n")
            f.write(f"- {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}: Migrated archived TODOs\n")
            f.write(f"- {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}: Standardized TODO format\n")
        
        print(f"New TODO file: {report_path}")
    
    def archive_old_todos(self):
        """Archive old TODO files."""
        print("Archiving old TODO files...")
        archive_dir = self.root_dir / 'docs' / 'archived_todos'
        archive_dir.mkdir(parents=True, exist_ok=True)
        
        date_str = datetime.now().strftime('%Y%m%d')
        for root, _, files in os.walk(self.root_dir):
            if any(excluded in root for excluded in ['.git', 'node_modules', 'venv', 'archived_todos']):
                continue
            
            for file in files:
                if file.lower() in ['todo.md', 'todos.md', 'todo.txt']:
                    file_path = Path(root) / file
                    if file_path != self.root_dir / 'docs' / 'TODO.md':
                        archive_path = archive_dir / f"{file_path.stem}_{date_str}{file_path.suffix}"
                        print(f"Archiving: {file_path} -> {archive_path}")
                        try:
                            with open(file_path, 'r') as src, open(archive_path, 'w') as dst:
                                dst.write(f"# Archived from {file_path} on {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n\n")
                                dst.write(src.read())
                        except Exception as e:
                            print(f"Error archiving {file_path}: {e}")
        
        print(f"Archived TODOs: {archive_dir}/")

def main():
    """Main function."""
    parser = argparse.ArgumentParser(description='Migrate and standardize TODO files across repositories.')
    parser.add_argument('--force', action='store_true', help='Force migration and report generation even if no changes detected')
    args = parser.parse_args()
    
    try:
        repo_root = Path(__file__).resolve().parent.parent.parent
        migrator = TodoMigrator(str(repo_root), force=args.force)
        
        print("Starting TODO migration...")
        migrator.find_todos()
        migrator.migrate_archived_todos()
        migrator.generate_todo_report()
        migrator.archive_old_todos()
        
        print("Migration complete!")
        
    except Exception as e:
        print(f"Error during migration: {e}")
        raise

if __name__ == '__main__':
    main()

#!/usr/bin/env python3
"""Document Manager for managing documentation, roadmaps, changelogs, and TODOs."""

import os
import re
import json
import yaml
import argparse
from datetime import datetime
from typing import Dict, List, Optional

class DocManager:
    """Manages documentation across repositories."""
    
    def __init__(self):
        self.config = self._load_config()
        self.root_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..'))
        
    def _load_config(self) -> Dict:
        """Load configuration from git-config.yml."""
        config_path = os.path.join(os.path.dirname(__file__), '..', 'git-config.yml')
        with open(config_path, 'r') as f:
            return yaml.safe_load(f)
    
    def sync_docs(self):
        """Synchronize documentation across repositories."""
        # Process main documentation
        self._process_main_docs()
        # Process submodule documentation
        self._process_submodule_docs()
        
    def _process_main_docs(self):
        """Process documentation in the main repository."""
        docs_dir = os.path.join(self.root_dir, 'docs')
        os.makedirs(docs_dir, exist_ok=True)
        
        # Create mkdocs.yml if it doesn't exist
        mkdocs_path = os.path.join(self.root_dir, 'mkdocs.yml')
        if not os.path.exists(mkdocs_path):
            self._create_mkdocs_config()
    
    def _process_submodule_docs(self):
        """Process documentation in submodules."""
        for submodule in self._get_submodules():
            submodule_path = os.path.join(self.root_dir, submodule)
            if os.path.exists(submodule_path):
                self._sync_submodule_docs(submodule_path)
    
    def _sync_submodule_docs(self, submodule_path: str):
        """Synchronize documentation from a submodule."""
        docs_dir = os.path.join(submodule_path, 'docs')
        if os.path.exists(docs_dir):
            target_dir = os.path.join(self.root_dir, 'docs', os.path.basename(submodule_path))
            os.makedirs(target_dir, exist_ok=True)
            # Copy docs while maintaining structure
            self._copy_docs(docs_dir, target_dir)
    
    def process_roadmap(self):
        """Process and combine roadmaps from all repositories."""
        combined_roadmap = "# Project Roadmap\n\n"
        combined_roadmap += f"Last updated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n\n"
        
        # Process main roadmap
        main_roadmap = os.path.join(self.root_dir, 'ROADMAP.md')
        if os.path.exists(main_roadmap):
            with open(main_roadmap, 'r') as f:
                combined_roadmap += "## Main Project\n\n"
                combined_roadmap += f.read() + "\n\n"
        
        # Process submodule roadmaps
        for submodule in self._get_submodules():
            submodule_path = os.path.join(self.root_dir, submodule)
            roadmap_path = os.path.join(submodule_path, 'ROADMAP.md')
            if os.path.exists(roadmap_path):
                with open(roadmap_path, 'r') as f:
                    combined_roadmap += f"## {os.path.basename(submodule)}\n\n"
                    combined_roadmap += f.read() + "\n\n"
        
        # Save combined roadmap
        with open(os.path.join(self.root_dir, 'docs', 'ROADMAP.md'), 'w') as f:
            f.write(combined_roadmap)
    
    def process_changelog(self):
        """Process and combine changelogs from all repositories."""
        combined_changelog = "# Project Changelog\n\n"
        combined_changelog += f"Last updated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n\n"
        
        # Process main changelog
        main_changelog = os.path.join(self.root_dir, 'CHANGELOG.md')
        if os.path.exists(main_changelog):
            with open(main_changelog, 'r') as f:
                combined_changelog += "## Main Project\n\n"
                combined_changelog += f.read() + "\n\n"
        
        # Process submodule changelogs
        for submodule in self._get_submodules():
            submodule_path = os.path.join(self.root_dir, submodule)
            changelog_path = os.path.join(submodule_path, 'CHANGELOG.md')
            if os.path.exists(changelog_path):
                with open(changelog_path, 'r') as f:
                    combined_changelog += f"## {os.path.basename(submodule)}\n\n"
                    combined_changelog += f.read() + "\n\n"
        
        # Save combined changelog
        with open(os.path.join(self.root_dir, 'docs', 'CHANGELOG.md'), 'w') as f:
            f.write(combined_changelog)
    
    def process_todos(self):
        """Process TODOs and integrate with roadmap/changelog."""
        todos = self._scan_todos()
        self._update_roadmap_with_todos(todos)
        self._save_todos(todos)
    
    def _scan_todos(self) -> List[Dict]:
        """Scan for TODOs in all repositories."""
        todos = []
        
        # Scan patterns from config
        patterns = self.config['documentation']['todo_tracking']['scan_patterns']
        todo_pattern = r'#\s*(' + '|'.join(patterns) + r')\([\w-]+\)\[(critical|high|normal|low)\]:\s*(.+)$'
        
        # Scan main repository
        todos.extend(self._scan_directory(self.root_dir, todo_pattern))
        
        # Scan submodules
        for submodule in self._get_submodules():
            submodule_path = os.path.join(self.root_dir, submodule)
            if os.path.exists(submodule_path):
                todos.extend(self._scan_directory(submodule_path, todo_pattern))
        
        return todos
    
    def _scan_directory(self, directory: str, pattern: str) -> List[Dict]:
        """Scan a directory for TODOs."""
        todos = []
        for root, _, files in os.walk(directory):
            if any(excluded in root for excluded in ['.git', 'node_modules', 'venv']):
                continue
            
            for file in files:
                if file.endswith(tuple(self.config['documentation']['todo_tracking']['scan_extensions'])):
                    file_path = os.path.join(root, file)
                    with open(file_path, 'r', encoding='utf-8') as f:
                        for i, line in enumerate(f, 1):
                            match = re.search(pattern, line)
                            if match:
                                todos.append({
                                    'type': match.group(1),
                                    'assignee': re.search(r'\(([\w-]+)\)', line).group(1),
                                    'priority': re.search(r'\[([\w-]+)\]', line).group(1),
                                    'text': match.group(3),
                                    'file': os.path.relpath(file_path, self.root_dir),
                                    'line': i
                                })
        return todos
    
    def _update_roadmap_with_todos(self, todos: List[Dict]):
        """Update roadmap with high-priority TODOs."""
        roadmap_path = os.path.join(self.root_dir, 'docs', 'ROADMAP.md')
        if os.path.exists(roadmap_path):
            with open(roadmap_path, 'r') as f:
                roadmap = f.read()
            
            # Add TODO section
            todo_section = "\n## TODOs\n\n"
            for todo in sorted(todos, key=lambda x: {'critical': 0, 'high': 1, 'normal': 2, 'low': 3}[x['priority']]):
                if todo['priority'] in ['critical', 'high']:
                    todo_section += f"- [{todo['priority']}] {todo['text']} ({todo['file']}:{todo['line']})\n"
            
            if "## TODOs" not in roadmap:
                roadmap += todo_section
            else:
                roadmap = re.sub(r'## TODOs.*?(?=##|$)', todo_section, roadmap, flags=re.DOTALL)
            
            with open(roadmap_path, 'w') as f:
                f.write(roadmap)
    
    def _save_todos(self, todos: List[Dict]):
        """Save TODOs to JSON for project board integration."""
        with open(os.path.join(self.root_dir, 'todos.json'), 'w') as f:
            json.dump(todos, f, indent=2)
    
    def _get_submodules(self) -> List[str]:
        """Get list of submodules."""
        submodules = []
        gitmodules_path = os.path.join(self.root_dir, '.gitmodules')
        if os.path.exists(gitmodules_path):
            with open(gitmodules_path, 'r') as f:
                content = f.read()
                submodules = re.findall(r'path\s*=\s*(.+)', content)
        return submodules
    
    def _create_mkdocs_config(self):
        """Create mkdocs.yml configuration."""
        config = {
            'site_name': 'Project Documentation',
            'theme': 'material',
            'nav': [
                {'Home': 'index.md'},
                {'Roadmap': 'ROADMAP.md'},
                {'Changelog': 'CHANGELOG.md'},
                {'TODOs': 'TODO.md'}
            ],
            'plugins': ['search'],
            'markdown_extensions': [
                'pymdownx.highlight',
                'pymdownx.superfences'
            ]
        }
        
        with open(os.path.join(self.root_dir, 'mkdocs.yml'), 'w') as f:
            yaml.dump(config, f, default_flow_style=False)

def main():
    """Main function."""
    parser = argparse.ArgumentParser(description='Document Manager')
    parser.add_argument('--sync', action='store_true', help='Sync documentation')
    parser.add_argument('--roadmap', action='store_true', help='Process roadmap')
    parser.add_argument('--changelog', action='store_true', help='Process changelog')
    parser.add_argument('--todos', action='store_true', help='Process TODOs')
    
    args = parser.parse_args()
    manager = DocManager()
    
    if args.sync:
        manager.sync_docs()
    if args.roadmap:
        manager.process_roadmap()
    if args.changelog:
        manager.process_changelog()
    if args.todos:
        manager.process_todos()

if __name__ == '__main__':
    main()

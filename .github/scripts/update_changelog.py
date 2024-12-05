#!/usr/bin/env python3
"""
Update CHANGELOG.md based on TODO changes and git commits.
"""

import os
from datetime import datetime
from pathlib import Path
import re

def parse_todo_changes(todo_file):
    """Parse TODO.md for changes since last update."""
    changes = []
    try:
        with open(todo_file, 'r') as f:
            content = f.read()
            # Extract recent changes section
            recent_changes = re.search(r'## Recent Changes\n(.*?)(?:\n#|$)', content, re.DOTALL)
            if recent_changes:
                changes = [line.strip('- ') for line in recent_changes.group(1).strip().split('\n') if line.strip()]
    except Exception as e:
        print(f"Error parsing TODO file: {e}")
    return changes

def update_changelog(repo_root):
    """Update CHANGELOG.md with TODO changes."""
    changelog_path = repo_root / 'docs' / 'CHANGELOG.md'
    todo_path = repo_root / 'docs' / 'TODO.md'
    
    # Get TODO changes
    todo_changes = parse_todo_changes(todo_path)
    
    # Read existing changelog
    try:
        with open(changelog_path, 'r') as f:
            content = f.read()
    except FileNotFoundError:
        content = "# Changelog\n\nAll notable changes to this project will be documented in this file.\n\n"
    
    # Add new changes
    today = datetime.now().strftime('%Y-%m-%d')
    new_changes = f"\n## [{today}]\n\n### Changed\n"
    for change in todo_changes:
        new_changes += f"- {change}\n"
    
    # Insert new changes after header
    if '# Changelog' in content:
        content = content.replace('# Changelog\n', f"# Changelog\n{new_changes}")
    else:
        content = f"# Changelog\n{new_changes}\n{content}"
    
    # Write updated changelog
    with open(changelog_path, 'w') as f:
        f.write(content)
    
    print(f"Updated changelog at {changelog_path}")

def main():
    """Main function."""
    try:
        repo_root = Path(__file__).resolve().parent.parent.parent
        update_changelog(repo_root)
    except Exception as e:
        print(f"Error updating changelog: {e}")
        raise

if __name__ == '__main__':
    main()

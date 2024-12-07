#!/usr/bin/env python3
"""
Update ROADMAP.md based on TODO priorities and assignments.
"""

import os
from datetime import datetime
from pathlib import Path
import re
import json
from collections import defaultdict

class RoadmapGenerator:
    """Generates project roadmap from TODO items."""
    
    def __init__(self, repo_root):
        self.repo_root = Path(repo_root)
        self.todos = []
        self.quarters = {
            'Q1': ['January', 'February', 'March'],
            'Q2': ['April', 'May', 'June'],
            'Q3': ['July', 'August', 'September'],
            'Q4': ['October', 'November', 'December']
        }
    
    def parse_todos(self):
        """Parse TODO.md for all TODO items."""
        todo_path = self.repo_root / 'docs' / 'TODO.md'
        try:
            with open(todo_path, 'r') as f:
                content = f.read()
                # Extract all TODO items with their priorities
                todo_pattern = r'- \[ \] TODO\(([^)]+)\)\[([^]]+)\]:\s*(.+?)\n'
                matches = re.finditer(todo_pattern, content)
                for match in matches:
                    assignee = match.group(1)
                    priority = match.group(2)
                    description = match.group(3)
                    self.todos.append({
                        'assignee': assignee,
                        'priority': priority,
                        'description': description.strip()
                    })
        except Exception as e:
            print(f"Error parsing TODO file: {e}")
    
    def generate_roadmap(self):
        """Generate roadmap content based on TODOs."""
        now = datetime.now()
        current_quarter = f"Q{(now.month - 1) // 3 + 1}"
        current_year = now.year
        
        # Group TODOs by priority
        priority_groups = defaultdict(list)
        for todo in self.todos:
            priority_groups[todo['priority']].append(todo)
        
        # Generate roadmap content
        content = f"""# Project Roadmap

Last updated: {now.strftime('%Y-%m-%d')}

## Current Focus ({current_quarter} {current_year})

### Critical Priority (P0)
"""
        
        # Add P0 items
        p0_items = priority_groups.get('p0-critical', [])
        if p0_items:
            for item in p0_items:
                content += f"- {item['description']} (Assigned: {item['assignee']})\n"
        else:
            content += "- No critical items currently\n"
        
        content += "\n### High Priority (P1)\n"
        p1_items = priority_groups.get('p1-high', [])
        if p1_items:
            for item in p1_items:
                content += f"- {item['description']} (Assigned: {item['assignee']})\n"
        else:
            content += "- No high priority items currently\n"
        
        # Add future quarters
        quarters = list(self.quarters.keys())
        current_quarter_idx = quarters.index(current_quarter)
        future_quarters = quarters[current_quarter_idx+1:] + quarters[:current_quarter_idx]
        
        content += f"\n## Future Plans\n"
        
        # Distribute remaining TODOs across future quarters
        normal_items = priority_groups.get('p2-normal', [])
        low_items = priority_groups.get('p3-low', [])
        
        items_per_quarter = max(len(normal_items) // 3, 1)  # Distribute over next 3 quarters
        
        for i, quarter in enumerate(future_quarters[:3]):
            year = current_year + ((now.month + i*3) // 12)
            content += f"\n### {quarter} {year}\n"
            
            start_idx = i * items_per_quarter
            end_idx = start_idx + items_per_quarter
            quarter_items = normal_items[start_idx:end_idx]
            
            if quarter_items:
                for item in quarter_items:
                    content += f"- {item['description']} (Assigned: {item['assignee']})\n"
            else:
                content += "- Planning in progress\n"
        
        content += "\n## Long-term Goals\n"
        if low_items:
            for item in low_items:
                content += f"- {item['description']} (Assigned: {item['assignee']})\n"
        else:
            content += "- Long-term planning in progress\n"
        
        return content
    
    def update_roadmap(self):
        """Update ROADMAP.md file."""
        self.parse_todos()
        content = self.generate_roadmap()
        
        roadmap_path = self.repo_root / 'docs' / 'ROADMAP.md'
        with open(roadmap_path, 'w') as f:
            f.write(content)
        
        print(f"Updated roadmap at {roadmap_path}")

def main():
    """Main function."""
    try:
        repo_root = Path(__file__).resolve().parent.parent.parent
        generator = RoadmapGenerator(repo_root)
        generator.update_roadmap()
    except Exception as e:
        print(f"Error updating roadmap: {e}")
        raise

if __name__ == '__main__':
    main()

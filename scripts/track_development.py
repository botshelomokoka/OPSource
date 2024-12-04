#!/usr/bin/env python3
import os
import sys
from datetime import datetime, timedelta
from pathlib import Path
import json
from git import Repo

def get_repo_stats(repo_path):
    """Get statistics for a Git repository."""
    repo = Repo(repo_path)
    stats = {
        'commits': [],
        'authors': set(),
        'files_changed': set(),
        'additions': 0,
        'deletions': 0
    }
    
    # Get commits from the last week
    since = datetime.now() - timedelta(days=7)
    
    for commit in repo.iter_commits():
        commit_date = datetime.fromtimestamp(commit.committed_date)
        if commit_date < since:
            break
            
        stats['commits'].append({
            'hash': commit.hexsha,
            'author': commit.author.name,
            'date': commit_date.isoformat(),
            'message': commit.message
        })
        stats['authors'].add(commit.author.name)
        
        # Get file changes
        if len(commit.parents) > 0:
            diff_index = commit.parents[0].diff(commit, create_patch=True)
            for diff in diff_index:
                stats['files_changed'].add(diff.a_path or diff.b_path)
                # Count lines changed
                if diff.a_blob and diff.b_blob:
                    for line in diff.diff.decode().split('\n'):
                        if line.startswith('+') and not line.startswith('+++'):
                            stats['additions'] += 1
                        elif line.startswith('-') and not line.startswith('---'):
                            stats['deletions'] += 1
    
    return {
        'commit_count': len(stats['commits']),
        'author_count': len(stats['authors']),
        'files_changed': len(stats['files_changed']),
        'additions': stats['additions'],
        'deletions': stats['deletions'],
        'commits': stats['commits']
    }

def generate_report(stats, output_dir):
    """Generate development tracking reports."""
    # Ensure reports directory exists
    reports_dir = Path(output_dir)
    reports_dir.mkdir(parents=True, exist_ok=True)
    
    # Save detailed JSON report
    with open(reports_dir / 'development_stats.json', 'w') as f:
        json.dump(stats, f, indent=2)
    
    # Generate markdown summary
    timestamp = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
    
    summary_lines = [
        "# Development Tracking Report",
        f"Generated: {timestamp}",
        "",
        "## Weekly Statistics"
    ]
    
    for repo_name, repo_stats in stats.items():
        summary_lines.extend([
            f"### {repo_name}",
            f"- Total Commits: {repo_stats['commit_count']}",
            f"- Active Authors: {repo_stats['author_count']}",
            f"- Files Changed: {repo_stats['files_changed']}",
            f"- Lines Added: {repo_stats['additions']}",
            f"- Lines Deleted: {repo_stats['deletions']}",
            ""
        ])
        
        if repo_stats['commits']:
            summary_lines.append("#### Recent Commits")
            for commit in repo_stats['commits'][:5]:
                summary_lines.extend([
                    f"- {commit['hash'][:8]} - {commit['message'].split()[0]}",
                    f"  Author: {commit['author']}, Date: {commit['date']}",
                    ""
                ])
    
    with open(reports_dir / 'development_summary.md', 'w') as f:
        f.write('\n'.join(summary_lines))

def main():
    # Base directory containing all repositories
    base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    
    # Get stats for main repository and submodules
    all_stats = {}
    
    # Main repository
    try:
        main_stats = get_repo_stats(base_dir)
        all_stats['main'] = main_stats
    except Exception as e:
        print(f"Error processing main repository: {e}", file=sys.stderr)
    
    # Submodules
    submodules_dir = os.path.join(base_dir, 'anya')
    if os.path.exists(submodules_dir):
        for submodule in os.listdir(submodules_dir):
            submodule_path = os.path.join(submodules_dir, submodule)
            if os.path.isdir(submodule_path) and os.path.exists(os.path.join(submodule_path, '.git')):
                try:
                    submodule_stats = get_repo_stats(submodule_path)
                    all_stats[submodule] = submodule_stats
                except Exception as e:
                    print(f"Error processing submodule {submodule}: {e}", file=sys.stderr)
    
    # Generate reports
    reports_dir = os.path.join(base_dir, 'reports')
    generate_report(all_stats, reports_dir)
    print(f"Reports generated in {reports_dir}")

if __name__ == '__main__':
    main()

#!/usr/bin/env python3
"""
Script to analyze Bitcoin implementation comparison logs and generate a report.
"""

import os
import sys
import re
import json
from datetime import datetime
from collections import defaultdict

def parse_log_entry(entry):
    """Parse a log entry into a structured format."""
    # Extract timestamp and operation
    timestamp_match = re.search(r'\[(.*?)\] Operation: (.*)', entry)
    if not timestamp_match:
        return None
    
    timestamp = timestamp_match.group(1)
    operation = timestamp_match.group(2)
    
    # Extract Python and Rust results
    python_match = re.search(r'Python: (.*)', entry)
    rust_match = re.search(r'Rust: (.*)', entry)
    match_match = re.search(r'Match: (.*)', entry)
    
    if not (python_match and rust_match and match_match):
        return None
    
    python_result = python_match.group(1)
    rust_result = rust_match.group(1)
    match = match_match.group(1).lower() == 'true'
    
    return {
        'timestamp': timestamp,
        'operation': operation,
        'python_result': python_result,
        'rust_result': rust_result,
        'match': match
    }

def analyze_log_file(log_file):
    """Analyze a single log file and return statistics."""
    with open(log_file, 'r') as f:
        content = f.read()
    
    # Split into entries (separated by double newlines)
    entries = content.split('\n\n')
    
    # Parse each entry
    parsed_entries = []
    for entry in entries:
        if entry.strip():
            parsed_entry = parse_log_entry(entry)
            if parsed_entry:
                parsed_entries.append(parsed_entry)
    
    # Calculate statistics
    total_operations = len(parsed_entries)
    matching_operations = sum(1 for entry in parsed_entries if entry['match'])
    mismatch_operations = total_operations - matching_operations
    
    # Group by operation type
    operation_stats = defaultdict(lambda: {'total': 0, 'matching': 0, 'mismatching': 0})
    for entry in parsed_entries:
        op_type = entry['operation'].split('(')[0]
        operation_stats[op_type]['total'] += 1
        if entry['match']:
            operation_stats[op_type]['matching'] += 1
        else:
            operation_stats[op_type]['mismatching'] += 1
    
    return {
        'total_operations': total_operations,
        'matching_operations': matching_operations,
        'mismatch_operations': mismatch_operations,
        'match_percentage': (matching_operations / total_operations * 100) if total_operations > 0 else 0,
        'operation_stats': dict(operation_stats),
        'entries': parsed_entries
    }

def generate_report(log_dir, output_file=None):
    """Generate a report from all log files in the directory."""
    # Find all log files
    log_files = [os.path.join(log_dir, f) for f in os.listdir(log_dir) if f.startswith('shadow_') and f.endswith('.log')]
    
    if not log_files:
        print(f"No log files found in {log_dir}")
        return
    
    # Analyze each log file
    results = {}
    for log_file in log_files:
        results[os.path.basename(log_file)] = analyze_log_file(log_file)
    
    # Generate report
    report = {
        'generated_at': datetime.now().isoformat(),
        'log_files': len(log_files),
        'results': results
    }
    
    # Calculate overall statistics
    total_operations = sum(r['total_operations'] for r in results.values())
    matching_operations = sum(r['matching_operations'] for r in results.values())
    
    report['overall'] = {
        'total_operations': total_operations,
        'matching_operations': matching_operations,
        'mismatch_operations': total_operations - matching_operations,
        'match_percentage': (matching_operations / total_operations * 100) if total_operations > 0 else 0
    }
    
    # Write report to file or stdout
    if output_file:
        with open(output_file, 'w') as f:
            json.dump(report, f, indent=2)
    
    # Print summary
    print("\n=== Bitcoin Implementation Comparison Report ===")
    print(f"Generated at: {report['generated_at']}")
    print(f"Log files analyzed: {report['log_files']}")
    print(f"Total operations: {report['overall']['total_operations']}")
    print(f"Matching operations: {report['overall']['matching_operations']} ({report['overall']['match_percentage']:.2f}%)")
    print(f"Mismatching operations: {report['overall']['mismatch_operations']}")
    
    # Print operation-specific statistics
    print("\nOperation-specific statistics:")
    for log_file, result in results.items():
        print(f"\n{log_file}:")
        for op_type, stats in result['operation_stats'].items():
            match_pct = (stats['matching'] / stats['total'] * 100) if stats['total'] > 0 else 0
            print(f"  {op_type}: {stats['matching']}/{stats['total']} matching ({match_pct:.2f}%)")
    
    # Print mismatches
    print("\nMismatches:")
    for log_file, result in results.items():
        mismatches = [e for e in result['entries'] if not e['match']]
        if mismatches:
            print(f"\n{log_file}:")
            for mismatch in mismatches:
                print(f"  {mismatch['operation']}:")
                print(f"    Python: {mismatch['python_result']}")
                print(f"    Rust: {mismatch['rust_result']}")
    
    return report

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: python analyze_comparison_logs.py <log_directory> [output_file]")
        sys.exit(1)
    
    log_dir = sys.argv[1]
    output_file = sys.argv[2] if len(sys.argv) > 2 else None
    
    generate_report(log_dir, output_file) 
# TODO Report
Generated on: {{ date }}

## Summary
- Total TODOs: {{ total_count }}
- Critical: {{ critical_count }}
- High Priority: {{ high_count }}
- Normal: {{ normal_count }}
- Low Priority: {{ low_count }}

## Critical TODOs
{% for todo in critical_todos %}
- [ ] **{{ todo.type }}** ({{ todo.assignee }}): {{ todo.text }}
  - File: `{{ todo.file }}:{{ todo.line }}`
  - Created: {{ todo.created }}
  - Priority: {{ todo.priority }}
{% endfor %}

## High Priority
{% for todo in high_todos %}
- [ ] **{{ todo.type }}** ({{ todo.assignee }}): {{ todo.text }}
  - File: `{{ todo.file }}:{{ todo.line }}`
  - Created: {{ todo.created }}
{% endfor %}

## Normal Priority
{% for todo in normal_todos %}
- [ ] **{{ todo.type }}** ({{ todo.assignee }}): {{ todo.text }}
  - File: `{{ todo.file }}:{{ todo.line }}`
  - Created: {{ todo.created }}
{% endfor %}

## Low Priority
{% for todo in low_todos %}
- [ ] **{{ todo.type }}** ({{ todo.assignee }}): {{ todo.text }}
  - File: `{{ todo.file }}:{{ todo.line }}`
  - Created: {{ todo.created }}
{% endfor %}

## Notes
- This report is automatically generated
- TODOs are extracted from code comments
- Format: TODO(assignee)[priority]: description
- Priorities: [critical, high, normal, low]
- Types: TODO, FIXME, XXX, HACK, BUG, OPTIMIZE, NOTE

## Recent Changes
{% for change in recent_changes %}
- {{ change.date }}: {{ change.description }}
{% endfor %}

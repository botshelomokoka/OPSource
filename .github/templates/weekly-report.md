# Weekly Development Report
Generated on: {{ date }}

## Development Status

### Active Development Branches
{% for branch in development_branches %}
#### {{ branch.name }}
- Last commit: {{ branch.last_commit }}
- Last updated: {{ branch.last_updated }}
- Status: {{ branch.status }}
- Failed checks: {{ branch.failed_checks | join(', ') }}
{% endfor %}

### Release Candidates
{% for rc in release_candidates %}
#### {{ rc.name }}
- Version: {{ rc.version }}
- Status: {{ rc.status }}
- Required reviews: {{ rc.required_reviews }}
- Completed reviews: {{ rc.completed_reviews }}
- Failed checks: {{ rc.failed_checks | join(', ') }}
{% endfor %}

## Code Quality Metrics
- Total tests: {{ metrics.total_tests }}
- Test coverage: {{ metrics.coverage }}%
- Failed tests: {{ metrics.failed_tests }}
- Code style violations: {{ metrics.style_violations }}
- Security issues: {{ metrics.security_issues }}

## Pending Reviews
{% for review in pending_reviews %}
- PR #{{ review.number }}: {{ review.title }}
  - Author: {{ review.author }}
  - Age: {{ review.age }}
  - Required reviewers: {{ review.required_reviewers | join(', ') }}
{% endfor %}

## Failed Checks
{% for check in failed_checks %}
### {{ check.name }}
- Branch: {{ check.branch }}
- Error: {{ check.error }}
- Last attempt: {{ check.last_attempt }}
- Failing since: {{ check.failing_since }}
{% endfor %}

## Action Items
{% for item in action_items %}
- [ ] {{ item }}
{% endfor %}

## Notes
- This report is automatically generated every Monday at 00:00 UTC
- For issues or suggestions about this report, please create an issue with the tag 'report-feedback'

# OPSource Troubleshooting Guide

## Common Issues and Solutions

### 1. Service Connection Issues

#### Database Connection Failures

**Symptoms**:
- Service logs show database connection errors
- API endpoints return 500 errors
- Services fail to start

**Solutions**:

1. Check Database Status
```bash
# Check if database is running
pg_isready -h localhost -p 5432

# Check connection string
echo $DATABASE_URL

# Verify credentials
psql -h localhost -U username -d database -c "\conninfo"
```

2. Verify Network Connectivity
```bash
# Test network connection
telnet localhost 5432

# Check firewall rules
netstat -an | grep 5432
```

3. Review Logs
```bash
# View database logs
tail -f /var/log/postgresql/postgresql.log

# View application logs
kubectl logs ${POD_NAME}
```

#### Redis Connection Issues

**Symptoms**:
- Cache misses
- Slow response times
- Connection timeout errors

**Solutions**:

1. Check Redis Status
```bash
# Test Redis connection
redis-cli ping

# Check Redis info
redis-cli info

# Monitor Redis
redis-cli monitor
```

2. Clear Cache
```bash
# Clear specific key
redis-cli DEL key_name

# Clear all cache
redis-cli FLUSHALL
```

### 2. Performance Issues

#### High CPU Usage

**Symptoms**:
- Slow response times
- High system load
- Service throttling

**Solutions**:

1. Identify Resource Usage
```bash
# Check CPU usage
top -H

# Monitor process stats
pidstat 1

# View container metrics
docker stats
```

2. Profile Application
```bash
# Run profiler
perf record -F 99 -p ${PID} -g -- sleep 30

# Analyze results
perf report
```

#### Memory Leaks

**Symptoms**:
- Increasing memory usage
- OOM errors
- Service restarts

**Solutions**:

1. Monitor Memory
```bash
# Check memory usage
free -m

# View detailed memory stats
vmstat 1

# Monitor specific process
ps -o pid,user,%mem,command ax | sort -b -k3 -r
```

2. Analyze Heap
```bash
# Generate heap dump
jmap -dump:format=b,file=heap.bin ${PID}

# Analyze dump
jhat heap.bin
```

### 3. API Issues

#### Endpoint Failures

**Symptoms**:
- HTTP 4xx/5xx errors
- Timeout errors
- Invalid responses

**Solutions**:

1. Check API Status
```bash
# Test endpoint
curl -v http://localhost:8080/health

# Monitor API logs
tail -f /var/log/opsource/api.log
```

2. Verify Request/Response
```bash
# Use API debugging tools
curl -X POST http://localhost:8080/api/endpoint \
  -H "Content-Type: application/json" \
  -d '{"key": "value"}' \
  -v
```

#### Rate Limiting

**Symptoms**:
- HTTP 429 errors
- Throttled requests
- Inconsistent response times

**Solutions**:

1. Check Rate Limits
```bash
# View rate limit config
cat /etc/opsource/rate-limits.conf

# Monitor rate limit metrics
redis-cli --scan --pattern "ratelimit:*"
```

2. Adjust Limits
```bash
# Update rate limits
kubectl apply -f k8s/rate-limits.yaml

# Verify changes
kubectl get configmap rate-limits -o yaml
```

### 4. Authentication Issues

#### Token Validation Failures

**Symptoms**:
- Authentication errors
- Invalid token messages
- Session expiration

**Solutions**:

1. Check Token
```bash
# Decode JWT
echo ${TOKEN} | base64 -d

# Verify token signature
jwt verify ${TOKEN} ${PUBLIC_KEY}
```

2. Review Auth Config
```bash
# Check auth settings
cat /etc/opsource/auth.conf

# Test auth endpoint
curl -X POST http://localhost:8080/auth/token \
  -H "Content-Type: application/json" \
  -d '{"username": "test", "password": "test"}'
```

### 5. Deployment Issues

#### Failed Deployments

**Symptoms**:
- Deployment timeout
- Pod creation failures
- Service unavailability

**Solutions**:

1. Check Deployment Status
```bash
# View deployment status
kubectl get deployments

# Check pod status
kubectl get pods

# View deployment events
kubectl describe deployment ${DEPLOYMENT_NAME}
```

2. Review Logs
```bash
# View pod logs
kubectl logs ${POD_NAME}

# Check previous pod logs
kubectl logs ${POD_NAME} --previous
```

### 6. Data Synchronization Issues

#### Inconsistent Data

**Symptoms**:
- Data mismatches
- Sync failures
- Stale data

**Solutions**:

1. Check Sync Status
```bash
# View sync logs
tail -f /var/log/opsource/sync.log

# Check sync metrics
curl http://localhost:8080/metrics | grep sync
```

2. Force Resync
```bash
# Trigger manual sync
./scripts/force-sync.sh

# Verify sync completion
./scripts/verify-sync.sh
```

## Diagnostic Tools

### 1. Logging Tools

```bash
# Enable debug logging
sed -i 's/log_level=info/log_level=debug/' /etc/opsource/logging.conf

# Collect all logs
./scripts/collect-logs.sh

# Analyze log patterns
grep -r "ERROR" /var/log/opsource/
```

### 2. Monitoring Tools

```bash
# Check system metrics
prometheus-query 'rate(http_requests_total[5m])'

# View service health
curl http://localhost:8080/health

# Monitor resources
top -H
```

### 3. Debugging Tools

```bash
# Start debug session
dlv attach ${PID}

# Profile CPU usage
go tool pprof http://localhost:8080/debug/pprof/profile

# Trace requests
opentelemetry-cli trace
```

## Recovery Procedures

### 1. Service Recovery

```bash
# Restart service
systemctl restart opsource

# Verify service status
systemctl status opsource

# Check logs after restart
journalctl -u opsource -f
```

### 2. Data Recovery

```bash
# Restore from backup
./scripts/restore-backup.sh ${BACKUP_FILE}

# Verify data integrity
./scripts/verify-data.sh

# Sync after recovery
./scripts/sync-data.sh
```

## Prevention Measures

### 1. Monitoring Setup

```bash
# Set up alerts
./scripts/setup-alerts.sh

# Configure metrics
./scripts/configure-metrics.sh

# Test monitoring
./scripts/test-monitoring.sh
```

### 2. Backup Procedures

```bash
# Schedule backups
crontab -e
# Add: 0 0 * * * /scripts/backup.sh

# Verify backup success
./scripts/verify-backup.sh

# Test restoration
./scripts/test-restore.sh
```

## Support Escalation

### 1. Support Levels

1. **Level 1**: Basic troubleshooting
   - Log analysis
   - Configuration checks
   - Common fixes

2. **Level 2**: Advanced troubleshooting
   - Performance analysis
   - Security issues
   - Complex bugs

3. **Level 3**: Expert support
   - Architecture issues
   - Critical failures
   - Custom solutions

### 2. Contact Information

- **Emergency Support**: support-emergency@opsource.com
- **Technical Support**: support-tech@opsource.com
- **Security Team**: security@opsource.com

### 3. Escalation Process

1. Create support ticket
2. Collect relevant logs and data
3. Document reproduction steps
4. Follow up with support team
5. Track resolution progress

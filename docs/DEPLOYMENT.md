# OPSource Deployment Guide

## Deployment Environments

### 1. Development Environment

#### Requirements
- Docker Desktop
- Kubernetes (local)
- Helm
- kubectl

#### Setup

```bash
# Create development namespace
kubectl create namespace opsource-dev

# Deploy services
helm install opsource-dev ./helm/opsource -f values-dev.yaml
```

### 2. Staging Environment

#### Requirements
- Kubernetes cluster
- CI/CD pipeline
- Monitoring setup

#### Deployment

```bash
# Create staging namespace
kubectl create namespace opsource-staging

# Deploy services
helm install opsource-staging ./helm/opsource -f values-staging.yaml
```

### 3. Production Environment

#### Requirements
- Production Kubernetes cluster
- Load balancers
- Monitoring and alerting
- Backup systems

#### Deployment

```bash
# Create production namespace
kubectl create namespace opsource-prod

# Deploy services
helm install opsource-prod ./helm/opsource -f values-prod.yaml
```

## Deployment Process

### 1. Build Process

```bash
# Build Docker images
docker build -t opsource/anya:latest ./anya
docker build -t opsource/dash33:latest ./dash33
docker build -t opsource/enterprise:latest ./enterprise
docker build -t opsource/mobile:latest ./mobile

# Push images
docker push opsource/anya:latest
docker push opsource/dash33:latest
docker push opsource/enterprise:latest
docker push opsource/mobile:latest
```

### 2. Database Migration

```bash
# Run migrations
./scripts/migrate.sh --env production

# Verify migration
./scripts/verify-migration.sh
```

### 3. Service Deployment

```bash
# Deploy core services
kubectl apply -f k8s/core/

# Deploy supporting services
kubectl apply -f k8s/support/

# Verify deployment
kubectl get pods -n opsource-prod
```

## Configuration Management

### 1. Environment Variables

```yaml
# config/production.yaml
database:
  url: postgresql://user:pass@host:5432/db
  pool_size: 10

redis:
  url: redis://host:6379
  pool_size: 5

api:
  port: 8080
  workers: 4
```

### 2. Secrets Management

```bash
# Create secrets
kubectl create secret generic db-credentials \
    --from-literal=username=myuser \
    --from-literal=password=mypass

# Use secrets in deployment
kubectl apply -f k8s/secrets/
```

## Monitoring Setup

### 1. Prometheus Configuration

```yaml
# prometheus/config.yaml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'opsource'
    static_configs:
      - targets: ['localhost:9090']
```

### 2. Grafana Dashboards

```bash
# Import dashboards
kubectl apply -f monitoring/dashboards/

# Configure data sources
kubectl apply -f monitoring/datasources/
```

## Backup and Recovery

### 1. Database Backup

```bash
# Backup database
./scripts/backup-db.sh

# Verify backup
./scripts/verify-backup.sh
```

### 2. Configuration Backup

```bash
# Backup configurations
kubectl get configmap -n opsource-prod -o yaml > configs-backup.yaml

# Backup secrets
kubectl get secret -n opsource-prod -o yaml > secrets-backup.yaml
```

## Scaling Configuration

### 1. Horizontal Pod Autoscaling

```yaml
# k8s/hpa.yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: opsource-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: opsource
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
```

### 2. Vertical Pod Autoscaling

```yaml
# k8s/vpa.yaml
apiVersion: autoscaling.k8s.io/v1
kind: VerticalPodAutoscaler
metadata:
  name: opsource-vpa
spec:
  targetRef:
    apiVersion: "apps/v1"
    kind: Deployment
    name: opsource
  updatePolicy:
    updateMode: "Auto"
```

## Security Configuration

### 1. Network Policies

```yaml
# k8s/network-policy.yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: opsource-network-policy
spec:
  podSelector:
    matchLabels:
      app: opsource
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - podSelector:
        matchLabels:
          app: frontend
    ports:
    - protocol: TCP
      port: 8080
```

### 2. RBAC Configuration

```yaml
# k8s/rbac.yaml
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  namespace: opsource-prod
  name: opsource-role
rules:
- apiGroups: [""]
  resources: ["pods"]
  verbs: ["get", "list", "watch"]
```

## Troubleshooting

### 1. Common Issues

#### Database Connection Issues

```bash
# Check database connectivity
kubectl exec -it ${POD_NAME} -- pg_isready -h ${DB_HOST}

# View database logs
kubectl logs ${DB_POD_NAME}
```

#### Service Health Issues

```bash
# Check service health
kubectl describe pod ${POD_NAME}

# View service logs
kubectl logs ${POD_NAME}
```

### 2. Recovery Procedures

#### Service Recovery

```bash
# Restart service
kubectl rollout restart deployment ${DEPLOYMENT_NAME}

# Verify recovery
kubectl get pods -w
```

#### Data Recovery

```bash
# Restore database
./scripts/restore-db.sh ${BACKUP_FILE}

# Verify restoration
./scripts/verify-db.sh
```

## Maintenance Procedures

### 1. Updates and Patches

```bash
# Update services
kubectl set image deployment/${DEPLOYMENT_NAME} ${CONTAINER_NAME}=${NEW_IMAGE}

# Rollback if needed
kubectl rollout undo deployment/${DEPLOYMENT_NAME}
```

### 2. Health Checks

```bash
# Check system health
./scripts/health-check.sh

# Generate health report
./scripts/generate-report.sh
```

## Deployment Checklist

### Pre-deployment
- [ ] Review changes
- [ ] Run tests
- [ ] Update documentation
- [ ] Backup data
- [ ] Notify stakeholders

### Deployment
- [ ] Deploy database changes
- [ ] Deploy service updates
- [ ] Verify deployment
- [ ] Run smoke tests
- [ ] Monitor metrics

### Post-deployment
- [ ] Verify functionality
- [ ] Check logs
- [ ] Monitor performance
- [ ] Update status
- [ ] Document issues

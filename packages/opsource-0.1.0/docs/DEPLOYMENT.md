# OPSource Deployment Guide

## Prerequisites

### Required Tools

- Docker Desktop for Windows
- Windows Subsystem for Linux 2 (WSL2)
- kubectl (Kubernetes CLI)
- Helm v3
- PowerShell 7+

### Installation Steps

```powershell
# Install WSL2
wsl --install

# Install Chocolatey (Run as Administrator)
Set-ExecutionPolicy Bypass -Scope Process -Force
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))

# Install required tools
choco install kubernetes-cli
choco install helm
```

## Deployment Environments

### 1. Development Environment

#### Development Requirements

- Docker Desktop
- Kubernetes (local)
- Helm
- kubectl

#### Setup

```powershell
# Create development namespace
kubectl create namespace opsource-dev

# Deploy using Helm
helm install opsource-dev .\helm\opsource -f values-dev.yaml
```

### 2. Staging Environment

#### Staging Requirements

- Kubernetes cluster
- CI/CD pipeline
- Monitoring setup

#### Staging Deployment Steps

```powershell
# Create staging namespace
kubectl create namespace opsource-staging

# Deploy using Helm
helm install opsource-staging .\helm\opsource -f values-staging.yaml
```

### 3. Production Environment

#### Production Requirements

- Production Kubernetes cluster
- Load balancers
- Monitoring and alerting
- Backup systems

#### Deployment

```powershell
# Create production namespace
kubectl create namespace opsource-prod

# Deploy using Helm
helm install opsource-prod .\helm\opsource -f values-prod.yaml
```

## Build and Deploy Process

### 1. Build Docker Images

```powershell
# Build images
docker build -t opsource/anya:latest .\anya
docker build -t opsource/dash33:latest .\dash33
docker build -t opsource/enterprise:latest .\enterprise
docker build -t opsource/mobile:latest .\mobile

# Push to registry
docker push opsource/anya:latest
docker push opsource/dash33:latest
docker push opsource/enterprise:latest
docker push opsource/mobile:latest
```

### 2. Database Operations

```powershell
# Run migrations
.\scripts\migrate.ps1 -Environment production

# Verify migration
.\scripts\verify-migration.ps1
```

### 3. Service Deployment

```powershell
# Deploy core services
kubectl apply -f .\k8s\core\

# Deploy support services
kubectl apply -f .\k8s\support\

# Verify deployment
kubectl get pods -n opsource-prod
```

## Configuration Management

### Environment Variables

```yaml
# config/production.yaml
database:
  url: "Server=host;Database=db;User Id=user;Password=pass;"
  pool_size: 10

redis:
  url: "localhost:6379"
  pool_size: 5

api:
  port: 8080
  workers: 4
```

### Secrets Management

```powershell
# Create secrets
kubectl create secret generic db-credentials `
    --from-literal=username=myuser `
    --from-literal=password=mypass

# Apply secret configuration
kubectl apply -f .\k8s\secrets\
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

- [ ] Run `.\scripts\pre-deploy-check.ps1`
- [ ] Backup data using `.\scripts\backup.ps1`
- [ ] Run tests `npm test`
- [ ] Update documentation

### Deployment Tasks

- [ ] Deploy database changes
- [ ] Deploy service updates
- [ ] Verify deployment
- [ ] Run smoke tests
- [ ] Monitor metrics

### Post-deployment

- [ ] Run `.\scripts\verify-deployment.ps1`
- [ ] Check logs `kubectl logs -n opsource-prod`
- [ ] Monitor metrics
- [ ] Update status
- [ ] Document issues

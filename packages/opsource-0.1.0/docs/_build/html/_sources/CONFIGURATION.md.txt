# OPSource Configuration Guide

## Global Configuration

### Environment Variables

```bash
# Core Settings
OPSOURCE_ENV=development              # Environment (development/staging/production)
LOG_LEVEL=info                        # Logging level (debug/info/warn/error)
API_PORT=8080                         # API server port
DEBUG_MODE=false                      # Enable debug mode

# Security
SECRET_KEY=your-secret-key           # Main encryption key
JWT_SECRET=your-jwt-secret           # JWT signing key
MFA_ENABLED=true                     # Enable multi-factor authentication
SESSION_TIMEOUT=3600                 # Session timeout in seconds

# Database
DATABASE_URL=postgres://user:pass@host:5432/db
DB_POOL_SIZE=10                      # Database connection pool size
DB_TIMEOUT=30                        # Database timeout in seconds

# Cache
REDIS_URL=redis://localhost:6379
CACHE_TTL=3600                       # Cache time-to-live
CACHE_SIZE=1000                      # Maximum cache entries

# Blockchain
BTC_NETWORK=testnet                  # Bitcoin network (mainnet/testnet)
ETH_NETWORK=ropsten                  # Ethereum network
LIGHTNING_NODE=                      # Lightning node address
```

### Configuration Files

#### Main Configuration (config/config.toml)
```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4

[database]
url = "${DATABASE_URL}"
pool_size = 10
timeout = 30

[security]
secret_key = "${SECRET_KEY}"
mfa_enabled = true
session_timeout = 3600

[cache]
url = "${REDIS_URL}"
ttl = 3600
max_size = 1000
```

## Component Configuration

### Anya Configuration

#### AI Settings (anya/config/ai_config.toml)
```toml
[ml]
model_path = "models/"
batch_size = 32
learning_rate = 0.001
epochs = 100

[training]
data_path = "data/"
validation_split = 0.2
shuffle = true
```

### Dash33 Configuration

#### Trading Settings (dash33/config/trading_config.toml)
```toml
[trading]
max_positions = 10
risk_limit = 0.02
stop_loss = 0.05

[market]
default_pair = "BTC/USD"
tick_interval = "1m"
```

### Enterprise Configuration

#### Business Rules (enterprise/config/rules_config.toml)
```toml
[workflow]
max_steps = 20
timeout = 3600
retry_limit = 3

[rules]
engine = "native"
cache_rules = true
```

### Mobile Configuration

#### App Settings (mobile/config/app_config.toml)
```toml
[app]
version = "1.0.0"
update_check = true
offline_mode = false

[storage]
max_cache = "100MB"
cleanup_interval = "24h"
```

## Environment-Specific Configuration

### Development

```bash
# development.env
OPSOURCE_ENV=development
DEBUG_MODE=true
LOG_LEVEL=debug
```

### Staging

```bash
# staging.env
OPSOURCE_ENV=staging
DEBUG_MODE=false
LOG_LEVEL=info
```

### Production

```bash
# production.env
OPSOURCE_ENV=production
DEBUG_MODE=false
LOG_LEVEL=warn
```

## Security Configuration

### Authentication

```toml
[auth]
provider = "native"
mfa_required = true
password_policy = "strong"
session_timeout = "12h"

[oauth]
github_client_id = "${GITHUB_CLIENT_ID}"
github_client_secret = "${GITHUB_CLIENT_SECRET}"
```

### Authorization

```toml
[rbac]
default_role = "user"
super_admin = ["admin@example.com"]
role_hierarchy = true

[permissions]
strict_mode = true
cache_ttl = "1h"
```

## Logging Configuration

### Log Settings (config/log4rs.yaml)

```yaml
appenders:
  console:
    kind: console
    encoder:
      pattern: "{d} - {l} - {m}{n}"

  file:
    kind: file
    path: "log/opsource.log"
    encoder:
      pattern: "{d} - {l} - {t} - {m}{n}"

root:
  level: info
  appenders:
    - console
    - file

loggers:
  app::backend::db:
    level: info
  app::server:
    level: info
```

## Monitoring Configuration

### Metrics Settings

```toml
[metrics]
enabled = true
interval = "10s"
retention = "7d"

[tracing]
enabled = true
sampling_rate = 0.1
```

## Cache Configuration

### Redis Settings

```toml
[redis]
url = "${REDIS_URL}"
pool_size = 10
timeout = 30

[cache]
default_ttl = "1h"
max_entries = 10000
eviction = "lru"
```

## Database Configuration

### PostgreSQL Settings

```toml
[database]
url = "${DATABASE_URL}"
max_connections = 100
idle_timeout = "10m"
connection_timeout = "30s"

[migrations]
auto_migrate = true
version_table = "schema_version"
```

## Integration Configuration

### External Services

```toml
[services]
timeout = "30s"
retry_attempts = 3
circuit_breaker = true

[apis]
rate_limit = 100
quota_window = "1m"
```

## Deployment Configuration

### Docker Settings (docker-compose.yml)

```yaml
version: '3.8'
services:
  api:
    build: .
    env_file: .env
    ports:
      - "8080:8080"
    volumes:
      - ./config:/app/config
```

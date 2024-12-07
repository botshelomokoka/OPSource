# OPSource Quick Start Guide

## Prerequisites

- Rust 1.70+
- PostgreSQL 13+
- Redis 6+
- Node.js 18+
- Git

## Installation

1. **Clone Repository**

```bash
git clone https://github.com/botshelomokoka/opsource.git
cd opsource
```

2. **Set Up Environment**

```bash
# Copy environment template
cp .env.example .env

# Edit .env with your settings
nano .env

# Create Python virtual environment (optional)
python -m venv .venv
source .venv/bin/activate  # or .venv\Scripts\activate on Windows
```

3. **Install Dependencies**

```bash
# Install Rust dependencies
cargo build

# Install Python dependencies
pip install -r requirements.txt

# Install development dependencies (optional)
pip install -r requirements-dev.txt
```

## Quick Setup

### 1. Database Setup

```bash
# Initialize database
./scripts/setup_db.sh

# Run migrations
./scripts/migrate_db.sh
```

### 2. Start Services

```bash
# Start Redis
redis-server

# Start development server
cargo run
```

### 3. Verify Installation

```bash
# Run tests
cargo test

# Check API status
curl http://localhost:8080/health
```

## Basic Usage

### 1. Anya Integration

```rust
// Initialize Anya
let anya = Anya::new(config)?;

// Start AI processing
anya.start_processing().await?;

// Make predictions
let prediction = anya.predict(data).await?;
```

### 2. Dash33 Trading

```rust
// Setup trading environment
let trading = Trading::new(config)?;

// Connect to market
trading.connect_market("BTC/USD").await?;

// Place order
let order = Order::market_buy("BTC/USD", 1.0);
trading.place_order(order).await?;
```

### 3. Enterprise Features

```rust
// Initialize enterprise system
let enterprise = Enterprise::new(config)?;

// Start workflow
let workflow = Workflow::new("approval_process");
enterprise.start_workflow(workflow).await?;
```

### 4. Mobile Integration

```rust
// Setup mobile backend
let mobile = Mobile::new(config)?;

// Initialize wallet
mobile.init_wallet().await?;

// Process transaction
mobile.send_transaction(tx).await?;
```

## Common Tasks

### User Management

```rust
// Create user
let user = User::new("john@example.com");
auth.create_user(user).await?;

// Authenticate
let token = auth.login(credentials).await?;
```

### Data Processing

```rust
// Process market data
let processor = DataProcessor::new();
processor.process_batch(data).await?;

// Generate analytics
let analytics = Analytics::new();
analytics.generate_report().await?;
```

### Blockchain Operations

```rust
// Connect to blockchain
let node = Node::connect(network).await?;

// Submit transaction
node.submit_transaction(tx).await?;
```

## Development Workflow

### 1. Create Feature Branch

```bash
git checkout -b feature/new-feature
```

### 2. Make Changes

```bash
# Edit code
code .

# Format code
cargo fmt

# Check lints
cargo clippy
```

### 3. Test Changes

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test '*'
```

### 4. Submit Changes

```bash
# Commit changes
git add .
git commit -m "Add new feature"

# Push changes
git push origin feature/new-feature
```

## Troubleshooting

### Common Issues

1. **Database Connection**

```bash
# Check database status
pg_isready

# Verify connection string
echo $DATABASE_URL
```

2. **Redis Connection**

```bash
# Check Redis status
redis-cli ping

# Clear Redis cache
redis-cli flushall
```

3. **API Issues**

```bash
# Check logs
tail -f log/opsource.log

# Verify API status
curl http://localhost:8080/health
```

## Next Steps

1. **Read Documentation**
   - [API Reference](./API_REFERENCE.md)
   - [Architecture Guide](./ARCHITECTURE.md)
   - [Security Guide](./SECURITY.md)

2. **Explore Features**
   - Anya AI capabilities
   - Dash33 trading features
   - Enterprise workflows
   - Mobile functionality

3. **Join Community**
   - GitHub discussions
   - Developer forum
   - Discord channel
   - Mailing list

## Support

- Documentation: [docs/](./INDEX.md)
- Issues: GitHub Issues
- Chat: Discord
- Email: support@opsource.com

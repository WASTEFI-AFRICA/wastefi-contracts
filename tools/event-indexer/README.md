# WasteFi Event Indexer

A reference implementation of an event indexer for WasteFi smart contracts.

## Overview

This event indexer polls Soroban RPC for contract events, processes them, and stores them in a database for querying by applications.

## Features

- ✅ Real-time event polling from Soroban RPC
- ✅ Event deduplication
- ✅ Retry logic for failed events
- ✅ PostgreSQL storage with optimized indexes
- ✅ REST API for querying indexed events
- ✅ Prometheus metrics
- ✅ Docker support

## Architecture

```
┌─────────────────┐
│  Soroban RPC    │
│   (Stellar)     │
└────────┬────────┘
         │
         │ Poll events every 5s
         │
┌────────▼────────┐
│   Event Poller  │
│   (TypeScript)  │
└────────┬────────┘
         │
         │ Parse & validate
         │
┌────────▼────────┐
│Event Processor  │
│  (Handlers)     │
└────────┬────────┘
         │
         │ Store
         │
┌────────▼────────┐
│   PostgreSQL    │
│    Database     │
└────────┬────────┘
         │
         │ Query
         │
┌────────▼────────┐
│   REST API      │
│  (Express.js)   │
└─────────────────┘
```

## Directory Structure

```
event-indexer/
├── src/
│   ├── index.ts              # Main entry point
│   ├── poller.ts             # Event polling logic
│   ├── processor.ts          # Event processing
│   ├── handlers/             # Event type handlers
│   │   ├── transaction.ts
│   │   ├── payment.ts
│   │   ├── reputation.ts
│   │   └── collector.ts
│   ├── database/             # Database layer
│   │   ├── connection.ts
│   │   ├── schema.sql
│   │   └── queries.ts
│   ├── api/                  # REST API
│   │   ├── server.ts
│   │   └── routes/
│   └── utils/                # Utilities
│       ├── logger.ts
│       └── metrics.ts
├── config/
│   └── default.json          # Configuration
├── docker/
│   ├── Dockerfile
│   └── docker-compose.yml
├── package.json
├── tsconfig.json
└── README.md
```

## Installation

### Prerequisites

- Node.js 18+
- PostgreSQL 14+
- Docker (optional)

### Setup

```bash
# Install dependencies
npm install

# Configure environment
cp .env.example .env
# Edit .env with your settings

# Initialize database
npm run db:init

# Run migrations
npm run db:migrate

# Start indexer
npm start
```

### Docker Setup

```bash
# Build and start all services
docker-compose up -d

# View logs
docker-compose logs -f indexer

# Stop services
docker-compose down
```

## Configuration

Edit `.env` file:

```env
# Soroban RPC
RPC_URL=https://soroban-testnet.stellar.org
NETWORK_PASSPHRASE=Test SDF Network ; September 2015

# Contracts
WASTE_TOKEN_CONTRACT=CA7Q...
COLLECTOR_REGISTRY_CONTRACT=CA8R...
COLLECTION_POINT_CONTRACT=CA9S...
WASTE_TRANSACTION_CONTRACT=CA10...
PAYMENT_DISTRIBUTION_CONTRACT=CA11...
REPUTATION_CONTRACT=CA12...
MATERIAL_PRICING_CONTRACT=CA13...

# Database
DATABASE_URL=postgresql://user:password@localhost:5432/wastefi_events

# Indexing
POLL_INTERVAL=5000              # milliseconds
BATCH_SIZE=100                  # events per batch
START_LEDGER=auto               # or specific ledger number

# API
API_PORT=3000
API_HOST=0.0.0.0

# Monitoring
METRICS_PORT=9090
LOG_LEVEL=info                  # debug, info, warn, error
```

## Usage

### Starting the Indexer

```bash
# Development mode with auto-reload
npm run dev

# Production mode
npm run build
npm start

# With PM2
pm2 start ecosystem.config.js
```

### API Endpoints

#### Get Events
```http
GET /api/events?contract=CA7Q...&type=tx_rec&limit=100
```

#### Get Collector Transactions
```http
GET /api/collectors/GA5X.../transactions
```

#### Get Material Statistics
```http
GET /api/statistics/materials/Plastic
```

#### Get Recent Events
```http
GET /api/events/recent?limit=50
```

#### Health Check
```http
GET /api/health
```

### Example Queries

```typescript
// Get all transaction events for a collector
const response = await fetch(
  'http://localhost:3000/api/collectors/GA5X.../transactions?limit=50'
);
const transactions = await response.json();

// Get payment events
const payments = await fetch(
  'http://localhost:3000/api/events?type=pay_new&limit=100'
);

// Get reputation changes
const reputationChanges = await fetch(
  'http://localhost:3000/api/events?type=rep_upd&collector=GA5X...'
);
```

## Database Schema

```sql
-- Events table
CREATE TABLE events (
  id BIGSERIAL PRIMARY KEY,
  contract_id VARCHAR(56) NOT NULL,
  event_type VARCHAR(32) NOT NULL,
  ledger_sequence BIGINT NOT NULL,
  transaction_hash VARCHAR(64) NOT NULL,
  event_index INTEGER NOT NULL,
  timestamp TIMESTAMP NOT NULL,
  data JSONB NOT NULL,
  processed BOOLEAN DEFAULT FALSE,
  indexed_at TIMESTAMP DEFAULT NOW(),
  UNIQUE(ledger_sequence, transaction_hash, event_index)
);

-- Indexes for fast queries
CREATE INDEX idx_contract_event ON events(contract_id, event_type);
CREATE INDEX idx_ledger ON events(ledger_sequence);
CREATE INDEX idx_timestamp ON events(timestamp DESC);
CREATE INDEX idx_data_gin ON events USING gin(data);
CREATE INDEX idx_collector ON events((data->>'collector')) WHERE data ? 'collector';

-- Indexer state
CREATE TABLE indexer_state (
  id INTEGER PRIMARY KEY DEFAULT 1,
  last_ledger BIGINT NOT NULL,
  last_updated TIMESTAMP DEFAULT NOW(),
  CHECK (id = 1)
);

-- Failed events for retry
CREATE TABLE failed_events (
  id BIGSERIAL PRIMARY KEY,
  event_data JSONB NOT NULL,
  error TEXT,
  retry_count INTEGER DEFAULT 0,
  last_retry TIMESTAMP,
  created_at TIMESTAMP DEFAULT NOW()
);
```

## Monitoring

### Metrics

Prometheus metrics available at `:9090/metrics`:

- `indexer_events_processed_total` - Total events processed
- `indexer_events_per_second` - Events processing rate
- `indexer_lag_seconds` - Indexing lag behind current ledger
- `indexer_errors_total` - Total errors encountered
- `indexer_last_ledger` - Last indexed ledger

### Grafana Dashboard

Import the provided dashboard from `grafana/dashboard.json` for visualization.

### Logging

Logs are structured JSON for easy parsing:

```json
{
  "level": "info",
  "timestamp": "2024-01-15T10:30:00Z",
  "message": "Processed event",
  "event_type": "tx_rec",
  "transaction_id": 42,
  "ledger": 123456
}
```

## Development

### Running Tests

```bash
# Unit tests
npm test

# Integration tests
npm run test:integration

# E2E tests
npm run test:e2e

# Coverage
npm run test:coverage
```

### Code Quality

```bash
# Lint
npm run lint

# Format
npm run format

# Type check
npm run type-check
```

## Deployment

### Production Checklist

- [ ] Set strong database passwords
- [ ] Configure rate limiting
- [ ] Set up monitoring and alerts
- [ ] Configure log aggregation
- [ ] Set up database backups
- [ ] Use connection pooling
- [ ] Configure proper CPU/memory limits
- [ ] Set up health check endpoints
- [ ] Use TLS for API endpoints
- [ ] Implement authentication if needed

### Docker Deployment

```bash
# Build production image
docker build -t wastefi-indexer:latest .

# Run with docker-compose
docker-compose -f docker-compose.prod.yml up -d

# Scale workers
docker-compose -f docker-compose.prod.yml up -d --scale worker=3
```

### Kubernetes Deployment

See `kubernetes/` directory for manifests.

## Troubleshooting

### Indexer is Lagging

```bash
# Check current lag
curl http://localhost:9090/metrics | grep indexer_lag

# Increase batch size in config
BATCH_SIZE=500

# Add more worker processes
docker-compose up -d --scale worker=3
```

### Database Performance

```bash
# Check slow queries
SELECT * FROM pg_stat_statements ORDER BY total_time DESC LIMIT 10;

# Rebuild indexes
REINDEX TABLE events;

# Vacuum
VACUUM ANALYZE events;
```

### Missing Events

```bash
# Check failed events
SELECT * FROM failed_events ORDER BY created_at DESC LIMIT 10;

# Retry failed events
npm run retry-failed

# Backfill specific ledger range
npm run backfill -- --start 123456 --end 123500
```

## Performance

### Benchmarks

- **Throughput**: ~1000 events/second
- **Latency**: <100ms average processing time
- **Storage**: ~1KB per event

### Optimization Tips

1. **Batch Processing**: Process events in batches
2. **Parallel Workers**: Run multiple worker processes
3. **Database Tuning**: Optimize PostgreSQL settings
4. **Connection Pooling**: Use pgbouncer or similar
5. **Caching**: Cache frequently accessed data

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

## License

MIT License - See [LICENSE](../../LICENSE)

## Support

- GitHub Issues: [repository-url]/issues
- Discord: [discord-invite]
- Email: support@wastefi.io

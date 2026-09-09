# WasteFi Event Indexing Guide

## Overview

WasteFi smart contracts emit events for all significant state changes. This guide explains how to index these events for building real-time dashboards, notifications, analytics, and event-driven applications.

## Event Categories

### 1. Token Events
**Source**: `WasteToken` contract
**Namespace**: `TokenEvents`

| Event | Symbol | Data | Emitted When |
|-------|--------|------|--------------|
| Mint | `mint` | (to: Address, amount: i128) | Tokens are minted to a collector |
| Burn | `burn` | (from: Address, amount: i128) | Tokens are burned |
| Transfer | `transfer` | (from: Address, to: Address, amount: i128) | Tokens are transferred |

**Use Cases**:
- Track token supply changes
- Monitor collector reward distributions
- Build token transaction history
- Calculate token velocity metrics

### 2. Collector Events
**Source**: `CollectorRegistry` contract
**Namespace**: `CollectorEvents`

| Event | Symbol | Data | Emitted When |
|-------|--------|------|--------------|
| Registered | `reg` | (collector: Address, name: String) | New collector registers |
| StatusUpdated | `status` | (collector: Address, status: CollectorStatus) | Collector status changes |

**Use Cases**:
- Track new collector onboarding
- Monitor status changes (Pending → Active → Suspended)
- Build collector activity timelines
- Generate onboarding reports

### 3. Collection Point Events
**Source**: `CollectionPoint` contract
**Namespace**: `CollectionPointEvents`

| Event | Symbol | Data | Emitted When |
|-------|--------|------|--------------|
| Registered | `pt_reg` | (point_id: u64, owner: Address) | New collection point registered |
| Verified | `pt_ver` | (point_id: u64) | Collection point verified by admin |

**Use Cases**:
- Track collection point network growth
- Monitor verification workflow
- Build geographical coverage maps
- Analyze point distribution

### 4. Transaction Events
**Source**: `WasteTransaction` contract
**Namespace**: `TransactionEvents`

| Event | Symbol | Data | Emitted When |
|-------|--------|------|--------------|
| Recorded | `tx_rec` | (tx_id: u64, collector: Address, material: MaterialType, weight: u64) | New waste collection recorded |
| Verified | `tx_ver` | (tx_id: u64) | Transaction verified by admin |
| StatusChanged | `tx_stat` | (tx_id: u64, status: TransactionStatus) | Transaction status updated |

**Use Cases**:
- Real-time waste collection monitoring
- Material flow tracking
- Verification queue management
- Transaction lifecycle analytics

### 5. Payment Events
**Source**: `PaymentDistribution` contract
**Namespace**: `PaymentEvents`

| Event | Symbol | Data | Emitted When |
|-------|--------|------|--------------|
| Created | `pay_new` | (payment_id: u64, recipient: Address, amount: i128) | New payment created |
| Processed | `pay_proc` | (payment_id: u64) | Payment processed successfully |
| Failed | `pay_fail` | (payment_id: u64) | Payment failed |

**Use Cases**:
- Payment flow monitoring
- Financial reconciliation
- Failed payment alerts
- Collector payment tracking

### 6. Reputation Events
**Source**: `Reputation` contract
**Namespace**: `ReputationEvents`

| Event | Symbol | Data | Emitted When |
|-------|--------|------|--------------|
| ScoreUpdated | `rep_upd` | (collector: Address, old_score: u32, new_score: u32) | Reputation score changes |

**Use Cases**:
- Track reputation trends over time
- Detect rapid score changes
- Build reputation history charts
- Leaderboard updates

### 7. Pricing Events
**Source**: `MaterialPricing` contract
**Namespace**: `PricingEvents`

| Event | Symbol | Data | Emitted When |
|-------|--------|------|--------------|
| PriceUpdated | `prc_upd` | (material_type: MaterialType, price: i128) | Material price updated |

**Use Cases**:
- Price history tracking
- Market analysis
- Pricing strategy evaluation
- Cost calculations

### 8. Admin Events
**Source**: All contracts
**Namespace**: `AdminEvents`

| Event | Symbol | Data | Emitted When |
|-------|--------|------|--------------|
| AdminChanged | `adm_chg` | (old_admin: Address, new_admin: Address) | Admin ownership transferred |
| Paused | `paused` | () | Contract paused |
| Unpaused | `unpaused` | () | Contract unpaused |

**Use Cases**:
- Security monitoring
- Admin action audit trail
- Emergency response tracking
- Governance transparency

## Event Indexing Architectures

### Architecture 1: Soroban RPC Event Polling

```
┌─────────────┐
│ Soroban RPC │
│   Server    │
└──────┬──────┘
       │
       │ Poll every N seconds
       │
┌──────▼──────┐
│   Indexer   │ ← getEvents()
│   Service   │
└──────┬──────┘
       │
       │ Store events
       │
┌──────▼──────┐
│  Database   │
│ (PostgreSQL)│
└──────┬──────┘
       │
       │ Query indexed data
       │
┌──────▼──────┐
│ Application │
│   Backend   │
└─────────────┘
```

**Components**:
1. **Indexer Service**: Polls Soroban RPC for new events
2. **Database**: Stores parsed events with metadata
3. **Application Backend**: Queries indexed data

**Tools**:
- [Stellar Horizon](https://developers.stellar.org/docs/data/horizon)
- Custom Node.js/Python indexer
- PostgreSQL or MongoDB

### Architecture 2: Event Stream Processing

```
┌─────────────┐
│   Soroban   │
│  Contracts  │
└──────┬──────┘
       │
       │ Emit events
       │
┌──────▼──────┐
│   Stellar   │
│   Network   │
└──────┬──────┘
       │
       │ Subscribe to events
       │
┌──────▼──────┐
│   Apache    │
│    Kafka    │
└──────┬──────┘
       │
       ├─────────────┬─────────────┬─────────────┐
       │             │             │             │
┌──────▼──────┐ ┌───▼────┐ ┌─────▼────┐ ┌──────▼──────┐
│  Analytics  │ │ Alerts │ │Dashboard │ │  Webhooks   │
│   Engine    │ │Service │ │ Backend  │ │   Service   │
└─────────────┘ └────────┘ └──────────┘ └─────────────┘
```

**Components**:
1. **Event Stream**: Kafka or similar message queue
2. **Multiple Consumers**: Each handles specific use case
3. **Real-time Processing**: Stream processing frameworks

**Tools**:
- Apache Kafka
- Apache Flink
- Redis Streams

### Architecture 3: Serverless Event Handlers

```
┌─────────────┐
│   Soroban   │
│   Events    │
└──────┬──────┘
       │
       │ Webhook/Stream
       │
┌──────▼──────────┐
│ Cloud Functions │
│  (AWS Lambda)   │
└──────┬──────────┘
       │
       ├─────────────┬─────────────┬─────────────┐
       │             │             │             │
┌──────▼──────┐ ┌───▼────┐ ┌─────▼────┐ ┌──────▼──────┐
│   DynamoDB  │ │  SNS   │ │   SQS    │ │    S3       │
│   Storage   │ │Notifs  │ │  Queue   │ │  Archive    │
└─────────────┘ └────────┘ └──────────┘ └─────────────┘
```

**Components**:
1. **Serverless Functions**: Handle each event type
2. **Managed Services**: AWS/GCP/Azure services
3. **Auto-scaling**: Handle variable load

**Tools**:
- AWS Lambda + DynamoDB
- Google Cloud Functions + Firestore
- Azure Functions + CosmosDB

## Indexing Strategies

### Strategy 1: Real-time Indexing

**Best for**: Live dashboards, notifications, monitoring

```typescript
// Pseudo-code for real-time indexer
async function indexEvents() {
  let lastLedger = await getLastIndexedLedger();
  
  while (true) {
    const newEvents = await rpc.getEvents({
      startLedger: lastLedger + 1,
      contractIds: WASTEFI_CONTRACTS
    });
    
    for (const event of newEvents) {
      await processEvent(event);
      await db.saveEvent(event);
    }
    
    lastLedger = newEvents.lastLedger;
    await sleep(POLL_INTERVAL);
  }
}
```

**Characteristics**:
- Low latency (seconds)
- Continuous polling
- Higher resource usage
- Suitable for critical operations

### Strategy 2: Batch Indexing

**Best for**: Analytics, reports, historical data

```typescript
// Pseudo-code for batch indexer
async function batchIndexEvents() {
  const startLedger = await getLastIndexedLedger();
  const endLedger = await getCurrentLedger();
  
  const BATCH_SIZE = 1000;
  
  for (let i = startLedger; i < endLedger; i += BATCH_SIZE) {
    const events = await rpc.getEvents({
      startLedger: i,
      endLedger: Math.min(i + BATCH_SIZE, endLedger),
      contractIds: WASTEFI_CONTRACTS
    });
    
    await db.bulkInsert(events);
    await updateLastIndexedLedger(i + BATCH_SIZE);
  }
}
```

**Characteristics**:
- Higher latency (minutes/hours)
- Efficient resource usage
- Bulk processing
- Suitable for reporting

### Strategy 3: Hybrid Indexing

**Best for**: Complete systems with multiple needs

```typescript
// Real-time for critical events
indexRealtime({
  events: ['tx_rec', 'pay_new', 'pay_fail'],
  callback: async (event) => {
    await updateDashboard(event);
    await sendNotification(event);
  }
});

// Batch for analytics
scheduleBatchJob({
  interval: '1 hour',
  callback: async () => {
    await indexHistoricalData();
    await updateAnalytics();
  }
});
```

**Characteristics**:
- Best of both approaches
- Optimized resource usage
- Flexible processing
- Production-ready

## Database Schema Examples

### PostgreSQL Schema

```sql
-- Events table
CREATE TABLE events (
  id BIGSERIAL PRIMARY KEY,
  contract_id VARCHAR(56) NOT NULL,
  event_type VARCHAR(32) NOT NULL,
  ledger_sequence BIGINT NOT NULL,
  transaction_hash VARCHAR(64) NOT NULL,
  timestamp TIMESTAMP NOT NULL,
  data JSONB NOT NULL,
  indexed_at TIMESTAMP DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_contract_event ON events(contract_id, event_type);
CREATE INDEX idx_ledger ON events(ledger_sequence);
CREATE INDEX idx_timestamp ON events(timestamp);
CREATE INDEX idx_data_gin ON events USING gin(data);

-- Transaction events view
CREATE VIEW transaction_events AS
SELECT 
  id,
  ledger_sequence,
  timestamp,
  data->>'transaction_id' as tx_id,
  data->>'collector' as collector,
  data->>'material_type' as material,
  data->>'weight' as weight
FROM events
WHERE event_type = 'tx_rec';

-- Collector statistics materialized view
CREATE MATERIALIZED VIEW collector_stats AS
SELECT 
  data->>'collector' as collector,
  COUNT(*) as total_transactions,
  SUM((data->>'weight')::numeric) as total_weight,
  MIN(timestamp) as first_transaction,
  MAX(timestamp) as last_transaction
FROM transaction_events
GROUP BY collector;

CREATE UNIQUE INDEX idx_collector_stats ON collector_stats(collector);
```

### MongoDB Schema

```javascript
// Events collection
db.events.createIndex({ contract_id: 1, event_type: 1 });
db.events.createIndex({ ledger_sequence: 1 });
db.events.createIndex({ timestamp: -1 });
db.events.createIndex({ "data.collector": 1 });

// Example document
{
  _id: ObjectId("..."),
  contract_id: "CA7Q...",
  event_type: "tx_rec",
  ledger_sequence: 123456,
  transaction_hash: "abc123...",
  timestamp: ISODate("2024-01-15T10:30:00Z"),
  data: {
    transaction_id: 42,
    collector: "GA5X...",
    material_type: "Plastic",
    weight: 5000
  },
  indexed_at: ISODate("2024-01-15T10:30:05Z")
}

// Aggregation for collector stats
db.events.aggregate([
  { $match: { event_type: "tx_rec" } },
  { $group: {
      _id: "$data.collector",
      total_transactions: { $sum: 1 },
      total_weight: { $sum: "$data.weight" },
      first_transaction: { $min: "$timestamp" },
      last_transaction: { $max: "$timestamp" }
    }
  }
]);
```

## Event Processing Examples

### Example 1: Transaction Monitoring

```typescript
interface TransactionEvent {
  transaction_id: number;
  collector: string;
  material_type: string;
  weight: number;
  timestamp: Date;
}

async function processTransactionEvent(event: TransactionEvent) {
  // Store in database
  await db.transactions.insert(event);
  
  // Update real-time dashboard
  await dashboardService.updateTransactionCount();
  
  // Check for milestones
  const total = await db.transactions.count({
    collector: event.collector
  });
  
  if (total === 10 || total === 50 || total === 100) {
    await notificationService.sendMilestone(
      event.collector,
      `Congratulations on ${total} collections!`
    );
  }
  
  // Update leaderboard
  await leaderboardService.recalculate();
}
```

### Example 2: Payment Alerts

```typescript
async function processPaymentEvent(event: PaymentEvent) {
  if (event.event_type === 'pay_new') {
    // Payment created
    await notificationService.send(
      event.recipient,
      `Payment of ${formatXLM(event.amount)} initiated`
    );
  } else if (event.event_type === 'pay_fail') {
    // Payment failed - urgent alert
    await alertService.urgentAlert(
      event.recipient,
      `Payment ${event.payment_id} failed`
    );
    
    // Notify admin
    await adminNotification(
      `Payment failure for ${event.recipient}`
    );
  }
}
```

### Example 3: Reputation Tracking

```typescript
async function processReputationEvent(event: ReputationEvent) {
  const { collector, old_score, new_score } = event.data;
  
  // Store history
  await db.reputationHistory.insert({
    collector,
    old_score,
    new_score,
    change: new_score - old_score,
    timestamp: event.timestamp
  });
  
  // Check for tier changes
  const oldTier = getReputationTier(old_score);
  const newTier = getReputationTier(new_score);
  
  if (newTier > oldTier) {
    // Tier upgrade!
    await notificationService.sendTierUpgrade(
      collector,
      getTierName(newTier)
    );
    
    // Update collector badge
    await profileService.updateTier(collector, newTier);
  }
  
  // Alert if score drops significantly
  if (new_score < old_score - 50) {
    await alertService.send(
      collector,
      `Your reputation score has decreased. Review your recent transactions.`
    );
  }
}
```

## Best Practices

### 1. Event Deduplication
```typescript
async function processEvent(event: Event) {
  const eventId = `${event.ledger}_${event.transaction_hash}_${event.index}`;
  
  // Check if already processed
  if (await db.processedEvents.exists(eventId)) {
    console.log(`Event ${eventId} already processed, skipping`);
    return;
  }
  
  // Process event
  await handleEvent(event);
  
  // Mark as processed
  await db.processedEvents.insert(eventId);
}
```

### 2. Error Handling
```typescript
async function safeProcessEvent(event: Event) {
  try {
    await processEvent(event);
  } catch (error) {
    console.error(`Failed to process event:`, error);
    
    // Store failed event for retry
    await db.failedEvents.insert({
      event,
      error: error.message,
      retry_count: 0,
      created_at: new Date()
    });
  }
}
```

### 3. Backfill Strategy
```typescript
async function backfillEvents(startLedger: number, endLedger: number) {
  const BATCH_SIZE = 100;
  
  for (let i = startLedger; i <= endLedger; i += BATCH_SIZE) {
    console.log(`Backfilling ledgers ${i} to ${i + BATCH_SIZE}`);
    
    const events = await rpc.getEvents({
      startLedger: i,
      endLedger: Math.min(i + BATCH_SIZE, endLedger)
    });
    
    for (const event of events) {
      await processEvent(event);
    }
    
    // Rate limiting
    await sleep(1000);
  }
}
```

### 4. Monitoring & Metrics
```typescript
interface IndexerMetrics {
  eventsProcessed: number;
  eventsPerSecond: number;
  lastLedgerIndexed: number;
  indexingLag: number; // seconds behind current ledger
  errorRate: number;
}

async function trackMetrics() {
  const currentLedger = await rpc.getLatestLedger();
  const lastIndexed = await db.getLastIndexedLedger();
  
  const metrics: IndexerMetrics = {
    eventsProcessed: await db.events.count(),
    eventsPerSecond: await calculateEventsPerSecond(),
    lastLedgerIndexed: lastIndexed,
    indexingLag: (currentLedger - lastIndexed) * 5, // ~5 seconds per ledger
    errorRate: await calculateErrorRate()
  };
  
  await metricsService.record(metrics);
  
  // Alert if lag is too high
  if (metrics.indexingLag > 300) {
    await alertService.send('Indexer is lagging behind by 5+ minutes');
  }
}
```

## Tools & Libraries

### Stellar SDKs
- **JavaScript**: `@stellar/stellar-sdk`
- **Python**: `stellar-sdk`
- **Go**: `github.com/stellar/go`
- **Rust**: `stellar-base`

### Indexing Frameworks
- **The Graph Protocol**: Decentralized indexing
- **Moralis**: Blockchain data APIs
- **Covalent**: Unified blockchain API
- **Custom**: Build your own with Stellar SDK

### Databases
- **PostgreSQL**: Relational, JSONB support
- **MongoDB**: Document store, flexible schema
- **TimescaleDB**: Time-series optimization
- **Elasticsearch**: Full-text search, analytics

### Monitoring
- **Prometheus**: Metrics collection
- **Grafana**: Dashboards and visualization
- **Datadog**: Full-stack monitoring
- **Sentry**: Error tracking

## Reference Implementation

See `tools/event-indexer/` directory for a complete reference implementation of an event indexer using Node.js and PostgreSQL.

## Additional Resources

- [Soroban Events Documentation](https://soroban.stellar.org/docs/fundamentals-and-concepts/events)
- [Stellar Horizon API](https://developers.stellar.org/api/horizon)
- [WasteFi Contract Events Reference](./API.md#events)
- [Event Indexer Example](../tools/event-indexer/README.md)

## Support

For questions about event indexing:
- GitHub Discussions: [repository-url]/discussions
- Discord: [discord-invite]
- Documentation: [docs-url]

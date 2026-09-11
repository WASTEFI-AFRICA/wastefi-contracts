# WasteFi Operations Runbook

## Document Purpose

This runbook provides standard operating procedures for the daily operations and maintenance of the WasteFi platform. It covers monitoring, routine tasks, system maintenance, and references to emergency procedures.

**Target Audience**: Operations team, DevOps engineers, on-call engineers  
**Update Frequency**: Monthly or as procedures change  
**Version**: 1.0.0  
**Last Updated**: September 11, 2026

---

## Table of Contents

1. [Daily Operations](#1-daily-operations)
2. [Monitoring and Alerting](#2-monitoring-and-alerting)
3. [Common Operational Tasks](#3-common-operational-tasks)
4. [System Maintenance](#4-system-maintenance)
5. [Performance Optimization](#5-performance-optimization)
6. [Data Management](#6-data-management)
7. [User Support](#7-user-support)
8. [Reporting and Metrics](#8-reporting-and-metrics)
9. [On-Call Procedures](#9-on-call-procedures)
10. [Emergency Procedures Reference](#10-emergency-procedures-reference)

---

## 1. Daily Operations

### 1.1 Daily Operations Checklist

**Performed By**: On-call engineer  
**Frequency**: Every business day (Monday-Friday)  
**Duration**: 15-30 minutes

#### Morning Checks (9:00 AM local time)

- [ ] **Review overnight alerts**
  ```bash
  # Check alert history
  curl -X GET "https://metrics.wastefi.io/api/alerts?since=24h" | jq .
  ```

- [ ] **Check system health**
  ```bash
  # Load contract addresses
  source scripts/load_addresses.sh mainnet
  
  # Check all contract versions
  for contract_id in $WASTE_TOKEN $COLLECTOR_REGISTRY $COLLECTION_POINT \
                     $MATERIAL_PRICING $REPUTATION $WASTE_TRANSACTION \
                     $PAYMENT_DISTRIBUTION; do
    echo "Checking $contract_id..."
    soroban contract invoke --id $contract_id --network mainnet -- version
  done
  ```

- [ ] **Review transaction volume**
  - Total transactions in last 24h
  - Peak transaction rate
  - Failed transactions (should be <1%)

- [ ] **Check fraud detection**
  - Number of high-risk users flagged
  - Risk score distribution
  - Suspicious patterns

- [ ] **Verify price updates**
  ```bash
  # Check last price update time for each material
  soroban contract invoke \
    --id $MATERIAL_PRICING \
    --network mainnet \
    -- \
    get_last_update \
    --material "plastic"
  ```
  - All materials should be updated within 24h

- [ ] **Review payment distribution**
  - Total payments processed
  - Average payment amount
  - Failed payments (investigate if >0)

- [ ] **Check reputation scores**
  - Average reputation score (should be ~500-600)
  - Number of users with score <300 (review)
  - Reputation score trends

- [ ] **Monitor rate limits**
  - Rate limit hit rate
  - Blocked requests count
  - Suspicious activity patterns

- [ ] **Review error logs**
  ```bash
  # Check for errors in last 24h
  grep -i "error" /var/log/wastefi/*.log | tail -50
  ```

#### Afternoon Checks (3:00 PM local time)

- [ ] **Verify backup completion**
  - Contract address backups
  - Configuration backups
  - Monitoring data backups

- [ ] **Check resource usage**
  - XLM balance for admin account (should be >50 XLM)
  - Storage usage per contract
  - Network bandwidth

- [ ] **Review user activity**
  - New registrations today
  - Active collectors
  - Active collection points

- [ ] **Update status dashboard**
  - All systems operational (green)
  - Any degraded services (yellow)
  - Any outages (red)

---

### 1.2 Weekly Operations Checklist

**Performed By**: Operations lead  
**Frequency**: Every Monday  
**Duration**: 1-2 hours

- [ ] **Review weekly metrics**
  - Total transactions (week over week)
  - Revenue/rewards distributed
  - User growth rate
  - System uptime percentage

- [ ] **Security review**
  - Review all P2+ incidents from past week
  - Check for new CVEs affecting dependencies
  - Review admin activity logs
  - Verify multi-sig operations (if applicable)

- [ ] **Performance analysis**
  - Average transaction time
  - Contract execution gas usage
  - Query response times
  - Storage growth rate

- [ ] **Update material prices** (if needed)
  ```bash
  # Review market prices and update if needed
  ./scripts/update_prices.sh mainnet
  ```

- [ ] **Fraud pattern review**
  - New fraud patterns detected
  - False positive rate
  - Adjustments to risk thresholds needed

- [ ] **User support review**
  - Open support tickets
  - Common issues
  - Documentation updates needed

- [ ] **Team sync meeting**
  - Review last week's incidents
  - Discuss upcoming changes
  - Address operational concerns

---

### 1.3 Monthly Operations Checklist

**Performed By**: Operations team + Security lead  
**Frequency**: First Monday of each month  
**Duration**: Half day

- [ ] **Monthly security audit**
  - Review all security logs
  - Access control audit (verify all admins)
  - Key rotation check (rotate if >90 days)
  - Vulnerability scan

- [ ] **Performance optimization**
  - Identify bottlenecks
  - Gas usage optimization opportunities
  - Storage cleanup if needed

- [ ] **Capacity planning**
  - Project user growth for next 3 months
  - XLM balance forecasting
  - Storage capacity planning

- [ ] **Disaster recovery test**
  - Test backup restoration
  - Verify rollback procedures
  - Update emergency contacts

- [ ] **Documentation review**
  - Update runbooks with lessons learned
  - Review and update OPERATIONS.md (this document)
  - Update API documentation if changes made

- [ ] **Contract health check**
  - Review contract state consistency
  - Check for data anomalies
  - Verify cross-contract references

- [ ] **Generate monthly report**
  - User metrics
  - Financial metrics
  - Security incidents summary
  - Uptime and SLA compliance

---

## 2. Monitoring and Alerting

### 2.1 Key Metrics to Monitor

#### System Health Metrics

| Metric | Normal Range | Warning Threshold | Critical Threshold | Alert |
|--------|--------------|-------------------|-------------------|-------|
| **Contract Uptime** | 100% | <99.9% | <99.5% | Yes |
| **Transaction Success Rate** | >99% | <99% | <95% | Yes |
| **Average Transaction Time** | <2s | >3s | >5s | Yes |
| **Admin Account Balance** | >50 XLM | <50 XLM | <20 XLM | Yes |

#### Transaction Metrics

| Metric | Normal Range | Warning Threshold | Critical Threshold | Alert |
|--------|--------------|-------------------|-------------------|-------|
| **Transactions per Hour** | 10-1000 | >1500 | >2000 | Yes |
| **Failed Transactions %** | <1% | >2% | >5% | Yes |
| **Duplicate Attempts** | <5/hour | >10/hour | >20/hour | Yes |
| **Payment Processing Time** | <5s | >10s | >30s | Yes |

#### Security Metrics

| Metric | Normal Range | Warning Threshold | Critical Threshold | Alert |
|--------|--------------|-------------------|-------------------|-------|
| **High-Risk Users (>800 score)** | 0-5/day | >10/day | >20/day | Yes |
| **Rate Limit Hits** | <100/hour | >200/hour | >500/hour | Yes |
| **Emergency Level** | Normal | Warning | Critical/Shutdown | Yes |
| **Admin Actions** | <10/day | >20/day | >50/day | Yes |

#### Business Metrics

| Metric | Track Daily | Report Weekly | Report Monthly |
|--------|-------------|---------------|----------------|
| **New Collectors** | Yes | Yes | Yes |
| **Active Collectors** | Yes | Yes | Yes |
| **Total Waste Collected (kg)** | Yes | Yes | Yes |
| **Rewards Distributed (tokens)** | Yes | Yes | Yes |
| **Average Reputation Score** | Yes | Yes | Yes |

---

### 2.2 Monitoring Setup

#### Prometheus Metrics (Example Configuration)

```yaml
# prometheus.yml
global:
  scrape_interval: 30s
  evaluation_interval: 30s

scrape_configs:
  - job_name: 'wastefi-contracts'
    static_configs:
      - targets: ['metrics.wastefi.io:9090']
    
alerting:
  alertmanagers:
    - static_configs:
        - targets: ['localhost:9093']

rule_files:
  - 'wastefi_alerts.yml'
```

#### Alert Rules (wastefi_alerts.yml)

```yaml
groups:
  - name: wastefi_critical
    interval: 1m
    rules:
      - alert: HighRiskUserSpike
        expr: high_risk_users_count > 20
        for: 5m
        labels:
          severity: critical
          team: security
        annotations:
          summary: "High spike in high-risk users"
          description: "{{ $value }} high-risk users detected in last 5 minutes"
      
      - alert: TransactionFailureRateHigh
        expr: (failed_transactions / total_transactions) > 0.05
        for: 10m
        labels:
          severity: critical
          team: operations
        annotations:
          summary: "Transaction failure rate exceeded 5%"
          description: "Failure rate: {{ $value | humanizePercentage }}"
      
      - alert: AdminBalanceLow
        expr: admin_xlm_balance < 20
        for: 1m
        labels:
          severity: critical
          team: operations
        annotations:
          summary: "Admin account balance critically low"
          description: "Current balance: {{ $value }} XLM"

  - name: wastefi_warning
    interval: 5m
    rules:
      - alert: PriceDataStale
        expr: time() - last_price_update_timestamp > 86400
        labels:
          severity: warning
          team: operations
        annotations:
          summary: "Material pricing data is stale"
          description: "Last update was {{ $value | humanizeDuration }} ago"
      
      - alert: RateLimitHitsIncreasing
        expr: rate(rate_limit_hits[1h]) > 200
        labels:
          severity: warning
          team: security
        annotations:
          summary: "Rate limit hits increasing"
          description: "Current rate: {{ $value }} hits/hour"
```

---

### 2.3 Dashboard Setup

#### Key Dashboards

**1. Operations Dashboard**
- System health (uptime, response times)
- Transaction volume (real-time, 24h, 7d)
- Error rates
- Resource usage

**2. Security Dashboard**
- Risk score distribution
- Fraud detection alerts
- Rate limit statistics
- Emergency level status
- Admin activity log

**3. Business Dashboard**
- User growth (collectors, collection points)
- Waste collection volume
- Rewards distribution
- Reputation score trends
- Geographic distribution (if available)

**4. Performance Dashboard**
- Gas usage per contract
- Transaction processing times
- Storage usage trends
- Network latency

#### Example Grafana Dashboard Query

```sql
-- Transaction volume over time
SELECT
  time_bucket('1 hour', timestamp) AS time,
  COUNT(*) as transactions
FROM waste_transactions
WHERE timestamp > NOW() - INTERVAL '7 days'
GROUP BY time
ORDER BY time;

-- Risk score distribution
SELECT
  CASE
    WHEN risk_score < 300 THEN 'Low'
    WHEN risk_score < 600 THEN 'Medium'
    WHEN risk_score < 800 THEN 'High'
    ELSE 'Critical'
  END as risk_category,
  COUNT(*) as count
FROM fraud_detection_events
WHERE timestamp > NOW() - INTERVAL '24 hours'
GROUP BY risk_category;
```

---

### 2.4 Alert Configuration

#### Slack Webhook Integration

```bash
# Configure Slack webhook in config file
# config/mainnet.json
{
  "monitoring": {
    "alert_webhook": "https://hooks.slack.com/services/YOUR/WEBHOOK/URL"
  }
}

# Test alert
curl -X POST \
  -H 'Content-type: application/json' \
  --data '{"text":"Test alert from WasteFi monitoring"}' \
  https://hooks.slack.com/services/YOUR/WEBHOOK/URL
```

#### Email Alerts

```bash
# Configure email alerts
# /etc/alertmanager/config.yml
route:
  receiver: 'wastefi-team'
  group_by: ['alertname', 'severity']
  group_wait: 10s
  group_interval: 10s
  repeat_interval: 1h
  routes:
    - match:
        severity: critical
      receiver: 'wastefi-critical'
      repeat_interval: 15m

receivers:
  - name: 'wastefi-critical'
    email_configs:
      - to: 'oncall@wastefi.io'
        from: 'alerts@wastefi.io'
        smarthost: 'smtp.gmail.com:587'
        auth_username: 'alerts@wastefi.io'
        auth_password: '<PASSWORD>'
```

---

## 3. Common Operational Tasks

### 3.1 Update Material Prices

**Frequency**: As needed (recommended weekly)  
**Duration**: 5-10 minutes  
**Authorization**: Admin only

```bash
#!/bin/bash
# update_prices.sh - Update material pricing

# Load contract addresses
MATERIAL_PRICING=$(jq -r '.contracts.material_pricing' deployed_addresses_mainnet.json)

# Update plastic price
soroban contract invoke \
  --id $MATERIAL_PRICING \
  --source mainnet-admin \
  --network mainnet \
  -- \
  update_price \
  --material "plastic" \
  --price 120

# Update paper price
soroban contract invoke \
  --id $MATERIAL_PRICING \
  --source mainnet-admin \
  --network mainnet \
  -- \
  update_price \
  --material "paper" \
  --price 85

# Update metal price
soroban contract invoke \
  --id $MATERIAL_PRICING \
  --source mainnet-admin \
  --network mainnet \
  -- \
  update_price \
  --material "metal" \
  --price 160

# Update glass price
soroban contract invoke \
  --id $MATERIAL_PRICING \
  --source mainnet-admin \
  --network mainnet \
  -- \
  update_price \
  --material "glass" \
  --price 65

# Update organic price
soroban contract invoke \
  --id $MATERIAL_PRICING \
  --source mainnet-admin \
  --network mainnet \
  -- \
  update_price \
  --material "organic" \
  --price 45

echo "✓ All material prices updated"

# Verify updates
for material in plastic paper metal glass organic; do
  PRICE=$(soroban contract invoke \
    --id $MATERIAL_PRICING \
    --network mainnet \
    -- \
    get_price \
    --material "$material")
  echo "$material: $PRICE"
done
```

---

### 3.2 Review and Manage High-Risk Users

**Frequency**: Daily (if high-risk alerts triggered)  
**Duration**: 15-30 minutes  
**Authorization**: Admin or operator

```bash
#!/bin/bash
# review_high_risk_users.sh

WASTE_TRANSACTION=$(jq -r '.contracts.waste_transaction' deployed_addresses_mainnet.json)
COLLECTOR_REGISTRY=$(jq -r '.contracts.collector_registry' deployed_addresses_mainnet.json)

# Get list of recent high-risk flagged users (implement query method)
# Note: This requires contract to have a query method for high-risk users

echo "Reviewing high-risk users..."

# Example: Ban a user if confirmed fraudulent
# USER_ADDRESS="GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"

# soroban contract invoke \
#   --id $COLLECTOR_REGISTRY \
#   --source mainnet-admin \
#   --network mainnet \
#   -- \
#   update_status \
#   --collector "$USER_ADDRESS" \
#   --status "Banned"

# echo "User $USER_ADDRESS banned"

# Generate report
echo "High-risk user review completed at $(date)"
```

---

### 3.3 Fund Admin Account

**Frequency**: As needed (when balance <50 XLM)  
**Duration**: 5 minutes  
**Authorization**: Finance team

```bash
# Check current balance
ADMIN_ADDRESS=$(soroban keys address mainnet-admin)

soroban contract invoke \
  --network mainnet \
  --source mainnet-admin \
  -- \
  balance \
  --id $ADMIN_ADDRESS

# Transfer XLM from treasury to admin account
# (Use Stellar Lab or wallet to transfer)
# Recommended: Maintain 100 XLM minimum balance

# Verify new balance
soroban contract invoke \
  --network mainnet \
  --source mainnet-admin \
  -- \
  balance \
  --id $ADMIN_ADDRESS
```

---

### 3.4 Review and Approve Collection Points

**Frequency**: As needed (when new registrations)  
**Duration**: 10-20 minutes per point  
**Authorization**: Admin or operator

```bash
#!/bin/bash
# review_collection_points.sh

COLLECTION_POINT=$(jq -r '.contracts.collection_point' deployed_addresses_mainnet.json)

# List pending collection points (requires query method)
echo "Fetching pending collection points..."

# Example: Approve a collection point
# POINT_ID="<COLLECTION_POINT_ID>"

# soroban contract invoke \
#   --id $COLLECTION_POINT \
#   --source mainnet-admin \
#   --network mainnet \
#   -- \
#   approve_point \
#   --point_id "$POINT_ID"

# echo "Collection point $POINT_ID approved"

# Verification checklist:
# - Valid location
# - Contact information verified
# - Operational capacity confirmed
# - Compliance with local regulations
```

---

### 3.5 Adjust Fraud Detection Thresholds

**Frequency**: Monthly or as needed  
**Duration**: 15 minutes  
**Authorization**: Admin only (requires careful review)

```bash
#!/bin/bash
# adjust_fraud_thresholds.sh

WASTE_TRANSACTION=$(jq -r '.contracts.waste_transaction' deployed_addresses_mainnet.json)

# Review current thresholds
echo "Current fraud detection settings..."

# Update thresholds if needed (example)
# soroban contract invoke \
#   --id $WASTE_TRANSACTION \
#   --source mainnet-admin \
#   --network mainnet \
#   -- \
#   update_fraud_config \
#   --critical_threshold 850 \
#   --high_threshold 650

# Verify update
echo "Fraud detection thresholds updated"
echo "Monitor for false positive/negative changes over next 7 days"
```

---

### 3.6 Generate Operational Reports

**Frequency**: Weekly (Monday), Monthly (1st of month)  
**Duration**: 30-60 minutes  
**Authorization**: Operations lead

```bash
#!/bin/bash
# generate_operational_report.sh

REPORT_DATE=$(date +%Y-%m-%d)
REPORT_FILE="reports/ops_report_$REPORT_DATE.md"

mkdir -p reports

cat > $REPORT_FILE <<EOF
# WasteFi Operational Report - $REPORT_DATE

## System Health
- Uptime: [Query monitoring system]
- Total Transactions: [Query metrics]
- Average Transaction Time: [Query metrics]
- Failed Transactions: [Query metrics]

## Security
- High-Risk Users Flagged: [Query fraud detection]
- Rate Limit Violations: [Query metrics]
- Incidents: [List P2+ incidents]

## Business Metrics
- New Collectors: [Query collector registry]
- Active Collectors: [Query metrics]
- Total Waste Collected: [Query transaction history]
- Rewards Distributed: [Query payment distribution]

## Actions Taken
- [List significant operational actions]

## Issues and Concerns
- [List any ongoing issues]

## Recommendations
- [Operational recommendations]

---
Generated by: $USER
Date: $REPORT_DATE
EOF

echo "Report generated: $REPORT_FILE"
```

---

## 4. System Maintenance

### 4.1 Routine Maintenance Windows

**Maintenance Schedule**:
- **Weekly**: Sunday 02:00-04:00 UTC (low traffic period)
- **Monthly**: First Sunday 02:00-06:00 UTC (extended window)

**Maintenance Activities**:
1. Configuration updates
2. Price adjustments
3. System optimization
4. Backup verification
5. Security patches (if needed)

**Maintenance Checklist**:
- [ ] Notify users 48h in advance (for extended maintenance)
- [ ] Enable maintenance mode if needed
- [ ] Perform maintenance activities
- [ ] Run post-maintenance verification
- [ ] Re-enable full operations
- [ ] Monitor for 2 hours post-maintenance
- [ ] Update status page

---

### 4.2 Backup Procedures

**Backup Items**:
1. Contract addresses (daily)
2. Configuration files (daily)
3. Admin keys (secure vault, daily verification)
4. Monitoring data (daily)
5. Operational logs (weekly archive)

```bash
#!/bin/bash
# backup.sh - Daily backup script

BACKUP_DIR="backups/$(date +%Y%m%d)"
mkdir -p $BACKUP_DIR

# Backup contract addresses
cp deployed_addresses_mainnet.json $BACKUP_DIR/

# Backup configuration
cp config/mainnet.json $BACKUP_DIR/mainnet.json.enc  # Encrypted

# Backup logs
tar -czf $BACKUP_DIR/logs.tar.gz /var/log/wastefi/*.log

# Upload to secure storage (S3, etc.)
# aws s3 cp $BACKUP_DIR s3://wastefi-backups/$BACKUP_DIR --recursive

echo "Backup completed: $BACKUP_DIR"

# Cleanup old backups (keep 90 days)
find backups/ -type d -mtime +90 -exec rm -rf {} \;
```

---

### 4.3 Log Management

**Log Retention**:
- Operational logs: 90 days
- Security logs: 1 year
- Audit logs: 7 years (compliance)

```bash
#!/bin/bash
# rotate_logs.sh - Weekly log rotation

LOG_DIR="/var/log/wastefi"
ARCHIVE_DIR="/var/log/wastefi/archive"

mkdir -p $ARCHIVE_DIR

# Rotate logs older than 7 days
find $LOG_DIR -name "*.log" -mtime +7 -exec gzip {} \;
find $LOG_DIR -name "*.log.gz" -exec mv {} $ARCHIVE_DIR/ \;

# Delete archived logs older than 90 days
find $ARCHIVE_DIR -name "*.log.gz" -mtime +90 -delete

echo "Log rotation completed"
```

---

## 5. Performance Optimization

### 5.1 Gas Usage Monitoring

```bash
#!/bin/bash
# monitor_gas_usage.sh

echo "Analyzing gas usage patterns..."

# Query recent transactions for gas usage
# (Requires monitoring integration)

# Identify high-gas operations
# Generate recommendations for optimization

echo "Gas usage report generated"
```

**Optimization Targets**:
- Transaction recording: <10,000 gas
- Payment distribution: <15,000 gas
- Reputation update: <5,000 gas

---

### 5.2 Storage Optimization

```bash
#!/bin/bash
# check_storage_usage.sh

for contract in waste_token collector_registry collection_point \
                material_pricing reputation waste_transaction \
                payment_distribution; do
  CONTRACT_ID=$(jq -r ".contracts.$contract" deployed_addresses_mainnet.json)
  
  echo "Storage usage for $contract:"
  # Query storage metrics (requires monitoring)
done

# Recommendations for cleanup if storage growing too fast
```

---

## 6. Data Management

### 6.1 Data Retention Policy

| Data Type | Retention Period | Archive | Deletion |
|-----------|------------------|---------|----------|
| **Active Transactions** | Indefinite | N/A | Never |
| **Collector Profiles** | Active + 1 year | Yes | After 1 year inactive |
| **Collection Points** | Active + 1 year | Yes | After 1 year inactive |
| **Payment Records** | 7 years | Yes | After 7 years (compliance) |
| **Reputation History** | 3 years | Yes | After 3 years |
| **Security Logs** | 1 year | Yes | After 1 year |

---

## 7. User Support

### 7.1 Common User Issues

**Issue 1: "My transaction is stuck"**
- Check transaction status on explorer
- Verify collector status is "Active"
- Check if emergency mode active
- Verify transaction meets all requirements

**Issue 2: "Payment not received"**
- Verify transaction verified
- Check payment distribution status
- Verify token balance in wallet
- Check for any payment distribution errors

**Issue 3: "Cannot register as collector"**
- Verify rate limits not exceeded
- Check if already registered
- Verify all required information provided

---

## 8. Reporting and Metrics

### 8.1 SLA Targets

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| **Uptime** | 99.9% | [Track] | 🟢 |
| **Transaction Success Rate** | >99% | [Track] | 🟢 |
| **Response Time** | <2s | [Track] | 🟢 |
| **Alert Response** | <15min | [Track] | 🟢 |

---

## 9. On-Call Procedures

### 9.1 On-Call Rotation

**Rotation**: 7-day shifts, Sunday to Sunday  
**Response Time**: <15 minutes for P0, <1 hour for P1  
**Escalation**: If no response in 30 minutes, escalate to next on-call

### 9.2 On-Call Checklist

**At Start of Shift**:
- [ ] Verify monitoring access
- [ ] Verify admin key access
- [ ] Review current system status
- [ ] Review open incidents
- [ ] Test alert delivery (phone, email, Slack)

**During Shift**:
- [ ] Respond to alerts per SLA
- [ ] Document all actions taken
- [ ] Escalate if needed
- [ ] Update status page for incidents

**At End of Shift**:
- [ ] Hand off open incidents
- [ ] Update runbook with learnings
- [ ] Brief next on-call engineer

---

## 10. Emergency Procedures Reference

### 10.1 Quick Reference

For detailed emergency procedures, see **INCIDENT_RESPONSE.md**

**P0 - Critical Emergency**:
1. Acknowledge alert immediately
2. Assess impact and scope
3. Activate emergency shutdown if needed
4. Notify all stakeholders
5. Begin incident response
6. Document everything

**Emergency Shutdown**:
```bash
# Set all contracts to shutdown mode
./scripts/emergency_shutdown.sh mainnet
```

**Emergency Contacts**:
- **Security Lead**: security@wastefi.io
- **On-Call Engineer**: +1-XXX-XXX-XXXX
- **CTO**: cto@wastefi.io

---

## Appendix A: Useful Commands Reference

### Contract Queries
```bash
# Check contract version
soroban contract invoke --id <CONTRACT_ID> --network mainnet -- version

# Get admin
soroban contract invoke --id <CONTRACT_ID> --network mainnet -- get_admin

# Get emergency level
soroban contract invoke --id <CONTRACT_ID> --network mainnet -- get_emergency_level
```

### System Health
```bash
# Check all contracts operational
./scripts/health_check.sh mainnet

# Get transaction volume
./scripts/get_metrics.sh --metric transactions --period 24h

# Check admin balance
./scripts/check_balance.sh mainnet-admin
```

---

## Document Maintenance

**Review Schedule**: Monthly  
**Owner**: Operations Lead  
**Contributors**: DevOps team, Security team  
**Change Log**: Track in git commits

---

## Related Documentation

- `INCIDENT_RESPONSE.md` - Emergency response procedures
- `DEPLOYMENT.md` - Deployment guide
- `MONITORING.md` - Monitoring setup (TBD)
- `API.md` - API reference
- `SECURITY_AUDIT.md` - Security documentation

---

**END OF OPERATIONS RUNBOOK**

For questions: ops@wastefi.io

# WasteFi Incident Response Plan

## Document Purpose

This document defines procedures for detecting, responding to, and recovering from security incidents affecting the WasteFi platform. It provides a structured approach to incident management, ensuring rapid and effective response to minimize impact.

**Version**: 0.1.0  
**Date**: September 2026  
**Status**: Pre-Mainnet  
**Review Cycle**: Quarterly

---

## Table of Contents

1. [Incident Classification](#1-incident-classification)
2. [Response Team Structure](#2-response-team-structure)
3. [Detection and Alerting](#3-detection-and-alerting)
4. [Response Procedures by Severity](#4-response-procedures-by-severity)
5. [Communication Protocols](#5-communication-protocols)
6. [Post-Incident Analysis](#6-post-incident-analysis)
7. [Emergency Contacts](#7-emergency-contacts)
8. [Appendices](#8-appendices)

---

## 1. Incident Classification

### 1.1 Severity Levels

#### P0 - CRITICAL (Emergency)
**Definition**: Active exploitation, imminent fund loss, or complete system compromise

**Examples**:
- Active exploit draining funds
- Admin key compromise confirmed
- Unlimited token minting detected
- Multiple contract failures

**Response Time**: **IMMEDIATE** (< 15 minutes)  
**Emergency Level**: **Shutdown**  
**Escalation**: All hands on deck

**Indicators**:
- Unusual transaction patterns (100+ TXs/minute)
- Large unexpected token mints
- Multiple emergency triggers
- Admin actions from unknown addresses
- Circuit breaker trips across multiple contracts

---

#### P1 - HIGH (Urgent)
**Definition**: Significant security risk, potential fund loss, or major functionality impact

**Examples**:
- Fraud detection bypass discovered
- Price manipulation in progress
- High-value payment anomaly
- Single contract failure
- Suspicious admin activity

**Response Time**: **< 1 hour**  
**Emergency Level**: **Critical**  
**Escalation**: Lead + Security + On-call engineer

**Indicators**:
- Risk scores spiking (10+ users >800)
- Material price changes >50% in 1 hour
- Payment amounts >3x average
- Repeated verification failures
- Suspicious collector registration pattern

---

#### P2 - MEDIUM (Important)
**Definition**: Security concern requiring attention, limited impact, workaround available

**Examples**:
- Elevated fraud activity
- Rate limit excessive hits
- Collection point suspicious pattern
- Stale pricing data (>24h)
- Circuit breaker single trip

**Response Time**: **< 4 hours**  
**Emergency Level**: **Warning**  
**Escalation**: On-call engineer

**Indicators**:
- 5+ collectors flagged for fraud in 1 hour
- Rate limit hits increasing 50%
- Price not updated in 24 hours
- Duplicate transaction attempts increasing
- Single circuit breaker trip

---

#### P3 - LOW (Standard)
**Definition**: Minor issue, no immediate risk, can be addressed during business hours

**Examples**:
- Single fraud flag
- Query performance degradation
- Event log anomaly
- Documentation update needed

**Response Time**: **< 24 hours**  
**Emergency Level**: **Normal**  
**Escalation**: Standard on-call rotation

---

#### P4 - INFORMATIONAL
**Definition**: Observation, question, or potential improvement

**Examples**:
- Gas optimization opportunity
- User experience issue
- Monitoring enhancement suggestion

**Response Time**: **Next sprint**  
**Emergency Level**: **Normal**  
**Escalation**: Backlog

---

### 1.2 Emergency Level Mapping

| Incident Severity | Emergency Level | Contract State | Admin Action Required |
|-------------------|-----------------|----------------|----------------------|
| P0 - Critical | Shutdown (3) | All operations halted | Immediate manual intervention |
| P1 - High | Critical (2) | Contract paused | Urgent review and decision |
| P2 - Medium | Warning (1) | Operations continue | Enhanced monitoring |
| P3 - Low | Normal (0) | Operations continue | Standard handling |
| P4 - Info | Normal (0) | Operations continue | None |

---

## 2. Response Team Structure

### 2.1 Roles and Responsibilities

#### Incident Commander (IC)
**Primary**: [Lead Developer Name]  
**Backup**: [Project Lead Name]

**Responsibilities**:
- Overall incident coordination
- Decision authority
- Communication with stakeholders
- Post-incident review leadership

**Required Skills**: Technical depth, decision-making under pressure, communication

---

#### Security Lead
**Primary**: [Security Engineer Name]  
**Backup**: [Senior Developer Name]

**Responsibilities**:
- Security assessment
- Attack vector analysis
- Mitigation recommendations
- Forensic investigation

**Required Skills**: Smart contract security, cryptography, threat analysis

---

#### Technical Lead
**Primary**: [Smart Contract Developer Name]  
**Backup**: [Backend Developer Name]

**Responsibilities**:
- Technical investigation
- Code analysis
- Fix implementation
- Deployment coordination

**Required Skills**: Rust, Soroban, WasteFi architecture

---

#### Communications Lead
**Primary**: [Community Manager Name]  
**Backup**: [Project Manager Name]

**Responsibilities**:
- User communication
- Social media updates
- Documentation updates
- Transparency reporting

**Required Skills**: Technical writing, community management, crisis communication

---

#### Operations Lead
**Primary**: [DevOps Engineer Name]  
**Backup**: [Infrastructure Engineer Name]

**Responsibilities**:
- Monitoring and alerting
- Log analysis
- Infrastructure stability
- Deployment operations

**Required Skills**: Monitoring tools, stellar network operations, incident management

---

### 2.2 On-Call Rotation

**Schedule**: 7-day rotation, 24/7 coverage

**On-Call Engineer Responsibilities**:
- Monitor alerts
- Initial triage (< 15 min)
- Escalate P0/P1 incidents
- Document all incidents
- Weekly handoff report

**On-Call Compensation**: [To be defined]

**Handoff Protocol**:
1. Review open incidents
2. Check monitoring dashboards
3. Review recent changes
4. Update on-call contact info
5. Document any concerns

---

## 3. Detection and Alerting

### 3.1 Monitoring Systems

#### Contract Events (Real-Time)
**Tool**: [Event monitoring service]

**Alerts**:
- ✅ Emergency triggers (any level)
- ✅ Admin role changes
- ✅ Fraud flags (risk >800)
- ✅ Large token mints (>100k tokens)
- ✅ Payment anomalies (>3x average)
- ✅ Circuit breaker trips

**Channels**: PagerDuty, Slack #alerts, Email

---

#### Transaction Monitoring
**Tool**: [Blockchain explorer API + custom scripts]

**Alerts**:
- ✅ Transaction volume spikes (>50% increase)
- ✅ Failed transaction rate (>10%)
- ✅ Unusual transaction patterns
- ✅ Admin transactions from new addresses
- ✅ Contract upgrade events

**Channels**: Slack #monitoring, Email

---

#### Fraud Detection Metrics
**Tool**: [Custom dashboard]

**Alerts**:
- ✅ 5+ critical risk collectors in 1 hour
- ✅ Average risk score >400
- ✅ Duplicate attempts >20/hour
- ✅ Rate limit hits >100/hour
- ✅ Weight anomaly frequency increase

**Channels**: Slack #fraud-alerts

---

#### Pricing Oracle
**Tool**: [Price monitoring service]

**Alerts**:
- ✅ Price change >30% in 1 hour
- ✅ Price staleness >24 hours
- ✅ Price outside bounds (min/max)
- ✅ Operator changes

**Channels**: Slack #pricing, Email

---

### 3.2 Alert Routing

```
┌─────────────┐
│   Events    │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Severity   │
│ Classifier  │
└──────┬──────┘
       │
       ├─── P0 ────▶ PagerDuty (all team) + SMS
       ├─── P1 ────▶ PagerDuty (on-call) + Slack
       ├─── P2 ────▶ Slack + Email
       ├─── P3 ────▶ Slack only
       └─── P4 ────▶ Dashboard only
```

---

## 4. Response Procedures by Severity

### 4.1 P0 - CRITICAL Incident Response

**Timeline**: 0-15 minutes

#### Step 1: IMMEDIATE ACTION (0-5 min)
1. **Acknowledge Alert**: On-call engineer acknowledges within 2 minutes
2. **Assess Severity**: Quick review of alert details
3. **Trigger Emergency Shutdown** (if necessary):
   ```rust
   // Admin action
   wastefi_contracts::trigger_emergency(
       EmergencyLevel::Shutdown,
       "P0 incident: [brief description]"
   );
   ```
4. **Escalate**: Page Incident Commander and entire team
5. **Open War Room**: Create incident Slack channel `#incident-YYYYMMDD-NNN`

#### Step 2: CONTAINMENT (5-15 min)
1. **Verify Shutdown**: Confirm all contracts paused
2. **Assess Impact**:
   - How many users affected?
   - Amount of funds at risk?
   - Attack still active?
3. **Isolate Attack Vector**:
   - Identify compromised accounts
   - Block malicious addresses (if possible)
   - Document attack pattern
4. **Initial Communication**: Post status on social media

**Status Update Template**:
```
🚨 INCIDENT ALERT

We've detected a security issue and have paused the WasteFi platform as a precaution.

- All contracts are safe
- Funds are secure
- No user action required
- Updates every 30 minutes

Investigation underway. Thank you for your patience.
```

#### Step 3: INVESTIGATION (15-60 min)
1. **Root Cause Analysis**:
   - Review transaction logs
   - Analyze attack pattern
   - Identify vulnerability
   - Estimate exploit window
2. **Impact Assessment**:
   - Calculate funds affected
   - Identify affected users
   - Document state changes
3. **Develop Fix**:
   - Security Lead + Technical Lead collaborate
   - Code fix or configuration change
   - Review with team
4. **Test Fix**: On testnet if possible

#### Step 4: RESOLUTION (1-4 hours)
1. **Deploy Fix** (if needed):
   - Admin approval required
   - Upgrade contracts or configuration
   - Verify fix applied
2. **Gradual Recovery**:
   - Step down emergency level: Shutdown → Critical → Warning → Normal
   - Monitor closely at each step
   - Be ready to re-pause if issues detected
3. **Verify Stability**: 1 hour of normal operations
4. **Announce Resolution**: Public communication

**Resolution Template**:
```
✅ INCIDENT RESOLVED

The security issue has been resolved and the platform is fully operational.

Summary:
- Issue: [Brief description]
- Duration: [X hours]
- Impact: [User/fund impact]
- Fix: [What was done]

Full post-mortem will be published within 7 days.

Thank you for your patience and trust.
```

---

### 4.2 P1 - HIGH Incident Response

**Timeline**: 0-60 minutes

#### Step 1: TRIAGE (0-10 min)
1. **Acknowledge**: On-call engineer acknowledges
2. **Assess**: Review alert details, check dashboards
3. **Escalate**: Page Incident Commander and Security Lead
4. **Open Channel**: Create `#incident-YYYYMMDD-NNN`

#### Step 2: INVESTIGATE (10-30 min)
1. **Trigger Warning Level** (if appropriate):
   ```rust
   wastefi_contracts::trigger_emergency(
       EmergencyLevel::Warning,
       "P1 incident under investigation: [description]"
   );
   ```
2. **Collect Data**:
   - Transaction history
   - Event logs
   - Affected accounts
   - Timeline of events
3. **Analyze Attack Vector**:
   - How was it exploited?
   - Is it still active?
   - What's the potential impact?

#### Step 3: RESPOND (30-60 min)
1. **Mitigation Options**:
   - **Option A**: Pause contract (if significant risk)
   - **Option B**: Enhanced monitoring (if low immediate risk)
   - **Option C**: Admin intervention (block specific accounts)
2. **Implement Mitigation**: Based on team decision
3. **Monitor**: Watch for continued exploitation
4. **Communication**: Internal stakeholders notified

#### Step 4: RESOLVE (1-4 hours)
1. **Deploy Fix** (if needed)
2. **Verify Resolution**
3. **Document**: Full incident documentation
4. **Schedule Post-Mortem**: Within 48 hours

---

### 4.3 P2 - MEDIUM Incident Response

**Timeline**: 0-4 hours

#### Standard Response
1. **Acknowledge** (0-15 min)
2. **Investigate** (15-60 min):
   - Review data
   - Assess if escalation needed
   - Document findings
3. **Respond** (1-2 hours):
   - Implement fix or workaround
   - Update monitoring if needed
   - Document resolution
4. **Monitor** (2-4 hours):
   - Verify issue resolved
   - Watch for recurrence

**No Public Communication Required** (unless escalated)

---

### 4.4 P3/P4 - LOW/INFO Response

**Timeline**: Business hours

#### Standard Process
1. **Triage**: On-call reviews during business hours
2. **Document**: Create ticket in issue tracker
3. **Prioritize**: Add to backlog with appropriate priority
4. **Resolve**: Address in next sprint or release

**No Escalation or Communication Required**

---

## 5. Communication Protocols

### 5.1 Internal Communication

#### War Room (P0/P1 Only)
- **Platform**: Slack channel `#incident-YYYYMMDD-NNN`
- **Participants**: Response team only
- **Rules**:
  - All findings documented in channel
  - No side conversations (keep everything centralized)
  - IC has final decision authority
  - Timeline posted every 30 minutes

#### Status Updates
- **Frequency**: 
  - P0: Every 30 minutes
  - P1: Every 1 hour
  - P2: Every 4 hours
- **Recipients**: All team members
- **Content**: Current status, next steps, ETA

---

### 5.2 External Communication

#### Social Media (Twitter/X, Discord, Telegram)
**P0 Incidents Only**

**Initial Post** (within 30 min):
```
🚨 We've detected a security issue and paused operations as a precaution.
- All contracts safe
- Funds secure
- Updates every 30 min
#WasteFi #SecurityFirst
```

**Update Posts** (every 30 min):
```
⏱️ INCIDENT UPDATE [HH:MM]

Status: Investigating/Fixing/Testing/Resolving
Progress: [Brief update]
Next update: [Time]

Thank you for your patience.
```

**Resolution Post**:
```
✅ INCIDENT RESOLVED

Operations restored. Full details: [link to post-mortem]
Thank you for your patience and trust.
```

---

#### Email (All Users)
**P0/P1 Incidents**

**Template**:
```
Subject: [RESOLVED/UPDATE] WasteFi Security Incident - [Date]

Dear WasteFi User,

[Status: In Progress / Resolved]

SUMMARY:
- What happened: [Brief description]
- Impact: [How you might be affected]
- Actions required: [None / Specific actions]
- Current status: [Resolved / In progress]

TIMELINE:
- [Time]: Issue detected
- [Time]: Operations paused
- [Time]: Fix implemented
- [Time]: Operations restored

YOUR FUNDS ARE SAFE. No user action required.

We take security seriously and will publish a full post-mortem within 7 days.

Thank you,
The WasteFi Team

Questions? security@wastefi.io
```

---

### 5.3 Communication Decision Matrix

| Severity | Twitter | Discord | Email | Blog Post | Post-Mortem |
|----------|---------|---------|-------|-----------|-------------|
| P0 | ✅ Real-time | ✅ Real-time | ✅ After resolution | ✅ Required | ✅ Within 7 days |
| P1 | ⚠️ If user-facing | ✅ Real-time | ⚠️ If major impact | ⚠️ Optional | ✅ Within 14 days |
| P2 | ❌ No | ⚠️ Optional | ❌ No | ❌ No | ⚠️ Internal only |
| P3/P4 | ❌ No | ❌ No | ❌ No | ❌ No | ❌ No |

---

## 6. Post-Incident Analysis

### 6.1 Post-Mortem Meeting

**Timing**: Within 48 hours for P0/P1, within 1 week for P2

**Participants**: All response team members

**Agenda**:
1. Timeline review (what happened when)
2. Response effectiveness (what went well)
3. Gaps identified (what could improve)
4. Root cause analysis (why did it happen)
5. Action items (how to prevent recurrence)

---

### 6.2 Post-Mortem Document

**Template**: See Appendix A

**Required Sections**:
- Executive summary
- Incident timeline
- Root cause analysis
- Impact assessment
- Response effectiveness
- Lessons learned
- Action items (with owners and deadlines)

**Distribution**:
- P0: Public (blog post)
- P1: Public if user-impacting, internal otherwise
- P2: Internal only

---

### 6.3 Action Item Tracking

**Process**:
1. All action items documented in post-mortem
2. Tickets created in issue tracker
3. Assigned owners and deadlines
4. Reviewed weekly in team meeting
5. Closed only when verified complete

**Categories**:
- **Prevent**: Changes to prevent recurrence
- **Detect**: Improve monitoring and alerting
- **Respond**: Enhance response procedures
- **Learn**: Training and documentation updates

---

## 7. Emergency Contacts

### 7.1 Response Team

| Role | Primary | Backup | Phone | Email |
|------|---------|--------|-------|-------|
| Incident Commander | [Name] | [Name] | [+X-XXX-XXX-XXXX] | [email] |
| Security Lead | [Name] | [Name] | [+X-XXX-XXX-XXXX] | [email] |
| Technical Lead | [Name] | [Name] | [+X-XXX-XXX-XXXX] | [email] |
| Communications Lead | [Name] | [Name] | [+X-XXX-XXX-XXXX] | [email] |
| Operations Lead | [Name] | [Name] | [+X-XXX-XXX-XXXX] | [email] |

### 7.2 External Contacts

| Entity | Contact | Purpose |
|--------|---------|---------|
| Stellar Foundation | [Contact] | Network issues, technical support |
| Security Auditor | [Firm] | Emergency security consultation |
| Legal Counsel | [Firm] | Legal implications, regulatory |
| Insurance Provider | [Company] | Coverage claims |
| Key Exchanges | [Contacts] | Trading halts, user protection |

### 7.3 Admin Wallet Access

**Primary Admin Wallet**: [Stellar address]  
**Access Control**: Multi-sig (planned)  
**Key Holders**: [Names]  
**Emergency Recovery**: [Process]

**⚠️ CRITICAL**: Admin keys must be accessible 24/7 for emergency response

---

## 8. Appendices

### Appendix A: Post-Mortem Template

```markdown
# Incident Post-Mortem: [Incident Name]

**Date**: [YYYY-MM-DD]  
**Severity**: [P0/P1/P2]  
**Duration**: [X hours Y minutes]  
**Incident Commander**: [Name]

## Executive Summary
[2-3 paragraph summary of what happened, impact, and resolution]

## Timeline (All times in UTC)
- **[HH:MM]** - Initial detection
- **[HH:MM]** - Alert triggered
- **[HH:MM]** - Incident commander paged
- **[HH:MM]** - Emergency shutdown triggered
- **[HH:MM]** - Root cause identified
- **[HH:MM]** - Fix implemented
- **[HH:MM]** - Services restored
- **[HH:MM]** - Incident closed

## Root Cause Analysis
### What Happened
[Detailed technical explanation]

### Why It Happened
[Underlying causes]

### How It Was Detected
[Detection method]

## Impact Assessment
### Users Affected
[Number and description]

### Funds At Risk
[Amount and status]

### Downtime
[Duration and services affected]

## Response Effectiveness
### What Went Well
- [Item 1]
- [Item 2]

### What Could Be Improved
- [Item 1]
- [Item 2]

## Lessons Learned
1. [Lesson 1]
2. [Lesson 2]
3. [Lesson 3]

## Action Items
| ID | Action | Owner | Deadline | Status |
|----|--------|-------|----------|--------|
| AI-1 | [Action description] | [Name] | [Date] | Open |
| AI-2 | [Action description] | [Name] | [Date] | Open |

## Supporting Data
[Links to logs, dashboards, charts]

---

**Document Status**: [Draft / Under Review / Published]  
**Review Date**: [YYYY-MM-DD]  
**Reviewers**: [Names]
```

---

### Appendix B: Emergency Command Reference

#### Trigger Emergency
```bash
# Via Soroban CLI (example)
soroban contract invoke \
  --id <COLLECTOR_REGISTRY_ID> \
  --source-account <ADMIN_SECRET_KEY> \
  --rpc-url <RPC_URL> \
  --network-passphrase <PASSPHRASE> \
  -- \
  trigger_emergency \
  --level 3 \
  --reason "P0 incident: [description]"
```

#### Resolve Emergency
```bash
soroban contract invoke \
  --id <COLLECTOR_REGISTRY_ID> \
  --source-account <ADMIN_SECRET_KEY> \
  --rpc-url <RPC_URL> \
  --network-passphrase <PASSPHRASE> \
  -- \
  resolve_emergency
```

#### Check Emergency Status
```bash
soroban contract invoke \
  --id <COLLECTOR_REGISTRY_ID> \
  --rpc-url <RPC_URL> \
  --network-passphrase <PASSPHRASE> \
  -- \
  get_emergency_level
```

#### Flag User for Fraud
```bash
soroban contract invoke \
  --id <WASTE_TRANSACTION_ID> \
  --source-account <ADMIN_SECRET_KEY> \
  --rpc-url <RPC_URL> \
  --network-passphrase <PASSPHRASE> \
  -- \
  flag_for_review \
  --collector <USER_ADDRESS> \
  --reason "[description]"
```

---

### Appendix C: Incident Classification Flowchart

```
┌─────────────────┐
│  Alert Received │
└────────┬────────┘
         │
         ▼
    ┌────────────┐
    │ Funds at   │───YES───▶ P0 (CRITICAL)
    │ immediate  │
    │ risk?      │
    └────┬───────┘
         │NO
         ▼
    ┌────────────┐
    │ Active     │───YES───▶ P1 (HIGH)
    │ exploitation│
    │ or major   │
    │ impact?    │
    └────┬───────┘
         │NO
         ▼
    ┌────────────┐
    │ Security   │───YES───▶ P2 (MEDIUM)
    │ concern    │
    │ requires   │
    │ attention? │
    └────┬───────┘
         │NO
         ▼
    ┌────────────┐
    │ Minor      │───YES───▶ P3 (LOW)
    │ issue or   │
    │ observation│
    └────┬───────┘
         │NO
         ▼
         P4 (INFO)
```

---

### Appendix D: Incident Response Checklist

**P0 INCIDENT CHECKLIST**

Initial Response (0-5 min):
- [ ] Acknowledge alert
- [ ] Assess severity
- [ ] Trigger emergency shutdown (if needed)
- [ ] Escalate to full team
- [ ] Open war room channel

Containment (5-15 min):
- [ ] Verify shutdown complete
- [ ] Assess impact
- [ ] Identify attack vector
- [ ] Post initial status update

Investigation (15-60 min):
- [ ] Root cause analysis
- [ ] Impact quantification
- [ ] Develop fix
- [ ] Test fix

Resolution (1-4 hours):
- [ ] Deploy fix
- [ ] Gradual recovery
- [ ] Verify stability
- [ ] Announce resolution

Post-Incident (Within 48 hours):
- [ ] Schedule post-mortem meeting
- [ ] Write post-mortem document
- [ ] Create action items
- [ ] Publish public post-mortem (if P0)

---

## Document Maintenance

**Review Cycle**: Quarterly  
**Owner**: Security Lead  
**Last Reviewed**: [Date]  
**Next Review**: [Date]

**Version History**:
| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 0.1.0 | 2026-09-11 | WasteFi Team | Initial incident response plan |

---

**End of Incident Response Plan**

For immediate security concerns, contact: security@wastefi.io  
For emergencies, page on-call: [PagerDuty/Phone number]

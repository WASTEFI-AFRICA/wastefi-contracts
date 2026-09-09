# WasteFi Smart Contracts - Incident Response Plan

## Document Information

**Version**: 1.0.0  
**Date**: February 2024  
**Status**: Active  
**Review Frequency**: Quarterly

---

## 1. Executive Summary

This incident response plan defines procedures for detecting, responding to, and recovering from security incidents affecting WasteFi smart contracts. The plan establishes severity levels, response procedures, escalation paths, and communication protocols.

### Incident Categories
- **P0 (Critical)**: Active exploit, funds at risk, system compromised
- **P1 (High)**: Potential exploit, elevated risk, service degradation
- **P2 (Medium)**: Suspicious activity, performance issues, non-critical bugs
- **P3 (Low)**: Minor issues, feature requests, documentation updates
- **P4 (Informational)**: Observations, suggestions, non-urgent items

---

## 2. Incident Classification

### 2.1 P0 - Critical Incidents

**Definition**: Active security breach or imminent threat to funds/data requiring immediate action.

**Examples**:
- Active fund drainage exploit
- Unauthorized contract upgrade
- Admin key compromise confirmed
- Smart contract vulnerability being exploited
- Complete system outage

**Response Time**: Immediate (< 15 minutes)  
**Response Team**: All hands on deck  
**Communication**: Immediate stakeholder notification

**Actions**:
1. Trigger emergency shutdown (if applicable)
2. Pause all contracts
3. Assess damage and attack vector
4. Deploy mitigation
5. Initiate forensic investigation

---

### 2.2 P1 - High Severity Incidents

**Definition**: Potential security issue or significant service degradation requiring urgent attention.

**Examples**:
- Suspected vulnerability reported
- Unusual transaction patterns detected
- Circuit breaker trip (repeated)
- High fraud risk activity surge
- Payment processing failures
- Contract upgrade failure

**Response Time**: < 1 hour  
**Response Team**: Security team + on-call engineer  
**Communication**: Stakeholder notification within 2 hours

**Actions**:
1. Assess severity and validate report
2. Activate enhanced monitoring
3. Implement temporary mitigations
4. Plan permanent fix
5. Prepare deployment

---

### 2.3 P2 - Medium Severity Incidents

**Definition**: Issues impacting functionality or user experience but not immediate security threat.

**Examples**:
- Rate limit false positives
- Fraud detection anomalies
- Performance degradation
- Non-critical bugs
- Configuration errors
- Documentation inaccuracies

**Response Time**: < 4 hours  
**Response Team**: On-call engineer  
**Communication**: Internal notification

**Actions**:
1. Investigate and document
2. Implement workaround if needed
3. Schedule fix in next release
4. Update monitoring
5. Document lessons learned

---

### 2.4 P3 - Low Severity Incidents

**Definition**: Minor issues with minimal impact on operations.

**Examples**:
- Feature enhancement requests
- Cosmetic issues
- Gas optimization opportunities
- Test coverage gaps
- Minor documentation updates

**Response Time**: < 24 hours (acknowledgment)  
**Response Team**: Development team  
**Communication**: Issue tracker update

**Actions**:
1. Log in issue tracker
2. Prioritize in backlog
3. Schedule for future sprint
4. Close when resolved

---

### 2.5 P4 - Informational

**Definition**: Non-urgent observations or suggestions.

**Examples**:
- Code quality improvements
- Best practice suggestions
- Future feature ideas
- Research opportunities

**Response Time**: No specific SLA  
**Response Team**: Product/Engineering  
**Communication**: Acknowledge receipt

---

## 3. Incident Response Procedures

### 3.1 Detection

#### Automated Detection
- **Fraud alerts**: High-risk score detected
- **Circuit breaker trips**: Repeated failures
- **Rate limit violations**: Unusual patterns
- **Emergency triggers**: Manual admin activation
- **Gas anomalies**: Unexpected consumption
- **Balance alerts**: Unusual fund movements

#### Manual Detection
- User reports via support channels
- Security researcher reports
- Internal code review findings
- External audit findings
- Community observations

#### Detection Tools
- Event monitoring dashboard
- Fraud detection metrics
- Transaction analytics
- Balance tracking
- Gas consumption tracking

---

### 3.2 Initial Response (First 15 Minutes)

#### Step 1: Validate Incident (2 minutes)
- [ ] Confirm incident is real (not false positive)
- [ ] Classify severity (P0-P4)
- [ ] Identify affected contracts/functions
- [ ] Estimate scope of impact

#### Step 2: Assemble Response Team (3 minutes)
- [ ] Page on-call engineer
- [ ] Alert security team (P0/P1)
- [ ] Notify technical lead
- [ ] Activate incident commander

#### Step 3: Initiate Containment (10 minutes)
**For P0 Critical Incidents**:
- [ ] Trigger emergency pause (if necessary)
- [ ] Activate emergency level (Warning/Critical/Shutdown)
- [ ] Trip circuit breakers for affected operations
- [ ] Document initial observations
- [ ] Begin incident log

**For P1 High Incidents**:
- [ ] Enable enhanced monitoring
- [ ] Implement temporary rate limits
- [ ] Flag suspicious accounts
- [ ] Prepare rollback plan

---

### 3.3 Investigation Phase

#### Forensic Analysis
1. **Gather Evidence**
   - Transaction logs
   - Event history
   - Admin action logs
   - Fraud detection data
   - Contract state snapshots

2. **Analyze Attack Vector**
   - Identify vulnerability exploited
   - Map attack steps
   - Estimate attacker capabilities
   - Assess damage

3. **Impact Assessment**
   - Funds affected
   - Users impacted
   - Data compromised
   - Reputation damage

4. **Root Cause Analysis**
   - Code review of vulnerable area
   - Identify how vulnerability was introduced
   - Review related code for similar issues
   - Document findings

#### Investigation Checklist
- [ ] Collect all relevant logs and events
- [ ] Interview team members if applicable
- [ ] Review recent code changes
- [ ] Check for related vulnerabilities
- [ ] Assess blast radius
- [ ] Document timeline
- [ ] Preserve evidence

---

### 3.4 Mitigation & Recovery

#### Immediate Mitigation
**For Active Exploits (P0)**:
1. **Emergency Pause**
   ```rust
   // Admin executes
   admin.trigger_emergency(
       EmergencyLevel::Shutdown,
       "Active exploit: [description]"
   );
   ```

2. **Circuit Breaker Activation**
   ```rust
   // Trip affected operations
   CircuitBreaker::trip(env, "vulnerable_operation");
   ```

3. **Fraud Flagging**
   ```rust
   // Flag attacker accounts
   FraudDetection::flag_for_review(
       env, attacker_address, 
       "Confirmed exploit attempt"
   );
   ```

#### Permanent Fix Deployment
1. **Code Fix**
   - Patch vulnerability
   - Add tests for exploit scenario
   - Conduct code review
   - Test on local environment

2. **Testnet Deployment**
   - Deploy to testnet
   - Verify fix works
   - Test edge cases
   - Validate no regressions

3. **Mainnet Deployment**
   - Prepare deployment plan
   - Coordinate with team
   - Execute upgrade
   - Verify deployment
   - Monitor closely

4. **Recovery Steps**
   - Resolve emergency state
   - Reset circuit breakers
   - Clear false-positive fraud flags
   - Resume normal operations
   - Verify system health

---

### 3.5 Communication

#### Internal Communication

**P0 Incidents** (Immediate):
```
TO: All Engineering, Leadership
SUBJECT: [P0] Critical Security Incident - [Brief Description]

SITUATION: [What happened]
IMPACT: [Affected users/funds/services]
STATUS: [Current state]
ACTIONS: [What we're doing]
NEXT UPDATE: [Timeframe]
```

**P1 Incidents** (Within 1 hour):
```
TO: Engineering Team, Security Team
SUBJECT: [P1] Security Incident - [Brief Description]

DETAILS: [What was detected]
SEVERITY: [Impact assessment]
RESPONSE: [Current actions]
TIMELINE: [Expected resolution]
```

#### External Communication

**Stakeholder Notification Template**:
```
TO: [Stakeholders/Community]
SUBJECT: Security Update - [Date]

We are writing to inform you of a security incident affecting WasteFi contracts.

WHAT HAPPENED: [Brief non-technical description]
WHEN: [Timestamp]
IMPACT: [Who/what is affected]
OUR RESPONSE: [Actions taken]
YOUR ACTION: [If any required from users]
STATUS: [Current state]
NEXT STEPS: [Resolution plan]

We take security seriously and are committed to transparency. 
We will provide updates as we learn more.

Contact: security@wastefi.example
```

#### Communication Guidelines
- Be transparent but don't reveal exploit details publicly until fixed
- Update stakeholders regularly (every 2-4 hours for P0)
- Acknowledge user reports promptly
- Post mortem published after resolution
- Coordinate with auditors if applicable

---

## 4. Contact Escalation Matrix

### 4.1 Response Team Roles

#### Incident Commander
**Responsibility**: Overall incident coordination  
**Contact**: [Primary Contact]  
**Backup**: [Backup Contact]  
**Availability**: 24/7 on-call rotation

#### Security Lead
**Responsibility**: Security analysis and mitigation  
**Contact**: [Security Contact]  
**Backup**: [Backup Security]  
**Availability**: 24/7 for P0/P1

#### Technical Lead
**Responsibility**: Code fixes and deployment  
**Contact**: [Tech Lead Contact]  
**Backup**: [Backup Tech]  
**Availability**: Business hours + on-call

#### DevOps Engineer
**Responsibility**: Infrastructure and monitoring  
**Contact**: [DevOps Contact]  
**Backup**: [Backup DevOps]  
**Availability**: 24/7 on-call

#### Communications Lead
**Responsibility**: Stakeholder communication  
**Contact**: [Comms Contact]  
**Backup**: [Backup Comms]  
**Availability**: Business hours + on-call for P0

### 4.2 Escalation Path

```
Level 1: On-Call Engineer (Receives alert)
    ↓ (If P0/P1 or needs help)
Level 2: Security Lead + Technical Lead
    ↓ (If critical or unclear resolution)
Level 3: Incident Commander + Engineering Manager
    ↓ (If business impact or external communication needed)
Level 4: Executive Leadership
```

### 4.3 External Contacts

**Security Researchers**:
- Responsible disclosure: security@wastefi.example
- Response SLA: 24 hours acknowledgment

**Auditors**:
- [Audit Firm Name]
- Contact: [Auditor Email]
- Purpose: Consult on complex vulnerabilities

**Stellar Network**:
- Stellar Support: [Contact]
- Purpose: Network-level issues

**Legal Counsel**:
- Firm: [Law Firm]
- Contact: [Legal Email]
- Purpose: Regulatory compliance, user impact

---

## 5. Emergency Procedures

### 5.1 Emergency Shutdown Procedure

**When to Use**: Active exploit draining funds or compromising system integrity.

**Steps**:
1. **Trigger Emergency** (Admin):
   ```bash
   # Connect as admin
   soroban contract invoke \
     --id [CONTRACT_ID] \
     --source admin \
     -- trigger_emergency \
     --level 3 \
     --reason "Active exploit detected"
   ```

2. **Verify Pause State**:
   - Check all contracts paused
   - Verify operations blocked
   - Confirm events emitted

3. **Assess Damage**:
   - Check balances (pre/post)
   - Identify affected transactions
   - List compromised accounts

4. **Coordinate Response**:
   - Brief response team
   - Begin investigation
   - Prepare fix

5. **Document Everything**:
   - Incident timeline
   - Actions taken
   - Evidence collected
   - Communications sent

### 5.2 Admin Key Compromise Procedure

**Indicators**:
- Unauthorized admin actions detected
- Admin key theft reported
- Suspicious transactions from admin address

**Immediate Actions**:
1. **Assume Worst Case**: Attacker has full admin control

2. **Emergency Contact**:
   - Alert all team members immediately
   - Contact exchange partners (if tokens listed)
   - Notify users via all channels

3. **Containment** (if possible):
   - If backup admin exists, use it to pause
   - If not, coordinate with Stellar support
   - Document all unauthorized actions

4. **Recovery**:
   - Deploy new contracts if necessary
   - Migrate state to new contracts
   - Snapshot balances before compromise
   - Plan user migration

5. **Post-Incident**:
   - Full security audit
   - Implement multi-sig
   - Review all admin actions post-compromise
   - Compensation plan for affected users

### 5.3 Smart Contract Upgrade Rollback

**When to Use**: Upgrade introduces critical bug or vulnerability.

**Steps**:
1. **Assess Severity**:
   - Is rollback necessary?
   - Can bug be patched forward?
   - What data might be lost?

2. **Prepare Rollback**:
   ```bash
   # Prepare previous WASM hash
   PREVIOUS_WASM="[hash]"
   
   # Coordinate with team
   # Schedule rollback window
   ```

3. **Execute Rollback**:
   ```bash
   soroban contract invoke \
     --id [CONTRACT_ID] \
     --source admin \
     -- upgrade_contract \
     --new_wasm_hash $PREVIOUS_WASM
   ```

4. **Verify Rollback**:
   - Check contract version
   - Test critical functions
   - Verify state integrity

5. **Post-Rollback**:
   - Investigate why upgrade failed
   - Fix issues in new version
   - Re-test thoroughly before redeployment

---

## 6. Post-Incident Analysis

### 6.1 Post-Mortem Template

**Incident ID**: [Unique identifier]  
**Date**: [Date of incident]  
**Duration**: [Start - End]  
**Severity**: [P0-P4]  
**Incident Commander**: [Name]

#### What Happened
- Brief description of incident
- Timeline of events
- Attack vector (if applicable)

#### Impact
- Users affected: [Number]
- Funds affected: [Amount]
- Service downtime: [Duration]
- Data compromised: [What data]

#### Root Cause
- Technical cause
- How vulnerability was introduced
- Why it wasn't caught earlier

#### Response Effectiveness
**What Went Well**:
- Detection speed
- Response coordination
- Mitigation effectiveness
- Communication clarity

**What Could Be Improved**:
- Gaps in monitoring
- Response delays
- Communication issues
- Tool limitations

#### Action Items
| Action | Owner | Priority | Due Date | Status |
|--------|-------|----------|----------|--------|
| [Action] | [Name] | [P0-P4] | [Date] | [Open/Closed] |

#### Lessons Learned
- What we learned about our system
- What we learned about our processes
- What we learned about our team

#### Preventive Measures
- Code changes
- Process improvements
- Monitoring enhancements
- Training needs

### 6.2 Post-Incident Review Meeting

**Attendees**: Response team + management  
**Timing**: Within 48 hours of resolution  
**Duration**: 60-90 minutes

**Agenda**:
1. Incident overview (10 min)
2. Timeline walkthrough (20 min)
3. Impact assessment (10 min)
4. Response review (20 min)
5. Action items (20 min)
6. Lessons learned (10 min)

**Output**: Documented post-mortem + action plan

---

## 7. Preventive Measures

### 7.1 Continuous Monitoring

**Daily**:
- Review fraud detection metrics
- Check rate limit violations
- Monitor transaction volume
- Review error rates

**Weekly**:
- Analyze fraud patterns
- Review admin actions
- Check system health
- Update alerting thresholds

**Monthly**:
- Security review
- Performance analysis
- Capacity planning
- Incident review

### 7.2 Security Practices

- Regular code audits
- Dependency updates
- Security training
- Incident response drills
- Bug bounty program
- Responsible disclosure policy

### 7.3 Operational Excellence

- Runbook maintenance
- Documentation updates
- Process improvements
- Tool enhancements
- Team training

---

## 8. Incident Response Drills

### 8.1 Drill Schedule
- **Monthly**: Tabletop exercise (30 min)
- **Quarterly**: Simulated P1 incident
- **Annually**: Simulated P0 incident (full team)

### 8.2 Drill Scenarios

**Scenario 1: Fraud Surge**
- Mass fraudulent transactions detected
- Practice fraud detection and response
- Test communication procedures

**Scenario 2: Circuit Breaker Trip**
- Multiple operations failing
- Practice investigation and recovery
- Test monitoring tools

**Scenario 3: Admin Key Compromise**
- Simulated unauthorized admin action
- Practice emergency response
- Test backup procedures

**Scenario 4: Contract Upgrade Failure**
- Upgrade introduces critical bug
- Practice rollback procedure
- Test downtime minimization

### 8.3 Drill Evaluation
- Response time metrics
- Team coordination effectiveness
- Communication clarity
- Tool functionality
- Documentation accuracy

---

## 9. Tools & Resources

### 9.1 Incident Response Tools

**Monitoring**:
- Event monitoring dashboard
- Fraud detection metrics
- Transaction analytics
- Gas consumption tracking

**Communication**:
- Incident chat channel (Slack/Discord)
- Email distribution lists
- Status page
- Social media accounts

**Documentation**:
- Incident log template
- Post-mortem template
- Communication templates
- Runbook repository

### 9.2 Emergency Access

**Admin Keys**:
- Location: [Secure storage]
- Access: [Who has access]
- Backup: [Backup location]

**Contract Addresses**:
- Documented in deployment records
- Backup copies maintained
- Verified on-chain

**Credentials**:
- Monitoring dashboards
- Cloud infrastructure
- Communication platforms

---

## 10. Plan Maintenance

### 10.1 Review Schedule
- **Monthly**: Contact information verification
- **Quarterly**: Procedure review and drill
- **Annually**: Comprehensive plan update

### 10.2 Update Triggers
- After each incident (incorporate lessons learned)
- After team changes (update contacts)
- After system changes (update procedures)
- After external events (industry incidents)

### 10.3 Version History

| Version | Date | Changes | Author |
|---------|------|---------|--------|
| 1.0.0 | Feb 2024 | Initial version | [Name] |

---

## Appendix A: Quick Reference Guide

### P0 Critical Incident Checklist
- [ ] Validate incident (2 min)
- [ ] Alert response team (3 min)
- [ ] Trigger emergency pause (5 min)
- [ ] Begin investigation (10 min)
- [ ] Notify stakeholders (15 min)
- [ ] Document everything

### Emergency Contact Numbers
- Incident Commander: [Phone]
- Security Lead: [Phone]
- Technical Lead: [Phone]

### Emergency Commands
```bash
# Trigger emergency shutdown
soroban contract invoke --id [ID] --source admin -- trigger_emergency --level 3 --reason "[reason]"

# Trip circuit breaker
# (Internal function, called via admin method)

# Pause contract
soroban contract invoke --id [ID] --source admin -- admin_pause
```

---

**End of Incident Response Plan**

*This plan should be reviewed regularly and updated based on incidents and drills. All team members should be familiar with their roles and responsibilities.*

# GrantFlow Architecture

## Overview

GrantFlow is a decentralized grant management system built on Stellar using Soroban smart contracts. The system enables transparent, milestone-based grant distribution with on-chain governance.

## System Components

### 1. Grant Pool Contract

**Purpose**: Manages grant pools and fund allocation

**Key Functions**:
- `initialize(admin)` - Set up contract with admin
- `create_pool(admin, name, description, total_funds)` - Create new grant pool
- `get_pool(pool_id)` - Retrieve pool details
- `allocate_funds(pool_id, amount)` - Reserve funds for approved proposal
- `return_funds(pool_id, amount)` - Return unused funds
- `deactivate_pool(admin, pool_id)` - Disable pool

**State**:
- Pool metadata (name, description, admin)
- Total and available funds
- Active status

**Events**:
- `pool_created` - New pool created
- `funds_allocated` - Funds reserved
- `funds_returned` - Funds returned
- `pool_deactivated` - Pool disabled

### 2. Proposal Contract

**Purpose**: Handles proposal submission, voting, and lifecycle management

**Key Functions**:
- `submit_proposal(...)` - Submit new grant proposal
- `vote(proposal_id, voter, vote_for, weight)` - Cast weighted vote
- `finalize_voting(proposal_id)` - Tally votes and determine outcome
- `get_proposal(proposal_id)` - Retrieve proposal details
- `update_status(proposal_id, new_status)` - Update proposal state

**State**:
- Proposal metadata (title, description, amount)
- Voting data (votes for/against, deadline)
- Status (Pending, Approved, Rejected, Active, Completed, Cancelled)
- Milestone descriptions

**Events**:
- `proposal_submitted` - New proposal created
- `vote_cast` - Vote recorded
- `voting_finalized` - Voting completed
- `status_updated` - Status changed

### 3. Milestone Contract

**Purpose**: Tracks milestone completion and fund release

**Key Functions**:
- `create_milestones(proposal_id, descriptions, amounts)` - Set up milestones
- `submit_milestone(milestone_id, submitter, evidence_url)` - Submit completion proof
- `approve_milestone(milestone_id, approver)` - Approve and release funds
- `reject_milestone(milestone_id, rejector, reason)` - Reject submission
- `mark_paid(milestone_id)` - Mark as paid
- `get_milestone(milestone_id)` - Retrieve milestone details

**State**:
- Milestone metadata (description, amount)
- Status (Pending, Submitted, Approved, Rejected, Paid)
- Evidence URL
- Timestamps (submitted, approved)

**Events**:
- `milestones_created` - Milestones initialized
- `milestone_submitted` - Evidence submitted
- `milestone_approved` - Milestone approved
- `milestone_rejected` - Milestone rejected
- `milestone_paid` - Payment completed

### 4. Governance Contract

**Purpose**: Manages voting parameters and power distribution

**Key Functions**:
- `initialize(admin, quorum_threshold, voting_duration)` - Set up governance
- `set_voting_power(admin, voter, power)` - Assign voting weight
- `get_voting_power(voter)` - Query voting weight
- `update_quorum(admin, new_threshold)` - Adjust quorum requirement
- `update_voting_duration(admin, new_duration)` - Adjust voting period
- `check_quorum(total_votes)` - Verify quorum met
- `get_params()` - Retrieve all parameters

**State**:
- Voting power per address
- Quorum threshold
- Voting duration
- Admin address

**Events**:
- `governance_initialized` - Contract initialized
- `voting_power_set` - Power assigned
- `quorum_updated` - Threshold changed
- `voting_duration_updated` - Duration changed

## Data Flow

### Grant Lifecycle

```
1. Pool Creation
   Admin → Grant Pool Contract
   ↓
   Pool created with budget

2. Proposal Submission
   Proposer → Proposal Contract
   ↓
   Proposal enters voting period

3. Voting
   Voters → Proposal Contract
   (weighted by Governance Contract)
   ↓
   Votes tallied

4. Voting Finalization
   Anyone → Proposal Contract
   ↓
   Status: Approved or Rejected

5. Milestone Creation (if approved)
   System → Milestone Contract
   ↓
   Milestones initialized

6. Milestone Submission
   Proposer → Milestone Contract
   ↓
   Evidence submitted for review

7. Milestone Approval
   Admin/Reviewer → Milestone Contract
   ↓
   Funds released

8. Payment
   System → Milestone Contract
   ↓
   Marked as paid
```

## Security Model

### Authentication
- All state-changing functions require `require_auth()`
- Admin-only functions verify admin address
- Proposers must authenticate submissions

### Authorization
- Pool admins control their pools
- Governance admin controls voting parameters
- Milestone approvers must be authorized

### Validation
- Voting period enforcement
- Double-vote prevention
- Fund availability checks
- Status transition validation

### Audit Trail
- All actions emit events
- Complete on-chain history
- Immutable record of decisions

## Storage Strategy

### Instance Storage
Used for:
- Contract configuration
- Admin addresses
- Global counters
- Governance parameters

### Persistent Storage
Used for:
- Pool data
- Proposal data
- Milestone data
- Voting records

## Integration Points

### External Contracts
- Can integrate with token contracts for voting power
- Can integrate with payment contracts for fund distribution
- Can integrate with reputation systems

### Off-Chain Systems
- Evidence URLs point to external storage (IPFS, etc.)
- Frontend applications query contract state
- Analytics systems consume events

## Scalability Considerations

### Current Design
- Linear storage per pool/proposal/milestone
- O(1) lookups by ID
- No pagination (frontend responsibility)

### Future Improvements
- Batch operations for multiple votes
- Delegated voting
- Proposal categories/tags
- Advanced search indices

## Upgrade Strategy

### Contract Upgrades
- Deploy new contract versions
- Migrate state if needed
- Update frontend to new addresses

### Parameter Updates
- Governance contract allows runtime updates
- No contract redeployment needed
- Backward compatible

## Testing Strategy

### Unit Tests
- Each contract has comprehensive tests
- Cover happy paths and edge cases
- Mock authentication for testing

### Integration Tests
- Test cross-contract interactions
- Verify event emissions
- Test complete workflows

### Testnet Deployment
- Deploy to Stellar testnet
- Manual testing of full lifecycle
- Performance validation

## Monitoring

### Key Metrics
- Pools created
- Proposals submitted
- Voting participation
- Milestone completion rate
- Funds distributed

### Events to Monitor
- All contract events
- Failed transactions
- Unusual voting patterns
- Milestone rejections

## Future Enhancements

### Phase 2
- Multi-signature approvals
- Reputation-based voting
- Proposal templates
- Automated fund distribution

### Phase 3
- DEX integration for token swaps
- Cross-chain bridges
- Mobile SDK
- Analytics dashboard

### Phase 4
- AI-powered proposal analysis
- Predictive funding models
- Automated compliance checks
- Advanced governance mechanisms

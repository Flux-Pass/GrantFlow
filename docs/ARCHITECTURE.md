# GrantFlow Architecture

## Overview

GrantFlow is a decentralized grant management system built on Stellar with Soroban smart contracts. The current codebase provides four independent contracts that model grant pools, proposals, milestones, and governance parameters.

The contracts are designed as a foundation for transparent grant programs. Some production features, such as cross-contract enforcement and token transfers, are not wired yet and are called out in this document.

## System Components

### Grant Pool Contract

**Purpose**: Manage grant pools and pool-level fund accounting.

**Key Functions**:

- `initialize(admin)` - Stores the contract admin and initializes the pool counter.
- `create_pool(admin, name, description, total_funds)` - Creates a pool and returns its ID.
- `get_pool(pool_id)` - Reads pool details.
- `allocate_funds(pool_id, amount)` - Reserves funds from a pool when enough budget is available.
- `return_funds(pool_id, amount)` - Adds unused funds back to a pool.
- `deactivate_pool(admin, pool_id)` - Disables a pool.
- `get_pool_count()` - Returns the number of pools created.

**State**:

- Pool metadata
- Pool admin
- Total and available funds
- Active status
- Pool counter

**Events**:

- `pool_created`
- `funds_allocated`
- `funds_returned`
- `pool_deactivated`

### Proposal Contract

**Purpose**: Handle proposal submission, voting, and proposal status.

**Key Functions**:

- `submit_proposal(...)` - Creates a proposal with a voting deadline.
- `vote(proposal_id, voter, vote_for, weight)` - Records a weighted vote.
- `finalize_voting(proposal_id)` - Finalizes the proposal by simple majority.
- `get_proposal(proposal_id)` - Reads proposal details.
- `update_status(proposal_id, new_status)` - Updates proposal status.
- `get_proposal_count()` - Returns the number of proposals created.

**State**:

- Proposal metadata
- Requested amount
- Votes for and against
- Voting deadline
- Proposal status
- Per-proposal voter records

**Events**:

- `proposal_submitted`
- `vote_cast`
- `voting_finalized`
- `status_updated`

### Milestone Contract

**Purpose**: Track milestone creation, evidence submission, review, and payment status.

**Key Functions**:

- `create_milestones(proposal_id, descriptions, amounts)` - Creates milestones for a proposal.
- `submit_milestone(milestone_id, submitter, evidence_url)` - Records milestone evidence.
- `approve_milestone(milestone_id, approver)` - Approves submitted evidence.
- `reject_milestone(milestone_id, rejector, reason)` - Rejects submitted evidence.
- `mark_paid(milestone_id)` - Marks an approved milestone as paid.
- `get_milestone(milestone_id)` - Reads milestone details.
- `get_proposal_milestone_count(proposal_id)` - Reads milestone count for a proposal.

**State**:

- Milestone metadata
- Milestone amount
- Review status
- Evidence URL
- Submitted and approved timestamps
- Milestone counter

**Events**:

- `milestones_created`
- `milestone_submitted`
- `milestone_approved`
- `milestone_rejected`
- `milestone_paid`

### Governance Contract

**Purpose**: Store voting power and governance parameters.

**Key Functions**:

- `initialize(admin, quorum_threshold, voting_duration)` - Initializes governance parameters.
- `set_voting_power(admin, voter, power)` - Assigns voting power to an address.
- `get_voting_power(voter)` - Reads voting power for an address.
- `update_quorum(admin, new_threshold)` - Updates the quorum threshold.
- `get_quorum()` - Reads the quorum threshold.
- `update_voting_duration(admin, new_duration)` - Updates default voting duration.
- `get_voting_duration()` - Reads default voting duration.
- `check_quorum(total_votes)` - Returns whether total votes meet quorum.
- `get_params()` - Reads governance parameters.

**State**:

- Governance admin
- Voting power by address
- Quorum threshold
- Default voting duration

**Events**:

- `governance_initialized`
- `voting_power_set`
- `quorum_updated`
- `voting_duration_updated`

## Grant Lifecycle

```text
1. Pool Creation
   Admin -> Grant Pool Contract
   |
   v
   Pool created with budget

2. Proposal Submission
   Proposer -> Proposal Contract
   |
   v
   Proposal enters voting period

3. Voting
   Voter -> Proposal Contract
   |
   v
   Weighted vote is recorded

4. Voting Finalization
   Anyone -> Proposal Contract
   |
   v
   Proposal becomes Approved or Rejected

5. Milestone Creation
   Operator -> Milestone Contract
   |
   v
   Milestones are created for an approved proposal

6. Milestone Submission
   Submitter -> Milestone Contract
   |
   v
   Evidence URL is recorded

7. Milestone Review
   Reviewer -> Milestone Contract
   |
   v
   Milestone becomes Approved or Rejected

8. Payment Recording
   Operator -> Milestone Contract
   |
   v
   Approved milestone is marked as paid
```

## Security Model

### Authentication

The contracts use `require_auth()` for direct user authorization on major state-changing actions:

- Pool creation and pool deactivation require the supplied admin address.
- Proposal submission requires the proposer address.
- Voting requires the voter address.
- Milestone submission requires the submitter address.
- Milestone approval and rejection require the reviewer address.
- Governance updates require the governance admin address.

### Authorization

The current implementation has basic authorization checks, but several functions intended for system or contract use are externally callable:

- `allocate_funds`
- `return_funds`
- `update_status`
- `create_milestones`
- `mark_paid`

Production deployments should add caller restrictions or cross-contract authorization before relying on these functions for real funds.

### Validation

Current validation includes:

- One-time initialization checks
- Pool active and available-fund checks
- Voting deadline enforcement
- Double-vote prevention
- Proposal status checks during voting and finalization
- Milestone status transition checks
- Governance admin checks for parameter updates

Current validation does not include:

- Automatic governance quorum enforcement in proposal finalization
- Automatic voting power lookup from the governance contract
- Automatic pool allocation when proposals are approved
- Native or token asset transfers for milestone payments

## Storage Strategy

The contracts use Soroban instance storage for contract state and indexed records:

- `Pool(id)` stores grant pool data.
- `Proposal(id)` stores proposal data.
- `Vote(proposal_id, voter)` stores whether a voter has voted.
- `Milestone(id)` stores milestone data.
- `ProposalMilestones(proposal_id)` stores milestone count per proposal.
- `VotingPower(address)` stores governance voting power.
- Counter keys store pool, proposal, and milestone totals.

## Integration Points

Potential future integrations include:

- Stellar asset contracts for escrow and milestone payment release
- Governance token contracts for voting power
- Reputation systems for non-token voting weight
- IPFS or similar storage for milestone evidence
- Frontend and analytics applications that read contract state and events

## Testing Strategy

Each contract includes a Rust test module under its own `src/test.rs`. Tests should cover:

- Happy-path workflows
- Authorization failures
- Invalid status transitions
- Boundary values for amounts and voting weights
- Event-critical state changes

Run all tests from the contracts workspace:

```bash
cd contracts
cargo test
```

## Upgrade Strategy

The current project does not define an on-chain upgrade mechanism. New versions should be deployed as new contracts, followed by any required state migration and application configuration updates.

## Future Enhancements

- Cross-contract calls between proposal, governance, grant pool, and milestone contracts
- Quorum enforcement during proposal finalization
- Token escrow and payment release
- Multi-signature milestone approvals
- Delegated or reputation-based voting
- Proposal categories and searchable indexes
- Frontend dashboard and event analytics

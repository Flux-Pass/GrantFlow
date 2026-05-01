# GrantFlow API Reference

Complete API documentation for all GrantFlow smart contracts.

## Grant Pool Contract

### initialize

Initialize the contract with an admin address.

```rust
fn initialize(env: Env, admin: Address)
```

**Parameters**:
- `admin`: Address with administrative privileges

**Authorization**: None (first-time only)

**Panics**: If already initialized

---

### create_pool

Create a new grant pool.

```rust
fn create_pool(
    env: Env,
    admin: Address,
    name: String,
    description: String,
    total_funds: i128
) -> u64
```

**Parameters**:
- `admin`: Pool administrator address
- `name`: Pool name
- `description`: Pool description
- `total_funds`: Total budget in stroops

**Returns**: Pool ID

**Authorization**: Requires `admin` signature

**Events**: `pool_created(pool_id, admin)`

---

### get_pool

Retrieve pool details.

```rust
fn get_pool(env: Env, pool_id: u64) -> GrantPool
```

**Parameters**:
- `pool_id`: Pool identifier

**Returns**: GrantPool struct

**Authorization**: None (read-only)

---

### allocate_funds

Reserve funds from pool for a proposal.

```rust
fn allocate_funds(env: Env, pool_id: u64, amount: i128) -> bool
```

**Parameters**:
- `pool_id`: Pool identifier
- `amount`: Amount to allocate

**Returns**: Success boolean

**Authorization**: None (called by proposal contract)

**Events**: `funds_allocated(pool_id, amount)`

---

### return_funds

Return unused funds to pool.

```rust
fn return_funds(env: Env, pool_id: u64, amount: i128)
```

**Parameters**:
- `pool_id`: Pool identifier
- `amount`: Amount to return

**Authorization**: None (called by proposal contract)

**Events**: `funds_returned(pool_id, amount)`

---

### deactivate_pool

Disable a grant pool.

```rust
fn deactivate_pool(env: Env, admin: Address, pool_id: u64)
```

**Parameters**:
- `admin`: Pool administrator
- `pool_id`: Pool identifier

**Authorization**: Requires `admin` signature

**Panics**: If caller is not pool admin

**Events**: `pool_deactivated(pool_id)`

---

### get_pool_count

Get total number of pools created.

```rust
fn get_pool_count(env: Env) -> u64
```

**Returns**: Pool count

**Authorization**: None (read-only)

---

## Proposal Contract

### submit_proposal

Submit a new grant proposal.

```rust
fn submit_proposal(
    env: Env,
    pool_id: u64,
    proposer: Address,
    title: String,
    description: String,
    requested_amount: i128,
    voting_duration: u64,
    milestones: Vec<String>
) -> u64
```

**Parameters**:
- `pool_id`: Target grant pool
- `proposer`: Proposal submitter
- `title`: Proposal title
- `description`: Detailed description
- `requested_amount`: Funds requested
- `voting_duration`: Voting period in seconds
- `milestones`: Milestone descriptions

**Returns**: Proposal ID

**Authorization**: Requires `proposer` signature

**Events**: `proposal_submitted(proposal_id, proposer)`

---

### vote

Cast a vote on a proposal.

```rust
fn vote(
    env: Env,
    proposal_id: u64,
    voter: Address,
    vote_for: bool,
    weight: i128
)
```

**Parameters**:
- `proposal_id`: Proposal identifier
- `voter`: Voter address
- `vote_for`: True for yes, false for no
- `weight`: Vote weight (from governance)

**Authorization**: Requires `voter` signature

**Panics**: 
- If voting period ended
- If already voted
- If proposal not in voting state

**Events**: `vote_cast(proposal_id, voter, vote_for, weight)`

---

### finalize_voting

Tally votes and determine outcome.

```rust
fn finalize_voting(env: Env, proposal_id: u64)
```

**Parameters**:
- `proposal_id`: Proposal identifier

**Authorization**: None (anyone can finalize)

**Panics**:
- If voting period not ended
- If already finalized

**Events**: `voting_finalized(proposal_id, status)`

---

### get_proposal

Retrieve proposal details.

```rust
fn get_proposal(env: Env, proposal_id: u64) -> Proposal
```

**Parameters**:
- `proposal_id`: Proposal identifier

**Returns**: Proposal struct

**Authorization**: None (read-only)

---

### update_status

Update proposal status.

```rust
fn update_status(env: Env, proposal_id: u64, new_status: ProposalStatus)
```

**Parameters**:
- `proposal_id`: Proposal identifier
- `new_status`: New status value

**Authorization**: None (internal use)

**Events**: `status_updated(proposal_id, new_status)`

---

### get_proposal_count

Get total number of proposals.

```rust
fn get_proposal_count(env: Env) -> u64
```

**Returns**: Proposal count

**Authorization**: None (read-only)

---

## Milestone Contract

### create_milestones

Initialize milestones for an approved proposal.

```rust
fn create_milestones(
    env: Env,
    proposal_id: u64,
    descriptions: Vec<String>,
    amounts: Vec<i128>
)
```

**Parameters**:
- `proposal_id`: Associated proposal
- `descriptions`: Milestone descriptions
- `amounts`: Payment amounts per milestone

**Authorization**: None (called by system)

**Panics**: If descriptions and amounts length mismatch

**Events**: `milestones_created(proposal_id, count)`

---

### submit_milestone

Submit milestone completion evidence.

```rust
fn submit_milestone(
    env: Env,
    milestone_id: u64,
    submitter: Address,
    evidence_url: String
)
```

**Parameters**:
- `milestone_id`: Milestone identifier
- `submitter`: Evidence submitter
- `evidence_url`: Link to completion proof

**Authorization**: Requires `submitter` signature

**Panics**: If milestone not in pending state

**Events**: `milestone_submitted(milestone_id, submitter)`

---

### approve_milestone

Approve milestone and authorize fund release.

```rust
fn approve_milestone(
    env: Env,
    milestone_id: u64,
    approver: Address
)
```

**Parameters**:
- `milestone_id`: Milestone identifier
- `approver`: Approver address

**Authorization**: Requires `approver` signature

**Panics**: If milestone not submitted

**Events**: `milestone_approved(milestone_id, approver, amount)`

---

### reject_milestone

Reject milestone submission.

```rust
fn reject_milestone(
    env: Env,
    milestone_id: u64,
    rejector: Address,
    reason: String
)
```

**Parameters**:
- `milestone_id`: Milestone identifier
- `rejector`: Rejector address
- `reason`: Rejection reason

**Authorization**: Requires `rejector` signature

**Panics**: If milestone not submitted

**Events**: `milestone_rejected(milestone_id, rejector, reason)`

---

### mark_paid

Mark milestone as paid.

```rust
fn mark_paid(env: Env, milestone_id: u64)
```

**Parameters**:
- `milestone_id`: Milestone identifier

**Authorization**: None (called after payment)

**Panics**: If milestone not approved

**Events**: `milestone_paid(milestone_id, amount)`

---

### get_milestone

Retrieve milestone details.

```rust
fn get_milestone(env: Env, milestone_id: u64) -> Milestone
```

**Parameters**:
- `milestone_id`: Milestone identifier

**Returns**: Milestone struct

**Authorization**: None (read-only)

---

### get_proposal_milestone_count

Get milestone count for a proposal.

```rust
fn get_proposal_milestone_count(env: Env, proposal_id: u64) -> u32
```

**Parameters**:
- `proposal_id`: Proposal identifier

**Returns**: Milestone count

**Authorization**: None (read-only)

---

## Governance Contract

### initialize

Initialize governance with parameters.

```rust
fn initialize(
    env: Env,
    admin: Address,
    quorum_threshold: i128,
    voting_duration: u64
)
```

**Parameters**:
- `admin`: Governance administrator
- `quorum_threshold`: Minimum votes required
- `voting_duration`: Default voting period

**Authorization**: None (first-time only)

**Panics**: If already initialized

**Events**: `governance_initialized(admin, quorum, duration)`

---

### set_voting_power

Assign voting power to an address.

```rust
fn set_voting_power(
    env: Env,
    admin: Address,
    voter: Address,
    power: i128
)
```

**Parameters**:
- `admin`: Governance administrator
- `voter`: Address to assign power
- `power`: Voting weight

**Authorization**: Requires `admin` signature

**Panics**: If caller not admin

**Events**: `voting_power_set(voter, power)`

---

### get_voting_power

Query voting power for an address.

```rust
fn get_voting_power(env: Env, voter: Address) -> i128
```

**Parameters**:
- `voter`: Address to query

**Returns**: Voting power (0 if not set)

**Authorization**: None (read-only)

---

### update_quorum

Update quorum threshold.

```rust
fn update_quorum(env: Env, admin: Address, new_threshold: i128)
```

**Parameters**:
- `admin`: Governance administrator
- `new_threshold`: New quorum value

**Authorization**: Requires `admin` signature

**Panics**: If caller not admin

**Events**: `quorum_updated(new_threshold)`

---

### get_quorum

Get current quorum threshold.

```rust
fn get_quorum(env: Env) -> i128
```

**Returns**: Quorum threshold

**Authorization**: None (read-only)

---

### update_voting_duration

Update default voting duration.

```rust
fn update_voting_duration(
    env: Env,
    admin: Address,
    new_duration: u64
)
```

**Parameters**:
- `admin`: Governance administrator
- `new_duration`: New duration in seconds

**Authorization**: Requires `admin` signature

**Panics**: If caller not admin

**Events**: `voting_duration_updated(new_duration)`

---

### get_voting_duration

Get default voting duration.

```rust
fn get_voting_duration(env: Env) -> u64
```

**Returns**: Voting duration in seconds

**Authorization**: None (read-only)

---

### check_quorum

Verify if quorum is met.

```rust
fn check_quorum(env: Env, total_votes: i128) -> bool
```

**Parameters**:
- `total_votes`: Total votes cast

**Returns**: True if quorum met

**Authorization**: None (read-only)

---

### get_params

Get all governance parameters.

```rust
fn get_params(env: Env) -> GovernanceParams
```

**Returns**: GovernanceParams struct

**Authorization**: None (read-only)

---

## Data Structures

### GrantPool

```rust
pub struct GrantPool {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub total_funds: i128,
    pub available_funds: i128,
    pub admin: Address,
    pub active: bool,
}
```

### Proposal

```rust
pub struct Proposal {
    pub id: u64,
    pub pool_id: u64,
    pub proposer: Address,
    pub title: String,
    pub description: String,
    pub requested_amount: i128,
    pub status: ProposalStatus,
    pub votes_for: i128,
    pub votes_against: i128,
    pub voting_deadline: u64,
    pub milestones: Vec<String>,
}
```

### ProposalStatus

```rust
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected,
    Active,
    Completed,
    Cancelled,
}
```

### Milestone

```rust
pub struct Milestone {
    pub id: u64,
    pub proposal_id: u64,
    pub description: String,
    pub amount: i128,
    pub status: MilestoneStatus,
    pub evidence_url: String,
    pub submitted_at: u64,
    pub approved_at: u64,
}
```

### MilestoneStatus

```rust
pub enum MilestoneStatus {
    Pending,
    Submitted,
    Approved,
    Rejected,
    Paid,
}
```

### GovernanceParams

```rust
pub struct GovernanceParams {
    pub quorum_threshold: i128,
    pub voting_duration: u64,
    pub min_proposal_amount: i128,
}
```

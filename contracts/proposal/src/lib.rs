#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

#[derive(Clone, PartialEq)]
#[contracttype]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected,
    Active,
    Completed,
    Cancelled,
}

#[derive(Clone)]
#[contracttype]
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

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Proposal(u64),
    ProposalCount,
    Vote(u64, Address), // (proposal_id, voter)
}

#[contract]
pub struct ProposalContract;

#[contractimpl]
impl ProposalContract {
    /// Submit a new proposal
    pub fn submit_proposal(
        env: Env,
        pool_id: u64,
        proposer: Address,
        title: String,
        description: String,
        requested_amount: i128,
        voting_duration: u64,
        milestones: Vec<String>,
    ) -> u64 {
        proposer.require_auth();

        let proposal_count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::ProposalCount)
            .unwrap_or(0);

        let proposal_id = proposal_count + 1;
        let voting_deadline = env.ledger().timestamp() + voting_duration;

        let proposal = Proposal {
            id: proposal_id,
            pool_id,
            proposer: proposer.clone(),
            title,
            description,
            requested_amount,
            status: ProposalStatus::Pending,
            votes_for: 0,
            votes_against: 0,
            voting_deadline,
            milestones,
        };

        env.storage()
            .instance()
            .set(&DataKey::Proposal(proposal_id), &proposal);
        env.storage()
            .instance()
            .set(&DataKey::ProposalCount, &proposal_id);

        env.events().publish(
            (String::from_str(&env, "proposal_submitted"),),
            (proposal_id, proposer),
        );

        proposal_id
    }

    /// Vote on a proposal
    pub fn vote(env: Env, proposal_id: u64, voter: Address, vote_for: bool, weight: i128) {
        voter.require_auth();

        let mut proposal: Proposal = env
            .storage()
            .instance()
            .get(&DataKey::Proposal(proposal_id))
            .unwrap();

        // Check if voting is still open
        if env.ledger().timestamp() > proposal.voting_deadline {
            panic!("Voting period ended");
        }

        if proposal.status != ProposalStatus::Pending {
            panic!("Proposal not in voting state");
        }

        // Check if already voted
        let vote_key = DataKey::Vote(proposal_id, voter.clone());
        if env.storage().instance().has(&vote_key) {
            panic!("Already voted");
        }

        // Record vote
        env.storage().instance().set(&vote_key, &vote_for);

        if vote_for {
            proposal.votes_for += weight;
        } else {
            proposal.votes_against += weight;
        }

        env.storage()
            .instance()
            .set(&DataKey::Proposal(proposal_id), &proposal);

        env.events().publish(
            (String::from_str(&env, "vote_cast"),),
            (proposal_id, voter, vote_for, weight),
        );
    }

    /// Finalize voting and determine outcome
    pub fn finalize_voting(env: Env, proposal_id: u64) {
        let mut proposal: Proposal = env
            .storage()
            .instance()
            .get(&DataKey::Proposal(proposal_id))
            .unwrap();

        if env.ledger().timestamp() <= proposal.voting_deadline {
            panic!("Voting period not ended");
        }

        if proposal.status != ProposalStatus::Pending {
            panic!("Proposal already finalized");
        }

        // Simple majority vote
        if proposal.votes_for > proposal.votes_against {
            proposal.status = ProposalStatus::Approved;
        } else {
            proposal.status = ProposalStatus::Rejected;
        }

        env.storage()
            .instance()
            .set(&DataKey::Proposal(proposal_id), &proposal);

        env.events().publish(
            (String::from_str(&env, "voting_finalized"),),
            (proposal_id, proposal.status.clone()),
        );
    }

    /// Get proposal details
    pub fn get_proposal(env: Env, proposal_id: u64) -> Proposal {
        env.storage()
            .instance()
            .get(&DataKey::Proposal(proposal_id))
            .unwrap()
    }

    /// Update proposal status
    pub fn update_status(env: Env, proposal_id: u64, new_status: ProposalStatus) {
        let mut proposal: Proposal = env
            .storage()
            .instance()
            .get(&DataKey::Proposal(proposal_id))
            .unwrap();

        proposal.status = new_status.clone();
        env.storage()
            .instance()
            .set(&DataKey::Proposal(proposal_id), &proposal);

        env.events().publish(
            (String::from_str(&env, "status_updated"),),
            (proposal_id, new_status),
        );
    }

    /// Cancel a proposal
    pub fn cancel_proposal(env: Env, proposal_id: u64, proposer: Address) {
        proposer.require_auth();

        let mut proposal: Proposal = env
            .storage()
            .instance()
            .get(&DataKey::Proposal(proposal_id))
            .unwrap();

        if proposal.proposer != proposer {
            panic!("Unauthorized");
        }

        if proposal.status != ProposalStatus::Pending {
            panic!("Proposal cannot be cancelled");
        }

        proposal.status = ProposalStatus::Cancelled;
        env.storage()
            .instance()
            .set(&DataKey::Proposal(proposal_id), &proposal);

        env.events().publish(
            (String::from_str(&env, "proposal_cancelled"),),
            proposal_id,
        );
    }

    /// Get total proposal count
    pub fn get_proposal_count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::ProposalCount)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test;

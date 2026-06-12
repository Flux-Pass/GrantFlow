#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[derive(Clone, PartialEq)]
#[contracttype]
/// Possible states for a grant milestone
pub enum MilestoneStatus {
    Pending,
    Submitted,
    Approved,
    Rejected,
    Paid,
}

#[derive(Clone)]
#[contracttype]
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

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Milestone(u64),
    MilestoneCount,
    ProposalMilestones(u64), // proposal_id -> milestone count
}

#[contract]
pub struct MilestoneContract;

#[contractimpl]
impl MilestoneContract {
    /// Create milestones for an approved proposal
    pub fn create_milestones(
        env: Env,
        proposal_id: u64,
        descriptions: soroban_sdk::Vec<String>,
        amounts: soroban_sdk::Vec<i128>,
    ) {
        if descriptions.len() != amounts.len() {
            panic!("Descriptions and amounts length mismatch");
        }

        let milestone_count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::MilestoneCount)
            .unwrap_or(0);

        let mut current_id = milestone_count;

        for i in 0..descriptions.len() {
            current_id += 1;

            let milestone = Milestone {
                id: current_id,
                proposal_id,
                description: descriptions.get(i).unwrap(),
                amount: amounts.get(i).unwrap(),
                status: MilestoneStatus::Pending,
                evidence_url: String::from_str(&env, ""),
                submitted_at: 0,
                approved_at: 0,
            };

            env.storage()
                .instance()
                .set(&DataKey::Milestone(current_id), &milestone);
        }

        env.storage()
            .instance()
            .set(&DataKey::MilestoneCount, &current_id);
        env.storage()
            .instance()
            .set(&DataKey::ProposalMilestones(proposal_id), &descriptions.len());

        env.events().publish(
            (String::from_str(&env, "milestones_created"),),
            (proposal_id, descriptions.len()),
        );
    }

    /// Submit milestone completion evidence
    pub fn submit_milestone(
        env: Env,
        milestone_id: u64,
        submitter: Address,
        evidence_url: String,
    ) {
        submitter.require_auth();

        let mut milestone: Milestone = env
            .storage()
            .instance()
            .get(&DataKey::Milestone(milestone_id))
            .unwrap();

        if milestone.status != MilestoneStatus::Pending {
            panic!("Milestone not in pending state");
        }

        milestone.status = MilestoneStatus::Submitted;
        milestone.evidence_url = evidence_url;
        milestone.submitted_at = env.ledger().timestamp();

        env.storage()
            .instance()
            .set(&DataKey::Milestone(milestone_id), &milestone);

        env.events().publish(
            (String::from_str(&env, "milestone_submitted"),),
            (milestone_id, submitter),
        );
    }

    /// Approve milestone and release funds
    pub fn approve_milestone(env: Env, milestone_id: u64, approver: Address) {
        approver.require_auth();

        let mut milestone: Milestone = env
            .storage()
            .instance()
            .get(&DataKey::Milestone(milestone_id))
            .unwrap();

        if milestone.status != MilestoneStatus::Submitted {
            panic!("Milestone not submitted");
        }

        milestone.status = MilestoneStatus::Approved;
        milestone.approved_at = env.ledger().timestamp();

        env.storage()
            .instance()
            .set(&DataKey::Milestone(milestone_id), &milestone);

        env.events().publish(
            (String::from_str(&env, "milestone_approved"),),
            (milestone_id, approver, milestone.amount),
        );
    }

    /// Reject milestone submission
    pub fn reject_milestone(env: Env, milestone_id: u64, rejector: Address, reason: String) {
        rejector.require_auth();

        let mut milestone: Milestone = env
            .storage()
            .instance()
            .get(&DataKey::Milestone(milestone_id))
            .unwrap();

        if milestone.status != MilestoneStatus::Submitted {
            panic!("Milestone not submitted");
        }

        milestone.status = MilestoneStatus::Rejected;

        env.storage()
            .instance()
            .set(&DataKey::Milestone(milestone_id), &milestone);

        env.events().publish(
            (String::from_str(&env, "milestone_rejected"),),
            (milestone_id, rejector, reason),
        );
    }

    /// Mark milestone as paid
    pub fn mark_paid(env: Env, milestone_id: u64) {
        let mut milestone: Milestone = env
            .storage()
            .instance()
            .get(&DataKey::Milestone(milestone_id))
            .unwrap();

        if milestone.status != MilestoneStatus::Approved {
            panic!("Milestone not approved");
        }

        milestone.status = MilestoneStatus::Paid;

        env.storage()
            .instance()
            .set(&DataKey::Milestone(milestone_id), &milestone);

        env.events().publish(
            (String::from_str(&env, "milestone_paid"),),
            (milestone_id, milestone.amount),
        );
    }

    /// Get milestone details
    pub fn get_milestone(env: Env, milestone_id: u64) -> Milestone {
        env.storage()
            .instance()
            .get(&DataKey::Milestone(milestone_id))
            .unwrap()
    }

    /// Get milestone count for a proposal
    pub fn get_proposal_milestone_count(env: Env, proposal_id: u64) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::ProposalMilestones(proposal_id))
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test;

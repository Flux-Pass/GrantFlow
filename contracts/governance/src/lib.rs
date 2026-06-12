#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    VotingPower(Address),
    QuorumThreshold,
    VotingDuration,
    Admin,
}

#[derive(Clone)]
#[contracttype]
/// Configuration parameters for grant governance
pub struct GovernanceParams {
    pub quorum_threshold: i128,
    pub voting_duration: u64,
    pub min_proposal_amount: i128,
}

#[contract]
pub struct GovernanceContract;

#[contractimpl]
impl GovernanceContract {
    /// Initialize governance with admin and default parameters
    pub fn initialize(
        env: Env,
        admin: Address,
        quorum_threshold: i128,
        voting_duration: u64,
    ) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::QuorumThreshold, &quorum_threshold);
        env.storage()
            .instance()
            .set(&DataKey::VotingDuration, &voting_duration);

        env.events().publish(
            (String::from_str(&env, "governance_initialized"),),
            (admin, quorum_threshold, voting_duration),
        );
    }

    /// Set voting power for an address (token-weighted or reputation-based)
    pub fn set_voting_power(env: Env, admin: Address, voter: Address, power: i128) {
        admin.require_auth();

        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if stored_admin != admin {
            panic!("Unauthorized");
        }

        env.storage()
            .instance()
            .set(&DataKey::VotingPower(voter.clone()), &power);

        env.events().publish(
            (String::from_str(&env, "voting_power_set"),),
            (voter, power),
        );
    }

    /// Get voting power for an address
    pub fn get_voting_power(env: Env, voter: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::VotingPower(voter))
            .unwrap_or(0)
    }

    /// Update quorum threshold
    pub fn update_quorum(env: Env, admin: Address, new_threshold: i128) {
        admin.require_auth();

        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if stored_admin != admin {
            panic!("Unauthorized");
        }

        env.storage()
            .instance()
            .set(&DataKey::QuorumThreshold, &new_threshold);

        env.events().publish(
            (String::from_str(&env, "quorum_updated"),),
            new_threshold,
        );
    }

    /// Get current quorum threshold
    pub fn get_quorum(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::QuorumThreshold)
            .unwrap_or(0)
    }

    /// Update voting duration
    pub fn update_voting_duration(env: Env, admin: Address, new_duration: u64) {
        admin.require_auth();

        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if stored_admin != admin {
            panic!("Unauthorized");
        }

        env.storage()
            .instance()
            .set(&DataKey::VotingDuration, &new_duration);

        env.events().publish(
            (String::from_str(&env, "voting_duration_updated"),),
            new_duration,
        );
    }

    /// Get voting duration
    pub fn get_voting_duration(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::VotingDuration)
            .unwrap_or(86400) // Default 1 day
    }

    /// Check if quorum is met for a proposal
    /// Total votes is sum of all voting weights cast
    pub fn check_quorum(env: Env, total_votes: i128) -> bool {
        let quorum: i128 = env
            .storage()
            .instance()
            .get(&DataKey::QuorumThreshold)
            .unwrap_or(0);
        total_votes >= quorum
    }

    /// Get governance parameters
    pub fn get_params(env: Env) -> GovernanceParams {
        GovernanceParams {
            quorum_threshold: env
                .storage()
                .instance()
                .get(&DataKey::QuorumThreshold)
                .unwrap_or(0),
            voting_duration: env
                .storage()
                .instance()
                .get(&DataKey::VotingDuration)
                .unwrap_or(86400),
            min_proposal_amount: 1000, // Hardcoded for now
        }
    }
}

#[cfg(test)]
mod test;

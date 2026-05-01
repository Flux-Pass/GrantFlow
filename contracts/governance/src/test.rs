#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_initialize_governance() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GovernanceContract);
    let client = GovernanceContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let quorum = 1000;
    let duration = 86400;

    client.initialize(&admin, &quorum, &duration);

    assert_eq!(client.get_quorum(), quorum);
    assert_eq!(client.get_voting_duration(), duration);
}

#[test]
fn test_set_and_get_voting_power() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register_contract(None, GovernanceContract);
    let client = GovernanceContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let voter = Address::generate(&env);

    client.initialize(&admin, &1000, &86400);
    client.set_voting_power(&admin, &voter, &500);

    assert_eq!(client.get_voting_power(&voter), 500);
}

#[test]
fn test_update_quorum() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register_contract(None, GovernanceContract);
    let client = GovernanceContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    client.initialize(&admin, &1000, &86400);
    client.update_quorum(&admin, &2000);

    assert_eq!(client.get_quorum(), 2000);
}

#[test]
fn test_check_quorum() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GovernanceContract);
    let client = GovernanceContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    client.initialize(&admin, &1000, &86400);

    assert!(client.check_quorum(&1500));
    assert!(!client.check_quorum(&500));
}

#[test]
fn test_get_params() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GovernanceContract);
    let client = GovernanceContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    client.initialize(&admin, &1000, &86400);

    let params = client.get_params();
    assert_eq!(params.quorum_threshold, 1000);
    assert_eq!(params.voting_duration, 86400);
}

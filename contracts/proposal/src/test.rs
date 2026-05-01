#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, vec, Address, Env, String};

#[test]
fn test_submit_proposal() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ProposalContract);
    let client = ProposalContractClient::new(&env, &contract_id);

    let proposer = Address::generate(&env);
    let milestones = vec![
        &env,
        String::from_str(&env, "Milestone 1"),
        String::from_str(&env, "Milestone 2"),
    ];

    let proposal_id = client.submit_proposal(
        &1,
        &proposer,
        &String::from_str(&env, "Test Proposal"),
        &String::from_str(&env, "Description"),
        &50_000,
        &86400, // 1 day
        &milestones,
    );

    assert_eq!(proposal_id, 1);

    let proposal = client.get_proposal(&proposal_id);
    assert_eq!(proposal.requested_amount, 50_000);
    assert_eq!(proposal.status, ProposalStatus::Pending);
}

#[test]
fn test_voting() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register_contract(None, ProposalContract);
    let client = ProposalContractClient::new(&env, &contract_id);

    let proposer = Address::generate(&env);
    let voter1 = Address::generate(&env);
    let voter2 = Address::generate(&env);

    let milestones = vec![&env, String::from_str(&env, "Milestone 1")];

    let proposal_id = client.submit_proposal(
        &1,
        &proposer,
        &String::from_str(&env, "Test"),
        &String::from_str(&env, "Test"),
        &50_000,
        &86400,
        &milestones,
    );

    client.vote(&proposal_id, &voter1, &true, &100);
    client.vote(&proposal_id, &voter2, &false, &50);

    let proposal = client.get_proposal(&proposal_id);
    assert_eq!(proposal.votes_for, 100);
    assert_eq!(proposal.votes_against, 50);
}

#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, vec, Address, Env, String};

#[test]
fn test_create_and_submit_milestone() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register_contract(None, MilestoneContract);
    let client = MilestoneContractClient::new(&env, &contract_id);

    let descriptions = vec![
        &env,
        String::from_str(&env, "Phase 1"),
        String::from_str(&env, "Phase 2"),
    ];
    let amounts = vec![&env, 25_000, 25_000];

    client.create_milestones(&1, &descriptions, &amounts);

    let milestone_count = client.get_proposal_milestone_count(&1);
    assert_eq!(milestone_count, 2);

    let submitter = Address::generate(&env);
    client.submit_milestone(
        &1,
        &submitter,
        &String::from_str(&env, "https://evidence.com/proof"),
    );

    let milestone = client.get_milestone(&1);
    assert_eq!(milestone.status, MilestoneStatus::Submitted);
}

#[test]
fn test_approve_milestone() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register_contract(None, MilestoneContract);
    let client = MilestoneContractClient::new(&env, &contract_id);

    let descriptions = vec![&env, String::from_str(&env, "Phase 1")];
    let amounts = vec![&env, 50_000];

    client.create_milestones(&1, &descriptions, &amounts);

    let submitter = Address::generate(&env);
    let approver = Address::generate(&env);

    client.submit_milestone(&1, &submitter, &String::from_str(&env, "https://proof.com"));
    client.approve_milestone(&1, &approver);

    let milestone = client.get_milestone(&1);
    assert_eq!(milestone.status, MilestoneStatus::Approved);

    client.mark_paid(&1);
    let milestone_paid = client.get_milestone(&1);
    assert_eq!(milestone_paid.status, MilestoneStatus::Paid);
}

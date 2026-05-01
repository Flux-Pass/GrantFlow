#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_initialize_and_create_pool() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GrantPoolContract);
    let client = GrantPoolContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    client.initialize(&admin);

    let pool_id = client.create_pool(
        &admin,
        &String::from_str(&env, "Community Grants"),
        &String::from_str(&env, "Grants for community projects"),
        &1_000_000,
    );

    assert_eq!(pool_id, 1);

    let pool = client.get_pool(&pool_id);
    assert_eq!(pool.total_funds, 1_000_000);
    assert_eq!(pool.available_funds, 1_000_000);
    assert!(pool.active);
}

#[test]
fn test_allocate_and_return_funds() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GrantPoolContract);
    let client = GrantPoolContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    let pool_id = client.create_pool(
        &admin,
        &String::from_str(&env, "Test Pool"),
        &String::from_str(&env, "Test"),
        &1_000_000,
    );

    let success = client.allocate_funds(&pool_id, &100_000);
    assert!(success);

    let pool = client.get_pool(&pool_id);
    assert_eq!(pool.available_funds, 900_000);

    client.return_funds(&pool_id, &50_000);
    let pool = client.get_pool(&pool_id);
    assert_eq!(pool.available_funds, 950_000);
}

#[test]
fn test_deactivate_pool() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GrantPoolContract);
    let client = GrantPoolContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    let pool_id = client.create_pool(
        &admin,
        &String::from_str(&env, "Test Pool"),
        &String::from_str(&env, "Test"),
        &1_000_000,
    );

    client.deactivate_pool(&admin, &pool_id);

    let pool = client.get_pool(&pool_id);
    assert!(!pool.active);
}

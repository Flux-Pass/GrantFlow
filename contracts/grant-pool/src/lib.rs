#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Pool(u64),
    PoolCount,
    Admin,
}

#[derive(Clone)]
#[contracttype]
pub struct GrantPool {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub total_funds: i128,
    pub available_funds: i128,
    pub admin: Address,
    pub active: bool,
}

#[contract]
pub struct GrantPoolContract;

#[contractimpl]
impl GrantPoolContract {
    /// Initialize the contract with an admin
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::PoolCount, &0u64);
    }

    /// Create a new grant pool
    pub fn create_pool(
        env: Env,
        admin: Address,
        name: String,
        description: String,
        total_funds: i128,
    ) -> u64 {
        admin.require_auth();

        let pool_count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::PoolCount)
            .unwrap_or(0);
        
        let pool_id = pool_count + 1;

        let pool = GrantPool {
            id: pool_id,
            name,
            description,
            total_funds,
            available_funds: total_funds,
            admin: admin.clone(),
            active: true,
        };

        env.storage().instance().set(&DataKey::Pool(pool_id), &pool);
        env.storage().instance().set(&DataKey::PoolCount, &pool_id);

        env.events().publish(
            (String::from_str(&env, "pool_created"),),
            (pool_id, admin),
        );

        pool_id
    }

    /// Get pool details
    pub fn get_pool(env: Env, pool_id: u64) -> GrantPool {
        env.storage()
            .instance()
            .get(&DataKey::Pool(pool_id))
            .unwrap()
    }

    /// Allocate funds from pool (called by proposal contract)
    pub fn allocate_funds(env: Env, pool_id: u64, amount: i128) -> bool {
        let mut pool: GrantPool = env
            .storage()
            .instance()
            .get(&DataKey::Pool(pool_id))
            .unwrap();

        if !pool.active || pool.available_funds < amount {
            return false;
        }

        pool.available_funds -= amount;
        env.storage().instance().set(&DataKey::Pool(pool_id), &pool);

        env.events().publish(
            (String::from_str(&env, "funds_allocated"),),
            (pool_id, amount),
        );

        true
    }

    /// Return funds to pool (if proposal rejected or cancelled)
    pub fn return_funds(env: Env, pool_id: u64, amount: i128) {
        let mut pool: GrantPool = env
            .storage()
            .instance()
            .get(&DataKey::Pool(pool_id))
            .unwrap();

        pool.available_funds += amount;
        env.storage().instance().set(&DataKey::Pool(pool_id), &pool);

        env.events().publish(
            (String::from_str(&env, "funds_returned"),),
            (pool_id, amount),
        );
    }

    /// Deactivate a pool
    pub fn deactivate_pool(env: Env, admin: Address, pool_id: u64) {
        admin.require_auth();

        let mut pool: GrantPool = env
            .storage()
            .instance()
            .get(&DataKey::Pool(pool_id))
            .unwrap();

        if pool.admin != admin {
            panic!("Unauthorized");
        }

        pool.active = false;
        env.storage().instance().set(&DataKey::Pool(pool_id), &pool);

        env.events().publish(
            (String::from_str(&env, "pool_deactivated"),),
            pool_id,
        );
    }

    /// Get all pool IDs
    pub fn get_pool_count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::PoolCount)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test;

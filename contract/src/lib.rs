#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone)]
pub struct BreachRecord {
    pub hash: String,
    pub platform: String,
    pub breach_date: u64,
    pub record_count: u32,
}

#[contract]
pub struct BreachCheckerContract;

#[contractimpl]
impl BreachCheckerContract {
    /// Initialize contract with admin
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&symbol_short!("admin"), &admin);
    }

    /// Add a breach record (admin only)
    pub fn add_breach(
        env: Env,
        hash: String,
        platform: String,
        breach_date: u64,
        record_count: u32,
    ) -> bool {
        let admin: Address = env.storage().instance().get(&symbol_short!("admin")).unwrap();
        admin.require_auth();

        let breach = BreachRecord {
            hash: hash.clone(),
            platform,
            breach_date,
            record_count,
        };

        env.storage().persistent().set(&hash, &breach);
        true
    }

    /// Check if a hash exists in breach database
    pub fn check_breach(env: Env, hash: String) -> bool {
        env.storage().persistent().has(&hash)
    }

    /// Get breach information for a hash
    pub fn get_breach_info(env: Env, hash: String) -> Option<BreachRecord> {
        env.storage().persistent().get(&hash)
    }

    /// Check if data is compromised (alias for check_breach)
    pub fn is_compromised(env: Env, hash: String) -> bool {
        Self::check_breach(env, hash)
    }

    /// Get total breach count (stored separately)
    pub fn get_breach_count(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&symbol_short!("count"))
            .unwrap_or(0)
    }

    /// Increment breach count (admin only)
    pub fn increment_count(env: Env) {
        let admin: Address = env.storage().instance().get(&symbol_short!("admin")).unwrap();
        admin.require_auth();

        let current: u32 = Self::get_breach_count(env.clone());
        env.storage()
            .instance()
            .set(&symbol_short!("count"), &(current + 1));
    }

    /// Get admin address
    pub fn get_admin(env: Env) -> Address {
        env.storage().instance().get(&symbol_short!("admin")).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_breach_checker() {
        let env = Env::default();
        let contract_id = env.register_contract(None, BreachCheckerContract);
        let client = BreachCheckerContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        
        env.mock_all_auths();

        // Initialize
        client.initialize(&admin);

        // Add breach
        let hash = String::from_str(&env, "abc123hash");
        let platform = String::from_str(&env, "TestPlatform");
        let result = client.add_breach(&hash, &platform, &1234567890, &1000);
        assert_eq!(result, true);

        // Check breach
        let is_breached = client.check_breach(&hash);
        assert_eq!(is_breached, true);

        // Get breach info
        let info = client.get_breach_info(&hash);
        assert!(info.is_some());

        // Check non-existent hash
        let safe_hash = String::from_str(&env, "safehash");
        let is_safe = client.check_breach(&safe_hash);
        assert_eq!(is_safe, false);
    }
}

#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub struct Kumbara {
    pub owner: Address,
    pub balance: i128,
    pub goal: i128,
}

#[contract]
pub struct KumbaraContract;

#[contractimpl]
impl KumbaraContract {
    /// Yeni kumbara oluştur
    pub fn initialize(env: Env, owner: Address, goal: i128) -> Kumbara {
        owner.require_auth();
        
        let kumbara = Kumbara {
            owner: owner.clone(),
            balance: 0,
            goal,
        };
        
        env.storage().instance().set(&owner, &kumbara);
        kumbara
    }

    /// Kumbaraya para yatır
    pub fn deposit(env: Env, owner: Address, amount: i128) -> i128 {
        owner.require_auth();
        
        let mut kumbara: Kumbara = env.storage().instance().get(&owner).unwrap();
        kumbara.balance += amount;
        
        env.storage().instance().set(&owner, &kumbara);
        kumbara.balance
    }

    /// Kumbaradan para çek
    pub fn withdraw(env: Env, owner: Address, amount: i128) -> i128 {
        owner.require_auth();
        
        let mut kumbara: Kumbara = env.storage().instance().get(&owner).unwrap();
        
        if kumbara.balance >= amount {
            kumbara.balance -= amount;
            env.storage().instance().set(&owner, &kumbara);
        }
        
        kumbara.balance
    }

    /// Bakiye sorgula
    pub fn get_balance(env: Env, owner: Address) -> i128 {
        let kumbara: Kumbara = env.storage().instance().get(&owner).unwrap_or(Kumbara {
            owner: owner.clone(),
            balance: 0,
            goal: 0,
        });
        kumbara.balance
    }

    /// Hedef sorgula
    pub fn get_goal(env: Env, owner: Address) -> i128 {
        let kumbara: Kumbara = env.storage().instance().get(&owner).unwrap_or(Kumbara {
            owner: owner.clone(),
            balance: 0,
            goal: 0,
        });
        kumbara.goal
    }

    /// Hedefe ulaşıldı mı?
    pub fn is_goal_reached(env: Env, owner: Address) -> bool {
        let kumbara: Kumbara = env.storage().instance().get(&owner).unwrap_or(Kumbara {
            owner: owner.clone(),
            balance: 0,
            goal: 0,
        });
        kumbara.balance >= kumbara.goal
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_kumbara() {
        let env = Env::default();
        let contract_id = env.register_contract(None, KumbaraContract);
        let client = KumbaraContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        
        // Initialize
        env.mock_all_auths();
        let kumbara = client.initialize(&owner, &1000);
        assert_eq!(kumbara.balance, 0);
        assert_eq!(kumbara.goal, 1000);

        // Deposit
        let balance = client.deposit(&owner, &500);
        assert_eq!(balance, 500);

        // Check goal
        let reached = client.is_goal_reached(&owner);
        assert_eq!(reached, false);

        // Deposit more
        client.deposit(&owner, &600);
        let reached = client.is_goal_reached(&owner);
        assert_eq!(reached, true);

        // Withdraw
        let balance = client.withdraw(&owner, &200);
        assert_eq!(balance, 900);
    }
}

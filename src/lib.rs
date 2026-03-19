#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Counter,
}

#[contract]
pub struct CounterContract;

#[contractimpl]
impl CounterContract {
    /// Initialize the counter to 0
    pub fn init(env: Env) {
        let key = DataKey::Counter;
        if !env.storage().instance().has(&key) {
            env.storage().instance().set(&key, &0i32);
            env.storage().instance().extend_ttl(100, 200);
        }
        env.events().publish((symbol_short!("init"),), 0i32);
    }

    /// Increment the counter by 1
    pub fn increment(env: Env) -> i32 {
        let key = DataKey::Counter;
        let val: i32 = env.storage().instance().get(&key).unwrap_or(0);
        let new_val = val + 1;
        env.storage().instance().set(&key, &new_val);
        env.storage().instance().extend_ttl(100, 200);
        env.events().publish((symbol_short!("increment"),), new_val);
        new_val
    }

    /// Decrement the counter by 1
    pub fn decrement(env: Env) -> i32 {
        let key = DataKey::Counter;
        let val: i32 = env.storage().instance().get(&key).unwrap_or(0);
        let new_val = val - 1;
        env.storage().instance().set(&key, &new_val);
        env.storage().instance().extend_ttl(100, 200);
        env.events().publish((symbol_short!("decrement"),), new_val);
        new_val
    }

    /// Get the current counter value
    pub fn value(env: Env) -> i32 {
        let key = DataKey::Counter;
        env.storage().instance().get(&key).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_init() {
        let env = Env::default();
        let contract_id = env.register_contract(None, CounterContract);
        let client = CounterContractClient::new(&env, &contract_id);

        client.init();
        assert_eq!(client.value(), 0);
    }

    #[test]
    fn test_increment() {
        let env = Env::default();
        let contract_id = env.register_contract(None, CounterContract);
        let client = CounterContractClient::new(&env, &contract_id);

        client.init();
        assert_eq!(client.increment(), 1);
        assert_eq!(client.increment(), 2);
        assert_eq!(client.value(), 2);
    }

    #[test]
    fn test_decrement() {
        let env = Env::default();
        let contract_id = env.register_contract(None, CounterContract);
        let client = CounterContractClient::new(&env, &contract_id);

        client.init();
        client.increment();
        assert_eq!(client.decrement(), 0);
    }
}
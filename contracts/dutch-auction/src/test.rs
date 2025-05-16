#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, String};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

/* 
    let words = client.greeting(&String::from_str(&env, "Hello from Toronto"));
    assert_eq!(
        words,
        vec![
            &env,
            String::from_str(&env, ""),
            String::from_str(&env, "Hello from Toronto"),
        ]
    );
*/
}

#![cfg(test)]
use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{vec, Address, Env, String};

#[test]
fn test_init_and_get_candidates() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let candidates = vec![
        &env,
        String::from_str(&env, "Alice"),
        String::from_str(&env, "Bob"),
        String::from_str(&env, "Charlie"),
    ];
    client.init(&candidates);

    let result = client.get_candidates();
    assert_eq!(result.len(), 3);
    assert_eq!(result.get(0).unwrap(), String::from_str(&env, "Alice"));
    assert_eq!(result.get(1).unwrap(), String::from_str(&env, "Bob"));
    assert_eq!(result.get(2).unwrap(), String::from_str(&env, "Charlie"));
}

#[test]
fn test_vote_and_get_votes() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let candidates = vec![
        &env,
        String::from_str(&env, "Alice"),
        String::from_str(&env, "Bob"),
    ];
    client.init(&candidates);

    let voter1 = Address::generate(&env);
    let voter2 = Address::generate(&env);

    client.vote(&voter1, &String::from_str(&env, "Alice"));
    client.vote(&voter2, &String::from_str(&env, "Bob"));

    assert_eq!(client.get_votes(&String::from_str(&env, "Alice")), 1);
    assert_eq!(client.get_votes(&String::from_str(&env, "Bob")), 1);
}

#[test]
fn test_vote_multiple_for_same_candidate() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let candidates = vec![&env, String::from_str(&env, "Alice")];
    client.init(&candidates);

    let voter1 = Address::generate(&env);
    let voter2 = Address::generate(&env);
    let voter3 = Address::generate(&env);

    client.vote(&voter1, &String::from_str(&env, "Alice"));
    client.vote(&voter2, &String::from_str(&env, "Alice"));
    client.vote(&voter3, &String::from_str(&env, "Alice"));

    assert_eq!(client.get_votes(&String::from_str(&env, "Alice")), 3);
}

#[test]
#[should_panic(expected = "already voted")]
fn test_duplicate_vote_panics() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let candidates = vec![
        &env,
        String::from_str(&env, "Alice"),
        String::from_str(&env, "Bob"),
    ];
    client.init(&candidates);

    let voter = Address::generate(&env);

    client.vote(&voter, &String::from_str(&env, "Alice"));
    client.vote(&voter, &String::from_str(&env, "Bob"));
}

#[test]
#[should_panic(expected = "candidate not found")]
fn test_vote_for_invalid_candidate_panics() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let candidates = vec![&env, String::from_str(&env, "Alice")];
    client.init(&candidates);

    let voter = Address::generate(&env);
    client.vote(&voter, &String::from_str(&env, "Dave"));
}

#[test]
fn test_has_voted() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let candidates = vec![&env, String::from_str(&env, "Alice")];
    client.init(&candidates);

    let voter = Address::generate(&env);
    assert!(!client.has_voted(&voter));

    client.vote(&voter, &String::from_str(&env, "Alice"));
    assert!(client.has_voted(&voter));
}

#[test]
fn test_get_votes_uninitialized_candidate_returns_zero() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let candidates = vec![&env, String::from_str(&env, "Alice")];
    client.init(&candidates);

    assert_eq!(client.get_votes(&String::from_str(&env, "Alice")), 0);
}

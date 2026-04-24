#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map, String, Symbol, Vec};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn init(env: Env, candidates: Vec<String>) {
        let mut votes: Map<String, u32> = Map::new(&env);
        for c in candidates.iter() {
            votes.set(c.clone(), 0);
        }
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "candidates"), &candidates);
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "votes"), &votes);
        env.storage().instance().set(
            &Symbol::new(&env, "voters"),
            &Map::<Address, bool>::new(&env),
        );
    }

    pub fn vote(env: Env, voter: Address, candidate: String) {
        voter.require_auth();

        let mut voters: Map<Address, bool> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "voters"))
            .unwrap();
        assert!(!voters.get(voter.clone()).unwrap_or(false), "already voted");

        let mut votes: Map<String, u32> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "votes"))
            .unwrap();
        let candidates: Vec<String> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "candidates"))
            .unwrap();

        assert!(candidates.contains(&candidate), "Invalid candidate");

        let count = votes.get(candidate.clone()).unwrap_or(0);
        votes.set(candidate.clone(), count + 1);

        voters.set(voter, true);

        env.storage()
            .instance()
            .set(&Symbol::new(&env, "votes"), &votes);
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "voters"), &voters);
    }

    pub fn get_votes(env: Env, candidate: String) -> u32 {
        let votes: Map<String, u32> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "votes"))
            .unwrap();
        votes.get(candidate).unwrap_or(0)
    }

    pub fn get_winner(env: Env) -> String {
        let votes: Map<String, u32> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "votes"))
            .unwrap();
        let mut max_votes: u32 = 0;
        let mut winner = String::from_str(&env, "");

        for (candidate, count) in votes.iter() {
            if count > max_votes {
                max_votes = count;
                winner = candidate;
            }
        }
        winner
    }
}

mod test;

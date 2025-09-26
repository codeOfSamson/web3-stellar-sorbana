#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Env, log, String, Vec};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        log!(&env, "to is:", to);

        vec![&env, String::from_str(&env, "Hello Mr."), to]
    
    }
}

mod test;

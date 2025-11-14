#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, symbol_short};

// Storage key for car info
const CAR: Symbol = symbol_short!("CAR");
// Total supply of fractional tokens
const SUPPLY: Symbol = symbol_short!("SUP");

// Structure for a luxury car asset
#[contracttype]
#[derive(Clone)]
pub struct CarAsset {
    pub name: soroban_sdk::String,
    pub description: soroban_sdk::String,
    pub total_tokens: u64,
}

// Contract
#[contract]
pub struct FractionalCarContract;

#[contractimpl]
impl FractionalCarContract {
    /// Register a luxury car and mint fractional tokens
    pub fn register_car(
        env: Env,
        name: soroban_sdk::String,
        description: soroban_sdk::String,
        total_tokens: u64,
    ) {
        let car = CarAsset {
            name,
            description,
            total_tokens,
        };

        env.storage().instance().set(&CAR, &car);
        env.storage().instance().set(&SUPPLY, &total_tokens);
    }

    /// View car info
    pub fn view_car(env: Env) -> CarAsset {
        env.storage()
            .instance()
            .get(&CAR)
            .unwrap_or(CarAsset {
                name: soroban_sdk::String::from_str(&env, "Not Registered"),
                description: soroban_sdk::String::from_str(&env, "No Description"),
                total_tokens: 0,
            })
    }

    /// Get total supply of fractional tokens
    pub fn view_total_supply(env: Env) -> u64 {
        env.storage().instance().get(&SUPPLY).unwrap_or(0)
    }

    /// Transfer fractional tokens (simple demo version)
    pub fn transfer(env: Env, from: Address, to: Address, amount: u64) {
        from.require_auth();

        let from_balance: u64 = env.storage().persistent().get(&from).unwrap_or(0);
        let to_balance: u64 = env.storage().persistent().get(&to).unwrap_or(0);

        if from_balance < amount {
            panic!("Insufficient Balance");
        }

        env.storage().persistent().set(&from, &(from_balance - amount));
        env.storage().persistent().set(&to, &(to_balance + amount));
    }
}

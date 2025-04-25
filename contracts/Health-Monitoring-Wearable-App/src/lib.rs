#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, Symbol, String, symbol_short};

// Structure for storing health record data
#[contracttype]
#[derive(Clone)]
pub struct HealthRecord {
    pub heart_rate: u32,
    pub steps: u32,
    pub timestamp: u64,
}

// Key mapping for health record storage
#[contracttype]
pub enum RecordBook {
    UserRecord(Symbol),
}

#[contract]
pub struct HealthMonitorContract;

#[contractimpl]
impl HealthMonitorContract {
    
    // Store health record (uploaded by wearable device)
    pub fn store_record(env: Env, user_id: Symbol, heart_rate: u32, steps: u32) {
        let record = HealthRecord {
            heart_rate,
            steps,
            timestamp: env.ledger().timestamp(),
        };
        env.storage().instance().set(&RecordBook::UserRecord(user_id), &record);
    }

    // Retrieve latest health record for a user
    pub fn get_record(env: Env, user_id: Symbol) -> HealthRecord {
        env.storage().instance().get(&RecordBook::UserRecord(user_id)).unwrap_or(HealthRecord {
            heart_rate: 0,
            steps: 0,
            timestamp: 0,
        })
    }
}

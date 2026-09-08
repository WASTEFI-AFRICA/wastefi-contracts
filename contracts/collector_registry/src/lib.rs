#![no_std]
use soroban_sdk::{contract, contractimpl};

#[cfg(test)]
mod test;

#[contract]
pub struct CollectorRegistry;

#[contractimpl]
impl CollectorRegistry {
    // Placeholder - will be implemented in Phase 2
}

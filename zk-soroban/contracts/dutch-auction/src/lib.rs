#![no_std]
use core::f32::consts::E;

use soroban_sdk::{
    contract, contractimpl, contracttype, panic_with_error, Address, BytesN, Env, Symbol, U256  
};

use soroban_auth::Identifier;

use tfhe::shortint::backward_compatibility::client_key;
use tfhe::{ConfigBuilder, FheUint64};
use tfhe::prelude::*;

#[contract]
pub struct Contract;

// Data models
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,            // The auction administrator
    TokenId,          // Payment token ID
    ItemId,           // ID of the item being auctioned
    StartTime,        // Auction start time
    EndTime,          // Auction end time
    StartPrice,       // Starting price
    MinimumPrice,     // Minimum price
    CurrentPrice,     // Current price
    HighestBidder,    // Highest bidder
    HighestBid,       // Highest bid
    State,            // Auction state (CREATED, ACTIVE, ENDED)
    PriceDecrement,   // Price decrement per time unit
    Nonce(Identifier),
}


#[derive(Clone, PartialEq)]
#[contracttype]
pub enum AuctionState {
    Created,
    Active,
    Ended,
}

#[derive(Clone)]
#[contracttype]
pub enum Error {
    AlreadyInitialized,
    NotAdmin,
    InvalidState,
    AuctionNotEnded,
    AuctionEnded,
    BidTooLow,
    InsufficientFunds,
}

// ======== Contract Implementation ========
#[contract]
pub struct DutchAuction;

#[contractimpl]
impl DutchAuction {
    pub fn init(
        env: Env,
        admin: Address,
        token_id: BytesN<32>,
        item_id: BytesN<32>,
        start_price: u64,      // Price in smallest units
        minimum_price: u64,    // Minimum price in smallest units
        duration: u64,
        price_decrement: u64,  // Price decrement in smallest units
    ) {
        let NOW: u64 = __contract_fn_get_now(env);
        __contract_fn_set_privacy(env);
        __contract_fn_set_storage(env, admin, token_id, item_id);
        __contract_fn_set_duration(env, NOW);
        __contract_fn_set_auction_config(env, sktart_price, minimum_price, duration, price_decrement);
        __contract_fn_set_state(env);

        log!(&env, "Dutch Auction initialized");
    }
}

pub fn get_current_price(env: Env) -> Result<FheInt, Error> {
    let state: AuctionState = env.storage().instance().get(&DataKey::State).unwrap();
    if state != AuctionState::Active {
        return Err(Error::InvalidState);
    }

    let start_time: u64 = env.storage().instance().get(&DataKey::StartTime).unwrap();
    let now = env.ledger().timestamp();
    let elapsed = now - start_time;

    Ok(encrypted_current_price)
}

fn __contract_fn_set_storage(env: Env, admin: Address, token_id: BytesN<32>, item_id: BytesN<32>) {
    // Requires to be called by the admin only once
    if env.storage().instance().has(&DataKey::Admin) {
        return Err(Error::AlreadyInitialized);
    }

    // Store auction configuration
    env.storage().instance().set(&DataKey::Admin, &admin);
    env.storage().instance().set(&DataKey::TokenId, &token_id);
    env.storage().instance().set(&DataKey::ItemId, &item_id);
}

fn __contract_fn_set_auction_config(env: Env, start_price: u64, minimum_price: u64, duration: u64, price_decrement: u64) {
    
    // Store auction configuration
    env.storage().instance().set(&DataKey::StartPrice, &start_price);
    env.storage().instance().set(&DataKey::MinimumPrice, &minimum_price);
    env.storage().instance().set(&DataKey::Duration, &duration);
    env.storage().instance().set(&DataKey::CurrentPrice, &start_price);
    env.storage().instance().set(&DataKey::PriceDecrement, &price_decrement);
}

fn __contract_fn_set_duration(env: Env, end_time: u64) {
    const END_TIME: u64 = NOW + duration;
    env.storage().instance().set(&DataKey::EndTime, &end_time );
}

fn __contract_fn_get_now(env: Env) -> u64 {
    env.ledger().timestamp()
}

fn __contract_fn_set_state(env: Env) -> u64 {
    env.storage().instance().set(&DataKey::State, &AuctionState::Created);
}


fn __contract_fn_set_privacy(env: Env) -> u64 {
    let config = ConfigBuilder::default().build();
    let (client_key, _) = tfhe::generate_keys(config);

    env.storage().instance().set(&DataKey::ClientKey, &client_key);
}

mod test;

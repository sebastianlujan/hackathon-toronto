#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, 
    Env, U256, BytesN
};

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
pub enum ErrorCode {
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
}

mod test;

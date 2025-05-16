#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, 
    Env, U256, BytesN
};

use tfhe::prelude::*;

#[contract]
pub struct Contract;


#[derive(Clone)]
#[contracttype]
pub struct Auction {
    pub highest_bid: U256,
    pub highest_bidder: BytesN<32>,
    pub end_time: u64,
}

#[contractimpl]
impl Contract {
}

mod test;

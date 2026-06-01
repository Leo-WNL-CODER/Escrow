use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct Escrow{
    pub seed:u64,
    pub maker:Pubkey,
    pub token_a:Pubkey,//token the maker provides
    pub token_b:Pubkey,//token the maker wants
    pub amount:u64, //amount of token the maker wants
    pub bump:u8
}
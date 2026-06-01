use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError{
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid Maker")]
    InvalidMaker,
    #[msg("Invalid Token A")]
    InvalidtokenA,
    #[msg("Invalid Token B")]
    InvalidtokenB
}
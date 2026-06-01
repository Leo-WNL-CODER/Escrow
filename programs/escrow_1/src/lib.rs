pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("EP9h7W6fD5nhFFRsRtd2QYEbXoegYxcAVyZBAMB3M8KU");

#[program]
pub mod escrow_1 {
    use super::*;

    pub fn make(ctx: Context<Maker>, seed: u64, receive: u64, amount: u64) -> Result<()> {
        instructions::make::handler(ctx, seed, amount, receive)
    }
    pub fn take(ctx: Context<Taker>) -> Result<()> {
        instructions::take::handler(ctx)
    }
}

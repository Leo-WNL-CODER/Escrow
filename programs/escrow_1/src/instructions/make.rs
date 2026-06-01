use std::rc;

use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{TransferChecked,transfer_checked,Mint, TokenAccount, TokenInterface}};

use crate::{error::EscrowError, state::Escrow};
// use anchor_lang::accounts::account;

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Maker<'info>{
    #[account(mut)]
    pub maker:Signer<'info>,

    #[account(
    init,
    payer=maker,
    space=8+Escrow::INIT_SPACE,
    seeds=[b"escrow",maker.key().as_ref(),seed.to_le_bytes().as_ref()],
    bump
    )]
    pub escrow_acc:Account<'info,Escrow>,

    //token_Acc
    #[account(mint::token_program=token_program)]
    pub token_a:InterfaceAccount<'info,Mint>,
    
    #[account(mint::token_program=token_program)]
    pub token_b:InterfaceAccount<'info,Mint>,
    #[account(mut,
    associated_token::mint=token_a,
    associated_token::authority=maker,
    associated_token::token_program=token_program
    )]
    pub maker_ata_a:InterfaceAccount<'info,TokenAccount>,

    #[account(
        init,
        payer=maker,
        associated_token::mint = token_a,
        associated_token::authority=escrow_acc,
        associated_token::token_program=token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
   
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Maker<'info>{
    pub fn populate_escrow(&mut self,seed:u64,amount:u64,bump:u8)->Result<()>{
        self.escrow_acc.set_inner(Escrow { seed, 
            maker: self.maker.key(), 
            token_a: self.token_a.key(), 
            token_b: self.token_b.key(), 
            amount, 
            bump });
        Ok(())
    } 

    pub fn deposit_to_vault(&mut self,amount:u64)->Result<()>{
        let cpi_context=CpiContext::new(
            self.token_program.key(),
            TransferChecked{
                from:self.maker_ata_a.to_account_info(),
                mint:self.token_a.to_account_info(),
                to:self.vault.to_account_info(),
                authority:self.maker.to_account_info()
            });
        transfer_checked(cpi_context, 
            amount,
            self.token_a.decimals
        )?;

        Ok(())
    }
}

pub fn handler(ctx:Context<Maker>,seed:u64,amount:u64,recieve:u64)->Result<()>{
    require_gt!(recieve,0,EscrowError::InvalidAmount);
    require_gt!(amount,0,EscrowError::InvalidAmount);
    
    ctx.accounts.populate_escrow(seed, recieve, ctx.bumps.escrow_acc)?;
    ctx.accounts.deposit_to_vault(amount)?;
    Ok(())
}
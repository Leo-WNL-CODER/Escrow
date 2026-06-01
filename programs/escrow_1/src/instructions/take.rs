use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{CloseAccount, TransferChecked, close_account, transfer_checked}, token_interface::{Mint ,TokenAccount,TokenInterface}};

// use crate::error::EscrowError;

use crate::{error::EscrowError, state::Escrow};

#[derive(Accounts)]
pub struct Taker<'info>{
    #[account(mut)]
    pub taker:Signer<'info>,
    #[account(mut)]
    pub maker:SystemAccount<'info>,

    #[account(mut,
        close=maker,
        seeds=[b"escrow",maker.key().as_ref(),escrow_acc.seed.to_le_bytes().as_ref()],
        bump=escrow_acc.bump,
        has_one=maker @ EscrowError::InvalidMaker,
        has_one=token_a @ EscrowError::InvalidtokenA,
        has_one=token_b @ EscrowError::InvalidtokenB,
    )]
    pub escrow_acc:Account<'info,Escrow>,

    pub token_a:Box<InterfaceAccount<'info,Mint>>,
    pub token_b:Box<InterfaceAccount<'info,Mint>>,
    
    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=token_a,
        associated_token::authority=taker,
        associated_token::token_program=token_program
    )]
    pub taker_ata_a:Box<InterfaceAccount<'info,TokenAccount>>,

    #[account(
        mut,
        associated_token::mint=token_b,
        associated_token::authority=taker,
        associated_token::token_program=token_program
    )]
    pub taker_ata_b:Box<InterfaceAccount<'info,TokenAccount>>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=token_b,
        associated_token::authority=maker,
        associated_token::token_program=token_program
    )]
    pub maker_ata_b:Box<InterfaceAccount<'info,TokenAccount>>,

    #[account(
        mut,
        associated_token::mint=token_a,
        associated_token::token_program=token_program,
        associated_token::authority=escrow_acc
    )]
    pub vault:Box<InterfaceAccount<'info,TokenAccount>>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub token_program:Interface<'info,TokenInterface>,
    pub system_program:Program<'info,System>
}   

impl <'info> Taker <'info>{
    pub fn send_to_maker(&mut self)->Result<()>{

        let cpi_context=CpiContext::new(self.token_program.key()
        , TransferChecked{
            from:self.taker_ata_b.to_account_info(),
            mint:self.token_b.to_account_info(),
            to:self.maker_ata_b.to_account_info(),
            authority:self.taker.to_account_info()
        });
        
        transfer_checked(cpi_context, 
            self.escrow_acc.amount, 
            self.token_b.decimals)?;
        Ok(())
    }

    pub fn withdraw_and_closing_escrow_account(&mut self)->Result<()>{

        let maker_key = self.maker.key();
        let seed_bytes = self.escrow_acc.seed.to_le_bytes();
        let bump = [self.escrow_acc.bump];

        let signer: [&[&[u8]]; 1]=[&[b"escrow",
        maker_key.as_ref(),
        &seed_bytes,
        &bump
        ]];

        let cpi_ctx=CpiContext::new_with_signer(
            self.token_program.key(), TransferChecked{
                from:self.vault.to_account_info(),
                to:self.taker_ata_a.to_account_info(),
                mint:self.token_a.to_account_info(),
                authority:self.escrow_acc.to_account_info()
            },
            &signer
        );

        transfer_checked(cpi_ctx,
            self.vault.amount, 
            self.token_a.decimals)?;

        close_account(
            CpiContext::new_with_signer(
                self.token_program.key()
                ,
                CloseAccount{
                    account:self.vault.to_account_info(),
                    destination:self.maker.to_account_info(),
                    authority:self.escrow_acc.to_account_info()
                }, &signer))?;
        
        Ok(())
    }
}

pub fn handler(ctx: Context<Taker>) -> Result<()> {
    ctx.accounts.send_to_maker()?;
    ctx.accounts.withdraw_and_closing_escrow_account()?;
    Ok(())
}
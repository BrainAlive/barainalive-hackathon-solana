use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("4YWZEUu6C3E5keb5dAvLmxLPrYnV7m7FATHVtHFCMiAw");

#[program]
pub mod focii_verifier {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let verifier = &mut ctx.accounts.verifier;
        verifier.authority = ctx.accounts.authority.key();
        verifier.reward_token_mint = ctx.accounts.reward_token_mint.key();
        verifier.node_license_mint = ctx.accounts.node_license_mint.key();
        Ok(())
    }

    pub fn verify_and_mint(
        ctx: Context<VerifyAndMint>,
        verification_data: Vec<u8>,
        amount: u64,
    ) -> Result<()> {
        // Verify the provided data
        if !verify_focii_data(&verification_data) {
            return err!(FociiVerifierError::VerificationFailed);
        }

        // Verify node license ownership
        require!(
            ctx.accounts.node_license.owner == ctx.accounts.node_owner.key(),
            FociiVerifierError::InvalidNodeLicense
        );

        // Mint reward tokens
        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.reward_token_mint.to_account_info(),
            to: ctx.accounts.rewards_account.to_account_info(),
            authority: ctx.accounts.verifier.to_account_info(),
        };

        let seeds = &[
            b"verifier".as_ref(),
            &[ctx.accounts.verifier.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer,
        );

        token::mint_to(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 32 + 1,
        seeds = [b"verifier"],
        bump
    )]
    pub verifier: Account<'info, Verifier>,
    pub reward_token_mint: Account<'info, Mint>,
    pub node_license_mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyAndMint<'info> {
    #[account(
        seeds = [b"verifier"],
        bump = verifier.bump,
        has_one = reward_token_mint,
        has_one = node_license_mint,
    )]
    pub verifier: Account<'info, Verifier>,
    #[account(mut)]
    pub reward_token_mint: Account<'info, Mint>,
    pub node_license_mint: Account<'info, Mint>,
    #[account(mut)]
    pub node_license: Account<'info, TokenAccount>,
    #[account(mut)]
    pub node_owner: Signer<'info>,
    #[account(mut)]
    pub rewards_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Verifier {
    pub authority: Pubkey,
    pub reward_token_mint: Pubkey,
    pub node_license_mint: Pubkey,
    pub bump: u8,
}

#[error_code]
pub enum FociiVerifierError {
    #[msg("Verification failed")]
    VerificationFailed,
    #[msg("Invalid node license")]
    InvalidNodeLicense,
}

fn verify_focii_data(data: &[u8]) -> bool {
    // Implement your verification logic here
    true
} 
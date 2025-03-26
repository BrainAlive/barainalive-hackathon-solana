use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;

declare_id!("FdeBMZ1iCrpj6hmLTXVa3gkbWutjXRLyn8EYebUAZnP5");

#[program]
pub mod baai_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Initialize state
        let state = &mut ctx.accounts.state;
        state.authority = ctx.accounts.authority.key();
        state.reward_token_mint = ctx.accounts.reward_token_mint.key();
        state.node_license_mint = ctx.accounts.node_license_mint.key();
        state.total_nodes = 0;
        Ok(())
    }

    pub fn create_node_license(
        ctx: Context<CreateNodeLicense>,
        node_id: String,
    ) -> Result<()> {
        // Mint NFT representing node license
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.node_license_mint.to_account_info(),
                    to: ctx.accounts.node_token_account.to_account_info(),
                    authority: ctx.accounts.state.to_account_info(),
                },
            ),
            1,
        )?;

        // Create node metadata
        let node = &mut ctx.accounts.node;
        node.owner = ctx.accounts.owner.key();
        node.node_id = node_id;
        node.rewards_earned = 0;
        node.verification_count = 0;
        node.license_token = ctx.accounts.node_token_account.key();

        // Update state
        let state = &mut ctx.accounts.state;
        state.total_nodes = state.total_nodes.checked_add(1).unwrap();

        Ok(())
    }

    pub fn distribute_rewards(
        ctx: Context<DistributeRewards>,
        amount: u64,
    ) -> Result<()> {
        require!(
            ctx.accounts.node.verification_count > 0,
            BaaiError::NoVerifications
        );

        // Mint reward tokens
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.reward_token_mint.to_account_info(),
                    to: ctx.accounts.rewards_account.to_account_info(),
                    authority: ctx.accounts.state.to_account_info(),
                },
            ),
            amount,
        )?;

        // Update node stats
        let node = &mut ctx.accounts.node;
        node.rewards_earned = node.rewards_earned.checked_add(amount).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        mint::decimals = 9,
        mint::authority = state,
    )]
    pub reward_token_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 0,
        mint::authority = state,
    )]
    pub node_license_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 32 + 8,
    )]
    pub state: Account<'info, ProgramState>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CreateNodeLicense<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 32 + STRING_LENGTH + 8 + 8 + 32,
    )]
    pub node: Account<'info, Node>,

    #[account(mut)]
    pub node_license_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = node_license_mint,
        associated_token::authority = owner,
    )]
    pub node_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub state: Account<'info, ProgramState>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(mut)]
    pub node: Account<'info, Node>,

    #[account(mut)]
    pub reward_token_mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = reward_token_mint,
        token::authority = node.owner,
    )]
    pub rewards_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub state: Account<'info, ProgramState>,

    pub token_program: Program<'info, Token>,
}

#[account]
pub struct ProgramState {
    pub authority: Pubkey,
    pub reward_token_mint: Pubkey,
    pub node_license_mint: Pubkey,
    pub total_nodes: u64,
}

#[account]
pub struct Node {
    pub owner: Pubkey,
    pub node_id: String,
    pub rewards_earned: u64,
    pub verification_count: u64,
    pub license_token: Pubkey,
}

const STRING_LENGTH: usize = 32;

#[error_code]
pub enum BaaiError {
    #[msg("Node has no verifications")]
    NoVerifications,
} 
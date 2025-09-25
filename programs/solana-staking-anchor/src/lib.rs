use anchor_lang::prelude::*;

declare_id!("Stak111111111111111111111111111111111111111");

#[program]
pub mod solana_staking_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.admin = *ctx.accounts.admin.key;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        require!(amount > 0, CustomError::InvalidAmount);
        let user = &mut ctx.accounts.user;
        user.staked += amount;
        Ok(())
    }

    pub fn unstake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        require!(amount > 0 && amount <= ctx.accounts.user.staked, CustomError::InvalidAmount);
        let user = &mut ctx.accounts.user;
        user.staked -= amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin, space = 8 + 32)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Account<'info, UserStake>,
}

#[account]
pub struct State {
    pub admin: Pubkey,
}

#[account]
pub struct UserStake {
    pub staked: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("invalid amount")]
    InvalidAmount,
}

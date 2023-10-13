use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{
    state::{Pool},
    constants::LIQUIDITY_SEED,
};

use solana_program::entrypoint::ProgramResult;
use spl_token::state::AccountState;

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(
        init,
        payer = payer,
        space = Pool::LEN,
        seeds = [
            pool.key().as_ref(),
            authority.to_account_info().key.as_ref(),
            LIQUIDITY_SEED.as_ref(),
        ],
        bump,
        //constraint = pool.mint.decimals == 6
    )]
    pub pool: Account<'info, Pool>,

    #[account(init, payer = payer, space = TokenAccount::LEN)]
    pub pool_account: Box<Account<'info, TokenAccount>>,

    /// The account paying for all rents
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The authority of the pool
    pub authority: AccountInfo<'info>,

    /// Solana ecosystem accounts
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn create_pool(
	ctx: Context<CreatePool>,
	pool_name: String,
	pool_symbol: String,
	pool_uri: String,
) -> ProgramResult {
	let pool = &mut ctx.accounts.pool;
	let pool_account = &mut *ctx.accounts.pool_account;

	// Set pool metadata
	pool.name = pool_name;
	pool.symbol = pool_symbol;
	pool.uri = pool_uri;

	Ok(())
}

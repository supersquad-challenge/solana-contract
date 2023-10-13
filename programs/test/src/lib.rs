#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod constants;

use instructions::*;

declare_id!("ei1j5p9iBPLfjR5GHsZ1Yd96peCk4F6x2hrFRu7iZDi");

#[program]
pub mod transfer_tokens {
    use super::*;

    pub fn create_token(
        ctx: Context<CreateToken>,
        token_title: String,
        token_symbol: String,
        token_uri: String,
    ) -> Result<()> {
        create::create_token(ctx, token_title, token_symbol, token_uri)
    }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        mint::mint_token(ctx, amount)
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        transfer::transfer_tokens(ctx, amount)
    }
}

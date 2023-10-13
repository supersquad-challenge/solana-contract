// FILEPATH: /home/sjlee80/test/anchor-test/test/programs/test/src/state.rs
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct TokenAccount {
    /// The mint associated with this account
    pub mint: Pubkey,

    /// The owner of this account.
    pub owner: Pubkey,

    /// The amount of tokens this account holds.
    pub amount: u64,

    /// The delegate assigned to this account.
    pub delegate: Option<Pubkey>,

    /// The state of a multisignature account.
    pub is_initialized: bool,

    /// The amount of tokens the delegate is authorized to spend.
    pub delegate_amount: u64,
}

impl TokenAccount {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 1 + 8;
}

#[account]
#[derive(Default)]
pub struct Pool {
    /// Primary key of the AMM
    pub amm: Pubkey,

    /// The mint associated with this pool
    pub mint: Pubkey,

    /// Pool metadata
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

impl Pool {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 32 + 32 + 32;
}
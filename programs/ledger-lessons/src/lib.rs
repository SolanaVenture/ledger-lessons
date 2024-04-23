use anchor_lang::prelude::*;

declare_id!("9XKM2fDpipiqeYSNFGi43cVuS888EpsVbhqYJmymiDCD");

#[program]
pub mod ledger_lessons {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct EscrowAccount {
    pub deposits: Vec<(Pubkey, u64)>, // student list
    pub course: Pubkey,
}

#[account]
pub struct Course {
    pub name: String,
    pub description: String,
    pub total_hours: u64,
    pub deposit_amount: u64,
    pub students: Vec<Pubkey>,
    pub teacher: Pubkey,
}

#[account]
pub struct Student {
    pub key: Pubkey,
    pub attended_hours: u64,
    pub course: Pubkey,
}

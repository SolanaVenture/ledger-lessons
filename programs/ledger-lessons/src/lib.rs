use std::mem;
use anchor_lang::prelude::*;

declare_id!("9XKM2fDpipiqeYSNFGi43cVuS888EpsVbhqYJmymiDCD");

#[program]
pub mod ledger_lessons {
    use super::*;

    // Initialize the course and set up the corresponding escrow account
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        // TODO
        Ok(())
    }

    // The students register to join the course
    pub fn enroll(_ctx: Context<Enroll>, _deposit_amount: u64) -> Result<()> {
        // TODO
        Ok(())
    }

    // The teacher? records student attendance
    pub fn mark_attendance(
        _ctx: Context<MarkAttendance>,
        _student_accounts: Vec<Pubkey>,
    ) -> Result<()> {
        // TODO: Update the class time for the attending students
    Ok(())
}

    // Close the course and handle the deposit based on attendance rate
    pub fn finish_course(_ctx: Context<FinishCourse>) -> Result<()> {
        // TODO: Iterate through students and handle deposits
    Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        seeds = [b"course", signer.key().as_ref()],
        bump,
        payer = signer, 
        space = 8 + mem::size_of::<CourseAccount>() + 8,
    )]
    pub course: Account<'info, CourseAccount>,

    #[account(
        init,
        seeds = [b"escrow", course.key().as_ref()],
        bump, 
        payer = signer, 
        space = 8 + mem::size_of::<EscrowAccount>() + 8
    )]
    pub escrow: Account<'info, EscrowAccount>,
    
    #[account(mut)]
    pub signer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Enroll {
    // TODO
}

#[derive(Accounts)]
pub struct MarkAttendance {
    // TODO
}

#[derive(Accounts)]
pub struct FinishCourse {
    // TODO
}


#[account]
pub struct EscrowAccount {
    pub deposits: Vec<(Pubkey, u64)>, // student list
    pub course: Pubkey,
}

#[account]
pub struct CourseAccount {
    pub name: String,
    pub description: String,
    pub total_hours: u64,
    pub deposit_amount: u64, // The deposit that each student should pay
    pub students: Vec<Pubkey>,
    pub teacher: Pubkey,
}

#[account]
pub struct StudentAccount {
    pub student: Pubkey,
    pub attended_hours: u64,
    pub course: Pubkey,
    pub refunded: bool, // Has the deposit been refunded
}

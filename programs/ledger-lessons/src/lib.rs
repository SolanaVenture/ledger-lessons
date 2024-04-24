use anchor_lang::prelude::*;
use std::mem;

declare_id!("9XKM2fDpipiqeYSNFGi43cVuS888EpsVbhqYJmymiDCD");

#[program]
pub mod ledger_lessons {
    use anchor_lang::solana_program::system_instruction;

    use super::*;

    // Initialize the course and set up the corresponding escrow account
    pub fn initialize(
        ctx: Context<Initialize>,
        name: String,
        description: String,
        total_hours: u64,
        deposit_amount: u64,
    ) -> Result<()> {
        let course = &mut ctx.accounts.course;
        course.name = name;
        course.description = description;
        course.total_hours = total_hours;
        course.deposit_amount = deposit_amount;

        let escrow = &mut ctx.accounts.escrow;
        escrow.deposits = Vec::new();
        escrow.course = course.key();

        let course_registry = &mut ctx.accounts.course_registry;
        course_registry.course_accounts.push(course.key());

        Ok(())
    }

    // The students register to join the course
    pub fn enroll(ctx: Context<Enroll>, _deposit_amount: u64) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        let student = &mut ctx.accounts.student;
        let course = &ctx.accounts.course;

        let deposit_amount = course.deposit_amount; //lamports

        let student_balance = student.to_account_info().lamports();
        require!(
            student_balance > deposit_amount,
            LedgerClassError::InsufficientFunds
        );

        // Transfer the deposit from the student account to the escrow account.
        // TODO: need to confirm it !
        let transfer_instruction = system_instruction::transfer(
            &student.to_account_info().key(),
            &escrow.to_account_info().key(),
            deposit_amount,
        );
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                student.to_account_info(),
                escrow.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;

        escrow.deposits.push((student.key(), deposit_amount));

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

    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + std::mem::size_of::<CourseRegistry>() + 8,
        seeds = [b"course_registry"],
        bump
    )]
    pub course_registry: Account<'info, CourseRegistry>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Enroll<'info> {
    #[account(mut)]
    pub student: Account<'info, StudentAccount>,

    #[account(mut)]
    pub escrow: Account<'info, EscrowAccount>,

    #[account(mut)]
    pub course: Account<'info, CourseAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum LedgerClassError {
    #[msg("The balance of the student account should be greater than the deposit amount for that course.")]
    InsufficientFunds,
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
pub struct CourseRegistry {
    pub course_accounts: Vec<Pubkey>, // Store all pub keys of courses
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
    pub deposit_amount: u64, // The deposit that each student should pay (lamports)
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

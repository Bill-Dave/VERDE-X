use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod verdex_escrow {
    use super::*;

    pub fn create_escrow(ctx: Context<CreateEscrow>, amount: u64) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.amount = amount;
        escrow.initiator = *ctx.accounts.initiator.key;
        escrow.counterparty = *ctx.accounts.counterparty.key;
        escrow.state = EscrowState::Pending;
        Ok(())
    }

    pub fn fund_escrow(ctx: Context<FundEscrow>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        require!(escrow.state == EscrowState::Pending, ErrorCode::InvalidState);
        escrow.state = EscrowState::Funded;
        Ok(())
    }

    pub fn release_escrow(ctx: Context<ReleaseEscrow>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        require!(escrow.state == EscrowState::Funded, ErrorCode::InvalidState);
        escrow.state = EscrowState::Released;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(init, payer = initiator, space = 8 + 120)]
    pub escrow: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub initiator: Signer<'info>,
    /// CHECK: counterparty pubkey only
    pub counterparty: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FundEscrow<'info> {
    #[account(mut)]
    pub escrow: Account<'info, EscrowAccount>,
    pub funder: Signer<'info>,
}

#[derive(Accounts)]
pub struct ReleaseEscrow<'info> {
    #[account(mut)]
    pub escrow: Account<'info, EscrowAccount>,
    pub releaser: Signer<'info>,
}

#[account]
pub struct EscrowAccount {
    pub amount: u64,
    pub initiator: Pubkey,
    pub counterparty: Pubkey,
    pub state: EscrowState,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EscrowState {
    Pending,
    Funded,
    Released,
}

#[error_code]
pub enum ErrorCode {
    InvalidState,
}

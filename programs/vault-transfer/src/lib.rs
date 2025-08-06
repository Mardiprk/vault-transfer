use anchor_lang::prelude::*;

declare_id!("94vd5bjzHPXHnN6w6s7gNrmrr5JwF1jSgVjzyZNRPTYs");

#[program]
pub mod vault_transfer {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vaults = &mut ctx.accounts.vault;

        vaults.set_admin(ctx.accounts.admin.key())?;
        vaults.set_bump(ctx.bumps.vault);

        msg!("Vault initialized with admin: {}", vaults.admin);

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::InvalidAmount);
        require!(amount <= 1_000_000, VaultError::ExcessiveAmount);

        let vault = &mut ctx.accounts.vault;

        vault.add_balance(amount)?;

        msg!(
            "Deposited {} tokens. New balance: {}",
            amount,
            vault.balance
        );

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let admin_key = ctx.accounts.vault.admin;
        let vault = &mut ctx.accounts.vault;
        vault.check_admin(admin_key)?;

        require!(amount > 0, VaultError::InvalidAmount);
        require!(vault.balance >= amount, VaultError::InsufficientFunds);

        vault.subtract_balance(amount)?;

        msg!(
            "Withdrawn {} tokens. Remaining balance: {}",
            amount,
            vault.balance
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = admin,
        space = Vault::INIT_SPACE,
        seeds = [b"vault", admin.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"vault", vault.admin.as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,

    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"vault", vault.admin.as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,

    pub user: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub admin: Pubkey,
    pub balance: u64,
    pub bump: u8,
}

impl Vault {
    pub fn set_admin(&mut self, admin: Pubkey) -> Result<()> {
        require!(admin != Pubkey::default(), VaultError::InvalidAdmin);
        self.admin = admin;
        Ok(())
    }

    pub fn set_bump(&mut self, bump: u8) -> Result<()> {
        self.bump = bump;
        Ok(())
    }

    pub fn add_balance(&mut self, amount: u64) -> Result<()> {
        self.balance = self
            .balance
            .checked_add(amount)
            .ok_or(VaultError::MathOverflow)?;
        Ok(())
    }

    pub fn subtract_balance(&mut self, amount: u64) -> Result<()> {
        self.balance = self
            .balance
            .checked_sub(amount)
            .ok_or(VaultError::InsufficientFunds)?;
        Ok(())
    }

    pub fn check_admin(&self, admin: Pubkey) -> Result<()> {
        require!(self.admin == admin, VaultError::UnauthorizedAdmin);
        Ok(())
    }
}
#[error_code]
pub enum VaultError {
    #[msg("Amount must be greater than 0")]
    InvalidAmount,

    #[msg("Amount exceeds maximum allowed")]
    ExcessiveAmount,

    #[msg("Insufficient funds in vault")]
    InsufficientFunds,

    #[msg("Invalid admin address")]
    InvalidAdmin,

    #[msg("Unauthorized: Only admin can perform this action")]
    UnauthorizedAdmin,

    #[msg("Mathematical operation resulted in overflow")]
    MathOverflow,
}

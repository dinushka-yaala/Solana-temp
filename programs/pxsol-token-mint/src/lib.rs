#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, Token2022, InitializeMint2};

pub mod constants;
use constants::*;
// pub mod instructions;
// use instructions::*;

declare_id!("8H5aRtSrNcF3TbrfPpV7UUYa57HKBy4syKqvADvHhiSk");

#[program]
pub mod pxsol_token_mint {
    use super::*;

    pub fn init_pxsol(ctx: Context<InitializePxSOL>) -> Result<()> {
        if ctx.accounts.mint_pxsol.to_account_info().data_is_empty() {
            token_interface::initialize_mint2(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    InitializeMint2 {
                        mint: ctx.accounts.mint_pxsol.to_account_info(),
                    },
                ),
                MINT_DECIMALS, // Token decimals
                &ctx.accounts.admin.key(), // Mint authority
                Some(&ctx.accounts.admin.key()), // freeze authority
            )?;
        }

        // Set token metadata (this part requires token-2022 extension support)
        let _metadata = format!("pxSOL - Liquid Staking Token");

        Ok(())
    }

    pub fn init_apxsol(ctx: Context<InitializeApxSOL>) -> Result<()> {
        if ctx.accounts.mint_apxsol.to_account_info().data_is_empty() {
            token_interface::initialize_mint2(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    InitializeMint2 {
                        mint: ctx.accounts.mint_apxsol.to_account_info(),
                    },
                ),
                MINT_DECIMALS,
                &ctx.accounts.admin.key(),
                Some(&ctx.accounts.admin.key()),
            )?;
        }

        let _metadata = format!("apxSOL - Liquid Staking Token");

        Ok(())
    }

    pub fn init_upxsol(ctx: Context<InitializeUpxSOL>) -> Result<()> {
        if ctx.accounts.mint_upxsol.to_account_info().data_is_empty() {
            token_interface::initialize_mint2(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    InitializeMint2 {
                        mint: ctx.accounts.mint_upxsol.to_account_info(),
                    },
                ),
                MINT_DECIMALS,
                &ctx.accounts.admin.key(),
                Some(&ctx.accounts.admin.key()),
            )?;
        }

        let _metadata = format!("upxSOL - Liquid Staking Token");

        Ok(())
    }

    pub fn init_ipxsol(ctx: Context<InitializeIpxSOL>) -> Result<()> {
        if ctx.accounts.mint_ipxsol.to_account_info().data_is_empty() {
            token_interface::initialize_mint2(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    InitializeMint2 {
                        mint: ctx.accounts.mint_ipxsol.to_account_info(),
                    },
                ),
                MINT_DECIMALS,
                &ctx.accounts.admin.key(),
                Some(&ctx.accounts.admin.key()),
            )?;
        }

        let _metadata = format!("ipxSOL - Liquid Staking Token");

        Ok(())
    }

    pub fn init_iapxsol(ctx: Context<InitializeIapxSOL>) -> Result<()> {
        if ctx.accounts.mint_iapxsol.to_account_info().data_is_empty() {
            token_interface::initialize_mint2(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    InitializeMint2 {
                        mint: ctx.accounts.mint_iapxsol.to_account_info(),
                    },
                ),
                MINT_DECIMALS,
                &ctx.accounts.admin.key(),
                Some(&ctx.accounts.admin.key()),
            )?;
        }

        let _metadata = format!("iapxSOL - Liquid Staking Token");

        Ok(())
    }

    pub fn init_iupxsol(ctx: Context<InitializeIupxSOL>) -> Result<()> {
        if ctx.accounts.mint_iupxsol.to_account_info().data_is_empty() {
            token_interface::initialize_mint2(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    InitializeMint2 {
                        mint: ctx.accounts.mint_iupxsol.to_account_info(),
                    },
                ),
                MINT_DECIMALS,
                &ctx.accounts.admin.key(),
                Some(&ctx.accounts.admin.key()),
            )?;
        }

        let _metadata = format!("iupxSOL - Liquid Staking Token");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePxSOL<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(
        init_if_needed,
        payer = admin,
        mint::decimals = MINT_DECIMALS,
        mint::authority = admin.key(),
        mint::freeze_authority = admin.key(),
        seeds = [PX_SOL_TOKEN_MINT.as_bytes()],
        bump
    )]
    pub mint_pxsol: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct InitializeApxSOL<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init_if_needed,
        payer = admin,
        mint::decimals = MINT_DECIMALS,
        mint::authority = admin.key(),
        mint::freeze_authority = admin.key(),
        seeds = [APX_SOL_TOKEN_MINT.as_bytes()],
        bump
    )]
    pub mint_apxsol: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct InitializeUpxSOL<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init_if_needed,
        payer = admin,
        mint::decimals = MINT_DECIMALS,
        mint::authority = admin.key(),
        mint::freeze_authority = admin.key(),
        seeds = [UPX_SOL_TOKEN_MINT.as_bytes()],
        bump
    )]
    pub mint_upxsol: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct InitializeIpxSOL<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init_if_needed,
        payer = admin,
        mint::decimals = MINT_DECIMALS,
        mint::authority = admin.key(),
        mint::freeze_authority = admin.key(),
        seeds = [IPX_SOL_TOKEN_MINT.as_bytes()],
        bump
    )]
    pub mint_ipxsol: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct InitializeIapxSOL<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init_if_needed,
        payer = admin,
        mint::decimals = MINT_DECIMALS,
        mint::authority = admin.key(),
        mint::freeze_authority = admin.key(),
        seeds = [IAPX_SOL_TOKEN_MINT.as_bytes()],
        bump
    )]
    pub mint_iapxsol: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct InitializeIupxSOL<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init_if_needed,
        payer = admin,
        mint::decimals = MINT_DECIMALS,
        mint::authority = admin.key(),
        mint::freeze_authority = admin.key(),
        seeds = [IUPX_SOL_TOKEN_MINT.as_bytes()],
        bump
    )]
    pub mint_iupxsol: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

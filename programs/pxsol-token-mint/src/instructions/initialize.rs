use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
        Mint,
        Token2022,
    };

use crate::constants::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
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

pub fn handle(ctx: Context<Initialize>) -> Result<()> {
    let mints: Vec<(&str, &InterfaceAccount<Mint>)> = vec![
        (PX_SOL_TOKEN_MINT, &ctx.accounts.mint_pxsol),
        (APX_SOL_TOKEN_MINT, &ctx.accounts.mint_apxsol),
        (UPX_SOL_TOKEN_MINT, &ctx.accounts.mint_upxsol),
        (IPX_SOL_TOKEN_MINT, &ctx.accounts.mint_ipxsol),
        (IAPX_SOL_TOKEN_MINT, &ctx.accounts.mint_iapxsol),
        (IUPX_SOL_TOKEN_MINT, &ctx.accounts.mint_iupxsol),
    ];

    msg!("Initializing Mint Accounts: {:#?}", mints);
    Ok(())
}
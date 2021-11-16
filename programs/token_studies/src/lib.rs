use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{self, SetAuthority, Token, TokenAccount, Transfer, Mint}};


declare_id!("6Txq6B3otMq2dg3Z7RA8ewqayV795CRSeCujq6JrmdSg");

static PROGRAMSEED: &[u8] = b"programauthority";
static POOLSEED: &[u8] = b"pool";
static TOKENSEED: &[u8] = b"token";
static VOUCHERSEED: &[u8] = b"voucher";

#[program]
pub mod token_studies {

    use super::*;
    pub fn initialize_authority(
        ctx: Context<InitializeAuthority>,  
        program_authority_bump: u8) 
        -> ProgramResult {
        
            let program_authority = &mut ctx.accounts.state;
            program_authority.bump = program_authority_bump;
            program_authority.authority = ctx.accounts.admin.key();
        
            Ok(())
    }

    pub fn add_pool(
        ctx: Context<AddPool>,
        pool_bump: u8)
    -> ProgramResult {
        ctx.accounts.pool.bump = pool_bump;
        ctx.accounts.pool.token_mint_address = ctx.accounts.token_mint.key();
        ctx.accounts.pool.token_account_address = ctx.accounts.pool_token.key();
        ctx.accounts.pool.voucher_mint_address = ctx.accounts.voucher_mint.key();

        Ok(())
    }

    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64)
    -> ProgramResult {
        
        //Transfer tokens from depositor token account to pool token account
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
                anchor_spl::token::Transfer{
                    from: ctx.accounts.depositor_token_account.to_account_info(),
                    to: ctx.accounts.pool_token_account.to_account_info(),
                    authority: ctx.accounts.depositor.to_account_info()
                }),
            amount
        )?;

        //Mint voucher tokens and transfer them to the depositors voucher token account
        anchor_spl::token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::MintTo{
                    mint: ctx.accounts.voucher_mint.to_account_info(),
                    to: ctx.accounts.depositor_voucher_account.to_account_info(),
                    authority: ctx.accounts.state.to_account_info(),
                }, 
                &[&[PROGRAMSEED, &[ctx.accounts.state.bump]]]), 
            amount
        )?;

        Ok(())
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        amount: u64,
    ) 
    -> ProgramResult {

        //Burn an 'amount' equivalent of the voucher tokens
        anchor_spl::token::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Burn{
                    mint: ctx.accounts.voucher_mint.to_account_info(),
                    to: ctx.accounts.depositor_voucher_account.to_account_info(),
                    authority: ctx.accounts.depositor.to_account_info()
                }
            ), 
            amount
        )?;

        //Transfer tokens from pool to depositor
        anchor_spl::token::transfer(
        CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        anchor_spl::token::Transfer{
                    from: ctx.accounts.pool_token_account.to_account_info(),
                    to: ctx.accounts.depositor_token_account.to_account_info(),
                    authority: ctx.accounts.state.to_account_info()
                },
    &[&[PROGRAMSEED, &[ctx.accounts.state.bump]]]
            ), 
            amount
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(program_authority_bump: u8)]
pub struct InitializeAuthority<'info> 
{
    pub admin: Signer<'info>,
    #[account(init, seeds =[PROGRAMSEED], bump = program_authority_bump, payer = admin)]
    pub state: Account<'info, ProgramAuthority>,

    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(pool_bump : u8)]
pub struct AddPool<'info>
{
    pub admin: Signer<'info>,

    #[account(
        seeds = [PROGRAMSEED], bump = state.bump, 
        constraint = admin.key() == state.authority
    )]
    pub state: Account<'info, ProgramAuthority>,
    
    pub token_mint : Account<'info, Mint>,

    #[account(
        init, 
        seeds = [POOLSEED, token_mint.key().as_ref()],
        bump = pool_bump, 
        payer = admin
    )]
    pub pool : Account<'info, Pool>,

    #[account(
        init,
        seeds = [TOKENSEED, token_mint.key().as_ref()],
        bump,
        token::mint = token_mint,
        token::authority = state,
        payer = admin
    )]
    pub pool_token : Account<'info, TokenAccount>,

    #[account(
        init,
        seeds = [VOUCHERSEED, token_mint.key().as_ref()],
        bump,
        mint::decimals = token_mint.decimals,
        mint::authority = state,
        payer = admin
    )]
    pub voucher_mint : Account<'info, Mint>,

    
    rent: Sysvar<'info, Rent>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Deposit<'info> 
{
    //User account that signs token deposits
    pub depositor : Signer<'info>,

    //Program state
    #[account(
        seeds = [PROGRAMSEED],
        bump = state.bump
    )]
    pub state: Account<'info, ProgramAuthority>,

    //Pool 
    #[account(
        seeds = [POOLSEED, pool.token_mint_address.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,

    //User token account
    #[account(
        mut,
        constraint = depositor_token_account.mint == pool.token_mint_address
    )]
    pub depositor_token_account : Account<'info, TokenAccount>,

    //Pool token account
    #[account(
        mut,
        address = pool.token_account_address,
        constraint = pool_token_account.mint == pool.token_mint_address
    )]
    pub pool_token_account : Account<'info, TokenAccount>,

    // //Pool voucher mint
    #[account(
        mut,
        address = pool.voucher_mint_address
    )]
    pub voucher_mint: Account<'info, Mint>,

    // //User voucher token
    #[account(
        init_if_needed,
        payer = depositor,
        associated_token::mint = voucher_mint,
        associated_token::authority = depositor    
    )]
    pub depositor_voucher_account : Account<'info, TokenAccount>,

    rent: Sysvar<'info, Rent>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Withdraw<'info> 
{
    //User account that signs token deposits
    pub depositor : Signer<'info>,

    //Program state
    #[account(
        seeds = [PROGRAMSEED],
        bump = state.bump
    )]
    pub state: Account<'info, ProgramAuthority>,

    //Pool 
    #[account(
        seeds = [POOLSEED, pool.token_mint_address.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,

    //User token account
    #[account(
        mut,
        constraint = depositor_token_account.mint == pool.token_mint_address
    )]
    pub depositor_token_account : Account<'info, TokenAccount>,

    //Pool token account
    #[account(
        mut,
        address = pool.token_account_address,
        constraint = pool_token_account.mint == pool.token_mint_address
    )]
    pub pool_token_account : Account<'info, TokenAccount>,

    // //Pool voucher mint
    #[account(
        mut,
        address = pool.voucher_mint_address
    )]
    pub voucher_mint: Account<'info, Mint>,

    // //User voucher token
    #[account(
        mut,
        constraint = depositor_voucher_account.mint == pool.voucher_mint_address
    )]
    pub depositor_voucher_account : Account<'info, TokenAccount>,

    rent: Sysvar<'info, Rent>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>
}

#[account]
#[derive(Default)]
pub struct ProgramAuthority {
    pub bump: u8,
    pub authority: Pubkey,
}

#[account]
#[derive(Default)]
pub struct Pool {
    pub bump : u8,
    pub token_mint_address : Pubkey,
    pub token_account_address : Pubkey,
    pub voucher_mint_address : Pubkey
}
use anchor_lang::{prelude::{AccountInfo, CpiContext, ProgramResult, AccountMeta}, Accounts, Key, solana_program, ToAccountMetas, ToAccountInfos};
use solend_token_lending::state::ReserveConfig;


#[derive(Accounts)]
pub struct InitLendingMarketAccounts<'info> {
    pub owner: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub oracle_program_id: AccountInfo<'info>,
    pub switchboard_oracle_program_id: AccountInfo<'info>,
}

fn init_lending_market<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, InitLendingMarketAccounts<'info>>,
    quote_currency: [u8; 32]
) -> ProgramResult {
    let ix = solend_token_lending::instruction::init_lending_market(
        solend_token_lending::id(),
        ctx.accounts.owner.key(),
        quote_currency,
        ctx.accounts.lending_market_pubkey.key(),
        ctx.accounts.oracle_program_id.key(),
        ctx.accounts.switchboard_oracle_program_id.key(),
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.owner.clone(),
            ctx.accounts.lending_market_pubkey.clone(),
            ctx.accounts.oracle_program_id.clone(),
            ctx.accounts.switchboard_oracle_program_id.clone(),
        ],
        ctx.signer_seeds,
    )
}


#[derive(Accounts)]
pub struct SetLendingMarketOwnerAccounts<'info> {
    pub new_owner: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub lending_market_owner: AccountInfo<'info>,
}

fn set_lending_market_owner<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, SetLendingMarketOwnerAccounts<'info>>
) -> ProgramResult {
    let ix = solend_token_lending::instruction::set_lending_market_owner(
        solend_token_lending::id(),
        ctx.accounts.lending_market_pubkey.key(),
        ctx.accounts.lending_market_owner.key(),
        ctx.accounts.new_owner.key(),
    );

    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.lending_market_pubkey.clone(),
            ctx.accounts.lending_market_owner.clone(),
            ctx.accounts.new_owner.clone(),
        ],
        ctx.signer_seeds,
    )
}

#[derive(Accounts)]
pub struct InitReserveAccounts<'info> {
    pub source_liquidity_pubkey: AccountInfo<'info>,
    pub destination_collateral_pubkey: AccountInfo<'info>,
    pub reserve_pubkey: AccountInfo<'info>,
    pub reserve_liquidity_mint_pubkey: AccountInfo<'info>, 
    pub reserve_liquidity_supply_pubkey: AccountInfo<'info>, 
    pub reserve_collateral_mint_pubkey: AccountInfo<'info>, 
    pub reserve_collateral_supply_pubkey: AccountInfo<'info>, 
    pub pyth_product_pubkey: AccountInfo<'info>,
    pub pyth_price_pubkey: AccountInfo<'info>, 
    pub switchboard_feed_pubkey: AccountInfo<'info>, 
    pub lending_market_pubkey: AccountInfo<'info>, 
    pub lending_market_owner_pubkey: AccountInfo<'info>, 
    pub user_transfer_authority_pubkey: AccountInfo<'info>
}

fn init_reserve<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, InitReserveAccounts<'info>>,
    liquidity_amount:u64,
    config: ReserveConfig
) -> ProgramResult {
    let ix = solend_token_lending::instruction::init_reserve(
        solend_token_lending::id(),
        liquidity_amount,
        config,
        ctx.accounts.source_liquidity_pubkey.key(),
        ctx.accounts.destination_collateral_pubkey.key(),
        ctx.accounts.reserve_pubkey.key(),
        ctx.accounts.reserve_liquidity_mint_pubkey.key(),
        ctx.accounts.reserve_liquidity_supply_pubkey.key(),
        ctx.accounts.reserve_collateral_mint_pubkey.key(),
        ctx.accounts.reserve_collateral_supply_pubkey.key(),
        ctx.accounts.pyth_product_pubkey.key(),
        ctx.accounts.pyth_price_pubkey.key(),
        ctx.accounts.switchboard_feed_pubkey.key(),
        ctx.accounts.lending_market_pubkey.key(),
        ctx.accounts.lending_market_owner_pubkey.key(),
        ctx.accounts.user_transfer_authority_pubkey.key(),
    );
    solana_program::program::invoke_signed(
        &ix,
        &ctx.accounts.to_account_infos(),
        ctx.signer_seeds,
    )?;
    Ok(())
}



#[derive(Accounts)]
pub struct RefreshReserveAccounts<'info> {
    pub reserve_pubkey: AccountInfo<'info>,
    pub reserve_liquidity_pyth_oracle_pubkey: AccountInfo<'info>,
    pub reserve_liquidity_switchboard_oracle_pubkey: AccountInfo<'info>,
}

fn refresh_reserve<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, RefreshReserveAccounts<'info>>
) -> ProgramResult {
    let ix = solend_token_lending::instruction::refresh_reserve(
        solend_token_lending::id(),
        ctx.accounts.reserve_pubkey.key(),
        ctx.accounts.reserve_liquidity_pyth_oracle_pubkey.key(),
        ctx.accounts.reserve_liquidity_switchboard_oracle_pubkey.key(),
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.reserve_pubkey.clone(),
            ctx.accounts.reserve_liquidity_pyth_oracle_pubkey.clone(),
            ctx.accounts.reserve_liquidity_switchboard_oracle_pubkey.clone(),
        ],
        ctx.signer_seeds,
    )
}

#[derive(Accounts)]
pub struct DepositReserveLiquidityAccounts<'info> {
    pub source_liquidity_pubkey: AccountInfo<'info>,
    pub destination_collateral_pubkey: AccountInfo<'info>,
    pub reserve_pubkey: AccountInfo<'info>,
    pub reserve_liquidity_supply_pubkey: AccountInfo<'info>,
    pub reserve_collateral_mint_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub user_transfer_authority_pubkey: AccountInfo<'info>,
} 

fn deposit_reserve_liquidity<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, DepositReserveLiquidityAccounts<'info>>,
    liquidity_amount: u64,
) -> ProgramResult {
    let ix = solend_token_lending::instruction::deposit_reserve_liquidity(
        solend_token_lending::id(),
        liquidity_amount,
        ctx.accounts.source_liquidity_pubkey.key(),
        ctx.accounts.destination_collateral_pubkey.key(),
        ctx.accounts.reserve_pubkey.key(),
        ctx.accounts.reserve_liquidity_supply_pubkey.key(),
        ctx.accounts.reserve_collateral_mint_pubkey.key(),
        ctx.accounts.lending_market_pubkey.key(),
        ctx.accounts.user_transfer_authority_pubkey.key(),
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.source_liquidity_pubkey.clone(),
            ctx.accounts.destination_collateral_pubkey.clone(),
            ctx.accounts.reserve_pubkey.clone(),
            ctx.accounts.reserve_liquidity_supply_pubkey.clone(),
            ctx.accounts.reserve_collateral_mint_pubkey.clone(),
            ctx.accounts.lending_market_pubkey.clone(),
            ctx.accounts.user_transfer_authority_pubkey.clone(),
        ],
        ctx.signer_seeds,
    )
}

#[derive(Accounts)]
pub struct RedeemReserveCollateralAccounts<'info> {
    pub source_collateral_pubkey: AccountInfo<'info>,
    pub destination_liquidity_pubkey: AccountInfo<'info>,
    pub reserve_pubkey: AccountInfo<'info>,
    pub reserve_collateral_mint_pubkey: AccountInfo<'info>,
    pub reserve_liquidity_supply_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub user_transfer_authority_pubkey: AccountInfo<'info>,
} 

fn redeem_reserve_collateral<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, RedeemReserveCollateralAccounts<'info>>,
    collateral_amount: u64,
) -> ProgramResult {
    let ix = solend_token_lending::instruction::redeem_reserve_collateral(
        solend_token_lending::id(),
        collateral_amount,
        ctx.accounts.source_collateral_pubkey.key(),
        ctx.accounts.destination_liquidity_pubkey.key(),
        ctx.accounts.reserve_pubkey.key(),
        ctx.accounts.reserve_collateral_mint_pubkey.key(),
        ctx.accounts.reserve_liquidity_supply_pubkey.key(),
        ctx.accounts.lending_market_pubkey.key(),
        ctx.accounts.user_transfer_authority_pubkey.key(),
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.source_collateral_pubkey.clone(),
            ctx.accounts.destination_liquidity_pubkey.clone(),
            ctx.accounts.reserve_pubkey.clone(),
            ctx.accounts.reserve_collateral_mint_pubkey.clone(),
            ctx.accounts.reserve_liquidity_supply_pubkey.clone(),
            ctx.accounts.lending_market_pubkey.clone(),
            ctx.accounts.user_transfer_authority_pubkey.clone(),
        ],
        ctx.signer_seeds,
    )
}

#[derive(Accounts)]
pub struct InitObligationAccounts<'info> {
    pub obligation_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub obligation_owner_pubkey: AccountInfo<'info>,
} 

fn init_obligation<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, InitObligationAccounts<'info>>
) -> ProgramResult {
    let ix = solend_token_lending::instruction::init_obligation(
        solend_token_lending::id(),
        ctx.accounts.obligation_pubkey.key(),
        ctx.accounts.lending_market_pubkey.key(),
        ctx.accounts.obligation_owner_pubkey.key(),
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.obligation_pubkey.clone(),
            ctx.accounts.lending_market_pubkey.clone(),
            ctx.accounts.obligation_owner_pubkey.clone(),
        ],
        ctx.signer_seeds,
    )
}

#[derive(Accounts)]
pub struct RefreshObligationAccounts<'info> {
    pub obligation_pubkey: AccountInfo<'info>,
    pub reserve_pubkeys: AccountInfo<'info>,
} 

fn refresh_obligation<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, RefreshObligationAccounts<'info>>
) -> ProgramResult {
    let reserve_pubkeys = ctx.remaining_accounts;
    let keys = reserve_pubkeys.iter().map(|k| k.key()).collect();
    let ix = solend_token_lending::instruction::refresh_obligation(
        solend_token_lending::id(),
        ctx.accounts.obligation_pubkey.key(),
        keys,
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.obligation_pubkey.clone(),
            ctx.accounts.reserve_pubkeys.clone(),
        ],
        ctx.signer_seeds,
    )
}

#[derive(Accounts)]
pub struct DepositObligationCollateralAccounts<'info> {
    pub source_collateral_pubkey: AccountInfo<'info>,
    pub destination_collateral_pubkey: AccountInfo<'info>,
    pub deposit_reserve_pubkey: AccountInfo<'info>,
    pub obligation_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub obligation_owner_pubkey: AccountInfo<'info>,
    pub user_transfer_authority_pubkey: AccountInfo<'info>,
} 

fn deposit_obligation_collateral<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, DepositObligationCollateralAccounts<'info>>,
    collateral_amount: u64,
) -> ProgramResult {
    let ix = solend_token_lending::instruction::deposit_obligation_collateral(
        solend_token_lending::id(),
        collateral_amount,
        ctx.accounts.source_collateral_pubkey.key(),
        ctx.accounts.destination_collateral_pubkey.key(),
        ctx.accounts.deposit_reserve_pubkey.key(),
        ctx.accounts.obligation_pubkey.key(),
        ctx.accounts.lending_market_pubkey.key(),
        ctx.accounts.obligation_owner_pubkey.key(),
        ctx.accounts.user_transfer_authority_pubkey.key(),
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.source_collateral_pubkey.clone(),
            ctx.accounts.destination_collateral_pubkey.clone(),
            ctx.accounts.deposit_reserve_pubkey.clone(),
            ctx.accounts.obligation_pubkey.clone(),
            ctx.accounts.lending_market_pubkey.clone(),
            ctx.accounts.obligation_owner_pubkey.clone(),
            ctx.accounts.user_transfer_authority_pubkey.clone(),
        ],
        ctx.signer_seeds,
    )
}

#[derive(Accounts)]
pub struct WithdrawObligationCollateralAccounts<'info> {
    pub source_collateral_pubkey: AccountInfo<'info>,
    pub destination_collateral_pubkey: AccountInfo<'info>,
    pub withdraw_reserve_pubkey: AccountInfo<'info>,
    pub obligation_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub obligation_owner_pubkey: AccountInfo<'info>,
} 

fn withdraw_obligation_collateral<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, WithdrawObligationCollateralAccounts<'info>>,
    collateral_amount: u64,
) -> ProgramResult {
    let ix = solend_token_lending::instruction::withdraw_obligation_collateral(
        solend_token_lending::id(),
        collateral_amount,
        ctx.accounts.source_collateral_pubkey.key(),
        ctx.accounts.destination_collateral_pubkey.key(),
        ctx.accounts.withdraw_reserve_pubkey.key(),
        ctx.accounts.obligation_pubkey.key(),
        ctx.accounts.lending_market_pubkey.key(),
        ctx.accounts.obligation_owner_pubkey.key(),    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.source_collateral_pubkey.clone(),
            ctx.accounts.destination_collateral_pubkey.clone(),
            ctx.accounts.withdraw_reserve_pubkey.clone(),
            ctx.accounts.obligation_pubkey.clone(),
            ctx.accounts.lending_market_pubkey.clone(),
            ctx.accounts.obligation_owner_pubkey.clone(),
        ],
        ctx.signer_seeds,
    )
}

#[derive(Accounts)]
pub struct BorrowObligationLiquidityAccounts<'info> {
    pub source_liquidity_pubkey: AccountInfo<'info>,
    pub destination_liquidity_pubkey: AccountInfo<'info>,
    pub borrow_reserve_pubkey: AccountInfo<'info>,
    pub borrow_reserve_liquidity_fee_receiver_pubkey: AccountInfo<'info>,
    pub obligation_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub obligation_owner_pubkey: AccountInfo<'info>,
    pub host_fee_receiver_pubkey: AccountInfo<'info>,
} 

fn borrow_obligation_liquidity<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, BorrowObligationLiquidityAccounts<'info>>,
    liquidity_amount: u64,
) -> ProgramResult {
    let host_fee_receiver_pubkey = ctx.remaining_accounts.get(0);
    let ix = solend_token_lending::instruction::borrow_obligation_liquidity(
        solend_token_lending::id(),
        liquidity_amount,
        ctx.accounts.source_liquidity_pubkey.key(),
        ctx.accounts.destination_liquidity_pubkey.key(),
        ctx.accounts.borrow_reserve_pubkey.key(),
        ctx.accounts.borrow_reserve_liquidity_fee_receiver_pubkey.key(),
        ctx.accounts.obligation_pubkey.key(),
        ctx.accounts.lending_market_pubkey.key(), 
        ctx.accounts.obligation_owner_pubkey.key(),
        host_fee_receiver_pubkey.map(|k| k.key()),
       );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.source_liquidity_pubkey.clone(),
            ctx.accounts.destination_liquidity_pubkey.clone(),
            ctx.accounts.borrow_reserve_pubkey.clone(),
            ctx.accounts.borrow_reserve_liquidity_fee_receiver_pubkey.clone(),
            ctx.accounts.obligation_pubkey.clone(),
            ctx.accounts.lending_market_pubkey.clone(), 
            ctx.accounts.obligation_owner_pubkey.clone(),
            ctx.accounts.host_fee_receiver_pubkey.clone(),
        ],
        ctx.signer_seeds,
    )
}

#[derive(Accounts)]
pub struct RepayObligationLiquidityAccounts<'info> {
    pub source_liquidity_pubkey: AccountInfo<'info>,
    pub destination_liquidity_pubkey: AccountInfo<'info>,
    pub repay_reserve_pubkey: AccountInfo<'info>,
    pub obligation_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub user_transfer_authority_pubkey: AccountInfo<'info>,
}  

fn repay_obligation_liquidity<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, RepayObligationLiquidityAccounts<'info>>,
    liquidity_amount: u64,
) -> ProgramResult {
    let ix = solend_token_lending::instruction::repay_obligation_liquidity(
        solend_token_lending::id(),
        liquidity_amount,
        ctx.accounts.source_liquidity_pubkey.key(),
        ctx.accounts.destination_liquidity_pubkey.key(),
        ctx.accounts.repay_reserve_pubkey.key(),
        ctx.accounts.obligation_pubkey.key(),
        ctx.accounts.lending_market_pubkey.key(),
        ctx.accounts.user_transfer_authority_pubkey.key(), 
       );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.source_liquidity_pubkey.clone(),
            ctx.accounts.destination_liquidity_pubkey.clone(),
            ctx.accounts.repay_reserve_pubkey.clone(),
            ctx.accounts.obligation_pubkey.clone(),
            ctx.accounts.lending_market_pubkey.clone(),
            ctx.accounts.user_transfer_authority_pubkey.clone(), 
        ],
        ctx.signer_seeds,
    )
}

#[derive(Accounts)]
pub struct LiquidateObligationAccounts<'info> {
    pub source_liquidity_pubkey: AccountInfo<'info>,
    pub destination_collateral_pubkey: AccountInfo<'info>,
    pub repay_reserve_pubkey: AccountInfo<'info>,
    pub repay_reserve_liquidity_supply_pubkey: AccountInfo<'info>,
    pub withdraw_reserve_pubkey: AccountInfo<'info>,
    pub withdraw_reserve_collateral_supply_pubkey: AccountInfo<'info>,
    pub obligation_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub user_transfer_authority_pubkey: AccountInfo<'info>,
}  

fn liquidate_obligation<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, LiquidateObligationAccounts<'info>>,
    liquidity_amount: u64,
) -> ProgramResult {
    let ix = solend_token_lending::instruction::liquidate_obligation(
        solend_token_lending::id(),
        liquidity_amount,
        ctx.accounts.source_liquidity_pubkey.key(),
        ctx.accounts.destination_collateral_pubkey.key(),
        ctx.accounts.repay_reserve_pubkey.key(),
        ctx.accounts.repay_reserve_liquidity_supply_pubkey.key(),
        ctx.accounts.withdraw_reserve_pubkey.key(),
        ctx.accounts.withdraw_reserve_collateral_supply_pubkey.key(), 
        ctx.accounts.obligation_pubkey.key(),
        ctx.accounts.lending_market_pubkey.key(),
        ctx.accounts.user_transfer_authority_pubkey.key(), 
       );

    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.source_liquidity_pubkey.clone(),
            ctx.accounts.destination_collateral_pubkey.clone(),
            ctx.accounts.repay_reserve_pubkey.clone(),
            ctx.accounts.repay_reserve_liquidity_supply_pubkey.clone(),
            ctx.accounts.withdraw_reserve_pubkey.clone(),
            ctx.accounts.withdraw_reserve_collateral_supply_pubkey.clone(), 
            ctx.accounts.obligation_pubkey.clone(),
            ctx.accounts.lending_market_pubkey.clone(),
            ctx.accounts.user_transfer_authority_pubkey.clone(),  
        ],
        ctx.signer_seeds,
    )
}

#[derive(Accounts)]
pub struct FlashLoanAccounts<'info> {
    pub source_liquidity_pubkey: AccountInfo<'info>,
    pub destination_liquidity_pubkey: AccountInfo<'info>,
    pub reserve_pubkey: AccountInfo<'info>,
    pub reserve_liquidity_fee_receiver_pubkey: AccountInfo<'info>,
    pub host_fee_receiver_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub flash_loan_receiver_program_id: AccountInfo<'info>,
    // pub accs: Vec<AccountInfo<'info>>,
}  
fn flash_loan<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, FlashLoanAccounts<'info>>,
    amount: u64,
    // flash_loan_receiver_program_accounts: Vec<AccountMeta>,
) -> ProgramResult {
    let mut flash_loan_receiver_program_accounts = ctx.remaining_accounts;
    let ix = solend_token_lending::instruction::flash_loan(
        solend_token_lending::id(),
        amount,
        ctx.accounts.source_liquidity_pubkey.key(),
        ctx.accounts.destination_liquidity_pubkey.key(),
        ctx.accounts.reserve_pubkey.key(),
        ctx.accounts.reserve_liquidity_fee_receiver_pubkey.key(),
        ctx.accounts.host_fee_receiver_pubkey.key(),
        ctx.accounts.lending_market_pubkey.key(), 
        ctx.accounts.flash_loan_receiver_program_id.key(),
        flash_loan_receiver_program_accounts.to_account_metas(None),
       );

    let mut accounts_infos = vec!(
        ctx.accounts.source_liquidity_pubkey.clone(),
        ctx.accounts.destination_liquidity_pubkey.clone(),
        ctx.accounts.reserve_pubkey.clone(),
        ctx.accounts.reserve_liquidity_fee_receiver_pubkey.clone(),
        ctx.accounts.host_fee_receiver_pubkey.clone(),
        ctx.accounts.lending_market_pubkey.clone(), 
        ctx.accounts.flash_loan_receiver_program_id.clone(),
    );
    accounts_infos.extend(flash_loan_receiver_program_accounts);

    solana_program::program::invoke_signed(
        &ix,
        &accounts_infos,
        ctx.signer_seeds,
    )
}


#[derive(Accounts)]
pub struct DepositReserveLiquidityAndObligationCollateralAccounts<'info> {
    pub source_liquidity_pubkey: AccountInfo<'info>,
    pub user_collateral_pubkey: AccountInfo<'info>,
    pub reserve_pubkey: AccountInfo<'info>,
    pub reserve_liquidity_supply_pubkey: AccountInfo<'info>,
    pub reserve_collateral_mint_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub destination_deposit_collateral_pubkey: AccountInfo<'info>,
    pub obligation_pubkey: AccountInfo<'info>,
    pub obligation_owner_pubkey: AccountInfo<'info>,
    pub reserve_liquidity_pyth_oracle_pubkey: AccountInfo<'info>,
    pub reserve_liquidity_switchboard_oracle_pubkey: AccountInfo<'info>,
    pub user_transfer_authority_pubkey: AccountInfo<'info>,
} 

fn deposit_reserve_liquidity_and_obligation_collateral<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, DepositReserveLiquidityAndObligationCollateralAccounts<'info>>,
    liquidity_amount: u64,
) -> ProgramResult {
    let ix = solend_token_lending::instruction::deposit_reserve_liquidity_and_obligation_collateral(
        solend_token_lending::id(),
        liquidity_amount,
        ctx.accounts.source_liquidity_pubkey.key(),
        ctx.accounts.user_collateral_pubkey.key(),
        ctx.accounts.reserve_pubkey.key(),
        ctx.accounts.reserve_liquidity_supply_pubkey.key(),
        ctx.accounts.reserve_collateral_mint_pubkey.key(),
        ctx.accounts.lending_market_pubkey.key(),
        ctx.accounts.destination_deposit_collateral_pubkey.key(),
        ctx.accounts.obligation_pubkey.key(),
        ctx.accounts.obligation_owner_pubkey.key(),
        ctx.accounts.reserve_liquidity_pyth_oracle_pubkey.key(),
        ctx.accounts.reserve_liquidity_switchboard_oracle_pubkey.key(),
        ctx.accounts.user_transfer_authority_pubkey.key(),
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.source_liquidity_pubkey.clone(),
            ctx.accounts.user_collateral_pubkey.clone(),
            ctx.accounts.reserve_pubkey.clone(),
            ctx.accounts.reserve_liquidity_supply_pubkey.clone(),
            ctx.accounts.reserve_collateral_mint_pubkey.clone(),
            ctx.accounts.lending_market_pubkey.clone(),
            ctx.accounts.destination_deposit_collateral_pubkey.clone(),
            ctx.accounts.obligation_pubkey.clone(),
            ctx.accounts.obligation_owner_pubkey.clone(),
            ctx.accounts.reserve_liquidity_pyth_oracle_pubkey.clone(),
            ctx.accounts.reserve_liquidity_switchboard_oracle_pubkey.clone(),
            ctx.accounts.user_transfer_authority_pubkey.clone(),
        ],
        ctx.signer_seeds,
    )
}

#[derive(Accounts)]
pub struct UpdateReserveConfigAccounts<'info> {
    pub reserve_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub lending_market_owner_pubkey: AccountInfo<'info>,
    pub pyth_product_pubkey: AccountInfo<'info>,
    pub pyth_price_pubkey: AccountInfo<'info>,
    pub switchboard_feed_pubkey: AccountInfo<'info>,
}   

fn update_reserve_config<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, UpdateReserveConfigAccounts<'info>>,
    config: ReserveConfig,
) -> ProgramResult {
    let ix = solend_token_lending::instruction::update_reserve_config(
        solend_token_lending::id(),
        config,
        ctx.accounts.reserve_pubkey.key(),
        ctx.accounts.lending_market_pubkey.key(),
        ctx.accounts.lending_market_owner_pubkey.key(),
        ctx.accounts.pyth_product_pubkey.key(),
        ctx.accounts.pyth_price_pubkey.key(),
        ctx.accounts.switchboard_feed_pubkey.key(),
       );

    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.reserve_pubkey.clone(),
            ctx.accounts.lending_market_pubkey.clone(),
            ctx.accounts.lending_market_owner_pubkey.clone(),
            ctx.accounts.pyth_product_pubkey.clone(),
            ctx.accounts.pyth_price_pubkey.clone(),
            ctx.accounts.switchboard_feed_pubkey.clone(),
        ],
        ctx.signer_seeds,
    )
}

fn main() {
    println!("Hello, world!");
    //init_reserve(ctx);
}

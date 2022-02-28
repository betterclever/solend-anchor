use anchor_lang::{
    prelude::{AccountInfo, CpiContext, Program, Pubkey, Rent, Sysvar, Clock, Signer},
    solana_program::{self, entrypoint::ProgramResult},
    Accounts, Key, ToAccountInfo, ToAccountInfos, ToAccountMetas,
};
use solend_token_lending::state::{ReserveConfig, ReserveFees};

#[derive(Clone)]
pub struct TokenProgram;

impl anchor_lang::AccountDeserialize for TokenProgram {
    fn try_deserialize(buf: &mut &[u8]) -> Result<TokenProgram, anchor_lang::error::Error> {
        TokenProgram::try_deserialize_unchecked(buf)
    }

    fn try_deserialize_unchecked(
        _buf: &mut &[u8],
    ) -> Result<TokenProgram, anchor_lang::error::Error> {
        Ok(TokenProgram)
    }
}

impl anchor_lang::Id for TokenProgram {
    fn id() -> Pubkey {
        spl_token::id()
    }
}

#[derive(Accounts)]
pub struct InitLendingMarketAccounts<'info> {
    pub owner: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub oracle_program_id: AccountInfo<'info>,
    pub switchboard_oracle_program_id: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, TokenProgram>,
}

pub fn init_lending_market<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, InitLendingMarketAccounts<'info>>,
    quote_currency: [u8; 32],
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
            ctx.accounts.lending_market_pubkey.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.oracle_program_id.to_account_info(),
            ctx.accounts.switchboard_oracle_program_id.to_account_info(),
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct SetLendingMarketOwnerAccounts<'info> {
    pub new_owner: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub lending_market_owner: AccountInfo<'info>,
}

pub fn set_lending_market_owner<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, SetLendingMarketOwnerAccounts<'info>>,
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
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
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
    pub lending_market_owner_pubkey: Signer<'info>,
    pub user_transfer_authority_pubkey: Signer<'info>,
    pub fee_receiver_pubkey: AccountInfo<'info>,
    pub lending_market_authority_pubkey: AccountInfo<'info>,
    pub rent_sysvar: Sysvar<'info, Rent>,
    pub clock_sysvar: Sysvar<'info, Clock>,
    pub token_program: Program<'info, TokenProgram>,
}

pub struct ReserveConfigData {
    /// Optimal utilization rate, as a percentage
    pub optimal_utilization_rate: u8,
    /// Target ratio of the value of borrows to deposits, as a percentage
    /// 0 if use as collateral is disabled
    pub loan_to_value_ratio: u8,
    /// Bonus a liquidator gets when repaying part of an unhealthy obligation, as a percentage
    pub liquidation_bonus: u8,
    /// Loan to value ratio at which an obligation can be liquidated, as a percentage
    pub liquidation_threshold: u8,
    /// Min borrow APY
    pub min_borrow_rate: u8,
    /// Optimal (utilization) borrow APY
    pub optimal_borrow_rate: u8,
    /// Max borrow APY
    pub max_borrow_rate: u8,
    /// Program owner fees assessed, separate from gains due to interest accrual
    pub fees: ReserveFees,
    /// Maximum deposit limit of liquidity in native units, u64::MAX for inf
    pub deposit_limit: u64,
    /// Borrows disabled
    pub borrow_limit: u64,
} 

pub fn init_reserve<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, InitReserveAccounts<'info>>,
    liquidity_amount: u64,
    config_data: ReserveConfigData,
) -> ProgramResult {
    
    let config =  ReserveConfig {
        optimal_utilization_rate: config_data.optimal_utilization_rate,
        loan_to_value_ratio: config_data.loan_to_value_ratio,
        liquidation_bonus: config_data.liquidation_bonus,
        liquidation_threshold: config_data.liquidation_threshold,
        min_borrow_rate: config_data.min_borrow_rate,
        optimal_borrow_rate: config_data.optimal_utilization_rate,
        max_borrow_rate: config_data.max_borrow_rate,
        fees: config_data.fees,
        deposit_limit: config_data.deposit_limit,
        borrow_limit: config_data.borrow_limit,
        fee_receiver: ctx.accounts.fee_receiver_pubkey.key()
    };

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
    
    solana_program::program::invoke_signed(&ix, &[
        ctx.accounts.source_liquidity_pubkey.to_account_info(),
        ctx.accounts.destination_collateral_pubkey.to_account_info(),
        ctx.accounts.reserve_pubkey.to_account_info(),
        ctx.accounts.reserve_liquidity_mint_pubkey.to_account_info(),
        ctx.accounts.reserve_collateral_supply_pubkey.to_account_info(),
        ctx.accounts.fee_receiver_pubkey.to_account_info(),
        ctx.accounts.reserve_collateral_mint_pubkey.to_account_info(),
        ctx.accounts.reserve_collateral_supply_pubkey.to_account_info(),
        ctx.accounts.pyth_product_pubkey.to_account_info(),
        ctx.accounts.pyth_price_pubkey.to_account_info(),
        ctx.accounts.switchboard_feed_pubkey.to_account_info(),
        ctx.accounts.switchboard_feed_pubkey.to_account_info(),
        ctx.accounts.lending_market_authority_pubkey.to_account_info(),
        ctx.accounts.lending_market_owner_pubkey.to_account_info(),
        ctx.accounts.user_transfer_authority_pubkey.to_account_info(),
        ctx.accounts.clock_sysvar.to_account_info(),
        ctx.accounts.rent_sysvar.to_account_info(),
        ctx.accounts.token_program.to_account_info()
    ], ctx.signer_seeds)
        .map_err(Into::into)
}

#[derive(Accounts)]
pub struct RefreshReserveAccounts<'info> {
    pub reserve_pubkey: AccountInfo<'info>,
    pub reserve_liquidity_pyth_oracle_pubkey: AccountInfo<'info>,
    pub reserve_liquidity_switchboard_oracle_pubkey: AccountInfo<'info>,
    pub clock_sysvar: Sysvar<'info, Clock>,
}

pub fn refresh_reserve<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, RefreshReserveAccounts<'info>>,
) -> ProgramResult {
    let ix = solend_token_lending::instruction::refresh_reserve(
        solend_token_lending::id(),
        ctx.accounts.reserve_pubkey.key(),
        ctx.accounts.reserve_liquidity_pyth_oracle_pubkey.key(),
        ctx.accounts
            .reserve_liquidity_switchboard_oracle_pubkey
            .key(),
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.reserve_pubkey.to_account_info(),
            ctx.accounts.reserve_liquidity_pyth_oracle_pubkey.to_account_info(),
            ctx.accounts
                .reserve_liquidity_switchboard_oracle_pubkey
                .to_account_info(),
            ctx.accounts.clock_sysvar.to_account_info()
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct DepositReserveLiquidityAccounts<'info> {
    pub source_liquidity_pubkey: AccountInfo<'info>,
    pub destination_collateral_pubkey: AccountInfo<'info>,
    pub reserve_pubkey: AccountInfo<'info>,
    pub reserve_liquidity_supply_pubkey: AccountInfo<'info>,
    pub reserve_collateral_mint_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub user_transfer_authority_pubkey: Signer<'info>,
    pub clock_sysvar: Sysvar<'info, Clock>,
    pub token_program: Program<'info, TokenProgram>,
}

pub fn deposit_reserve_liquidity<'a, 'b, 'c, 'info>(
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
            ctx.accounts.source_liquidity_pubkey.to_account_info(),
            ctx.accounts.destination_collateral_pubkey.to_account_info(),
            ctx.accounts.reserve_pubkey.to_account_info(),
            ctx.accounts.reserve_liquidity_supply_pubkey.to_account_info(),
            ctx.accounts.reserve_collateral_mint_pubkey.to_account_info(),
            ctx.accounts.lending_market_pubkey.to_account_info(),
            ctx.accounts.user_transfer_authority_pubkey.to_account_info(),
            ctx.accounts.clock_sysvar.to_account_info(),
            ctx.accounts.token_program.to_account_info()
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct RedeemReserveCollateralAccounts<'info> {
    pub source_collateral_pubkey: AccountInfo<'info>,
    pub destination_liquidity_pubkey: AccountInfo<'info>,
    pub reserve_pubkey: AccountInfo<'info>,
    pub reserve_collateral_mint_pubkey: AccountInfo<'info>,
    pub reserve_liquidity_supply_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub user_transfer_authority_pubkey: Signer<'info>,
    pub clock_sysvar: Sysvar<'info, Clock>,
    pub token_program: Program<'info, TokenProgram>,
}

pub fn redeem_reserve_collateral<'a, 'b, 'c, 'info>(
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
            ctx.accounts.source_collateral_pubkey.to_account_info(),
            ctx.accounts.destination_liquidity_pubkey.to_account_info(),
            ctx.accounts.reserve_pubkey.to_account_info(),
            ctx.accounts.reserve_collateral_mint_pubkey.to_account_info(),
            ctx.accounts.reserve_liquidity_supply_pubkey.to_account_info(),
            ctx.accounts.lending_market_pubkey.to_account_info(),
            ctx.accounts.user_transfer_authority_pubkey.to_account_info(),
            ctx.accounts.clock_sysvar.to_account_info(),
            ctx.accounts.token_program.to_account_info()
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct InitObligationAccounts<'info> {
    pub obligation_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub obligation_owner_pubkey: AccountInfo<'info>,
    pub clock_sysvar: Sysvar<'info, Clock>,
    pub rent_sysvar: Sysvar<'info, Rent>,
    pub token_program: Program<'info, TokenProgram>,
}

pub fn init_obligation<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, InitObligationAccounts<'info>>,
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
            ctx.accounts.obligation_pubkey.to_account_info(),
            ctx.accounts.lending_market_pubkey.to_account_info(),
            ctx.accounts.obligation_owner_pubkey.to_account_info(),
            ctx.accounts.clock_sysvar.to_account_info(),
            ctx.accounts.rent_sysvar.to_account_info(),
            ctx.accounts.token_program.to_account_info()
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct RefreshObligationAccounts<'info> {
    pub obligation_pubkey: AccountInfo<'info>,
    pub reserve_pubkeys: AccountInfo<'info>,
}

pub fn refresh_obligation<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, RefreshObligationAccounts<'info>>,
) -> ProgramResult {
    let reserve_pubkeys = ctx.remaining_accounts;
    let keys = reserve_pubkeys.iter().map(|k| k.key()).collect();
    let ix = solend_token_lending::instruction::refresh_obligation(
        solend_token_lending::id(),
        ctx.accounts.obligation_pubkey.key(),
        keys,
    );

    let mut account_infos = vec![
        ctx.accounts.obligation_pubkey.to_account_info(),
        ctx.accounts.reserve_pubkeys.to_account_info()
    ];

    account_infos.extend(reserve_pubkeys.to_account_infos());
    solana_program::program::invoke_signed(
        &ix,
        &account_infos,
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct DepositObligationCollateralAccounts<'info> {
    pub source_collateral_pubkey: AccountInfo<'info>,
    pub destination_collateral_pubkey: AccountInfo<'info>,
    pub deposit_reserve_pubkey: AccountInfo<'info>,
    pub obligation_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub obligation_owner_pubkey: Signer<'info>,
    pub user_transfer_authority_pubkey: Signer<'info>,
    pub clock_sysvar: Sysvar<'info, Clock>,
    pub token_program: Program<'info, TokenProgram>,
}

pub fn deposit_obligation_collateral<'a, 'b, 'c, 'info>(
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
            ctx.accounts.source_collateral_pubkey.to_account_info(),
            ctx.accounts.destination_collateral_pubkey.to_account_info(),
            ctx.accounts.deposit_reserve_pubkey.to_account_info(),
            ctx.accounts.obligation_pubkey.to_account_info(),
            ctx.accounts.lending_market_pubkey.to_account_info(),
            ctx.accounts.obligation_owner_pubkey.to_account_info(),
            ctx.accounts.user_transfer_authority_pubkey.to_account_info(),
            ctx.accounts.clock_sysvar.to_account_info(),
            ctx.accounts.token_program.to_account_info()
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct WithdrawObligationCollateralAccounts<'info> {
    pub source_collateral_pubkey: AccountInfo<'info>,
    pub destination_collateral_pubkey: AccountInfo<'info>,
    pub withdraw_reserve_pubkey: AccountInfo<'info>,
    pub obligation_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub obligation_owner_pubkey: Signer<'info>,
    pub clock_sysvar: Sysvar<'info, Clock>,
    pub token_program: Program<'info, TokenProgram>,
}

pub fn withdraw_obligation_collateral<'a, 'b, 'c, 'info>(
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
        ctx.accounts.obligation_owner_pubkey.key(),
    );

    let account_infos = vec![
        ctx.accounts.source_collateral_pubkey.to_account_info(),
        ctx.accounts.destination_collateral_pubkey.to_account_info(),
        ctx.accounts.withdraw_reserve_pubkey.to_account_info(),
        ctx.accounts.obligation_pubkey.to_account_info(),
        ctx.accounts.lending_market_pubkey.to_account_info(),
        ctx.accounts.obligation_owner_pubkey.to_account_info(),
        ctx.accounts.clock_sysvar.to_account_info(),
        ctx.accounts.token_program.to_account_info()
    ];
    solana_program::program::invoke_signed(
        &ix,
        &account_infos,
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct BorrowObligationLiquidityAccounts<'info> {
    pub source_liquidity_pubkey: AccountInfo<'info>,
    pub destination_liquidity_pubkey: AccountInfo<'info>,
    pub borrow_reserve_pubkey: AccountInfo<'info>,
    pub borrow_reserve_liquidity_fee_receiver_pubkey: AccountInfo<'info>,
    pub obligation_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub obligation_owner_pubkey: Signer<'info>,
    pub clock_sysvar: Sysvar<'info, Clock>,
    pub token_program: Program<'info, TokenProgram>,
}

pub fn borrow_obligation_liquidity<'a, 'b, 'c, 'info>(
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
        ctx.accounts
            .borrow_reserve_liquidity_fee_receiver_pubkey
            .key(),
        ctx.accounts.obligation_pubkey.key(),
        ctx.accounts.lending_market_pubkey.key(),
        ctx.accounts.obligation_owner_pubkey.key(),
        host_fee_receiver_pubkey.map(|k| k.key()),
    );
    
    let mut account_infos = vec![
        ctx.accounts.source_liquidity_pubkey.to_account_info(),
        ctx.accounts.destination_liquidity_pubkey.to_account_info(),
        ctx.accounts.borrow_reserve_pubkey.to_account_info(),
        ctx.accounts
            .borrow_reserve_liquidity_fee_receiver_pubkey
            .to_account_info(),
        ctx.accounts.obligation_pubkey.to_account_info(),
        ctx.accounts.lending_market_pubkey.to_account_info(),
        ctx.accounts.obligation_owner_pubkey.to_account_info(),
        ctx.accounts.clock_sysvar.to_account_info(),
        ctx.accounts.token_program.to_account_info()
    ];

    if let Some(host_fee_receiver_pubkey) = host_fee_receiver_pubkey {
        account_infos.push(host_fee_receiver_pubkey.to_account_info());
    }

    solana_program::program::invoke_signed(
        &ix,
        &account_infos,
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct RepayObligationLiquidityAccounts<'info> {
    pub source_liquidity_pubkey: AccountInfo<'info>,
    pub destination_liquidity_pubkey: AccountInfo<'info>,
    pub repay_reserve_pubkey: AccountInfo<'info>,
    pub obligation_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub user_transfer_authority_pubkey: Signer<'info>,
    pub clock_sysvar: Sysvar<'info, Clock>,
    pub token_program: Program<'info, TokenProgram>,
}

pub fn repay_obligation_liquidity<'a, 'b, 'c, 'info>(
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
            ctx.accounts.source_liquidity_pubkey.to_account_info(),
            ctx.accounts.destination_liquidity_pubkey.to_account_info(),
            ctx.accounts.repay_reserve_pubkey.to_account_info(),
            ctx.accounts.obligation_pubkey.to_account_info(),
            ctx.accounts.lending_market_pubkey.to_account_info(),
            ctx.accounts.user_transfer_authority_pubkey.to_account_info(),
            ctx.accounts.clock_sysvar.to_account_info(),
            ctx.accounts.token_program.to_account_info()
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
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
    pub user_transfer_authority_pubkey: Signer<'info>,
    pub clock_sysvar: Sysvar<'info, Clock>,
    pub token_program: Program<'info, TokenProgram>
}

pub fn liquidate_obligation<'a, 'b, 'c, 'info>(
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
            ctx.accounts.source_liquidity_pubkey.to_account_info(),
            ctx.accounts.destination_collateral_pubkey.to_account_info(),
            ctx.accounts.repay_reserve_pubkey.to_account_info(),
            ctx.accounts.repay_reserve_liquidity_supply_pubkey.to_account_info(),
            ctx.accounts.withdraw_reserve_pubkey.to_account_info(),
            ctx.accounts
                .withdraw_reserve_collateral_supply_pubkey
                .to_account_info(),
            ctx.accounts.obligation_pubkey.to_account_info(),
            ctx.accounts.lending_market_pubkey.to_account_info(),
            ctx.accounts.user_transfer_authority_pubkey.to_account_info(),
            ctx.accounts.clock_sysvar.to_account_info(),
            ctx.accounts.token_program.to_account_info()
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct FlashLoanAccounts<'info> {
    pub source_liquidity_pubkey: AccountInfo<'info>,
    pub destination_liquidity_pubkey: AccountInfo<'info>,
    pub reserve_pubkey: AccountInfo<'info>,
    pub reserve_liquidity_fee_receiver_pubkey: AccountInfo<'info>,
    pub host_fee_receiver_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub lending_market_authority_pubkey: AccountInfo<'info>,
    pub flash_loan_receiver_program_id: AccountInfo<'info>,
    pub token_program: Program<'info, TokenProgram>
}
pub fn flash_loan<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'a, 'b, 'c, 'info, FlashLoanAccounts<'info>>,
    amount: u64,
) -> ProgramResult {
    let flash_loan_receiver_program_accounts: Vec<AccountInfo> = ctx.remaining_accounts;
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

    let mut accounts_infos = vec![
        ctx.accounts.source_liquidity_pubkey.to_account_info(),
        ctx.accounts.destination_liquidity_pubkey.to_account_info(),
        ctx.accounts.reserve_pubkey.to_account_info(),
        ctx.accounts.reserve_liquidity_fee_receiver_pubkey.to_account_info(),
        ctx.accounts.host_fee_receiver_pubkey.to_account_info(),
        ctx.accounts.lending_market_pubkey.to_account_info(),
        ctx.accounts.lending_market_authority_pubkey.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.flash_loan_receiver_program_id.to_account_info(),
    ];
    accounts_infos.extend(flash_loan_receiver_program_accounts);

    solana_program::program::invoke_signed(&ix, &accounts_infos, ctx.signer_seeds)
        .map_err(Into::into)
}

#[derive(Accounts)]
pub struct DepositReserveLiquidityAndObligationCollateralAccounts<'info> {
    pub source_liquidity_pubkey: AccountInfo<'info>,
    pub user_collateral_pubkey: AccountInfo<'info>,
    pub reserve_pubkey: AccountInfo<'info>,
    pub reserve_liquidity_supply_pubkey: AccountInfo<'info>,
    pub reserve_collateral_mint_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub lending_market_authority_pubkey: AccountInfo<'info>,
    pub destination_deposit_collateral_pubkey: AccountInfo<'info>,
    pub obligation_pubkey: AccountInfo<'info>,
    pub obligation_owner_pubkey: Signer<'info>,
    pub reserve_liquidity_pyth_oracle_pubkey: AccountInfo<'info>,
    pub reserve_liquidity_switchboard_oracle_pubkey: AccountInfo<'info>,
    pub user_transfer_authority_pubkey: Signer<'info>,
    pub clock_sysvar: Sysvar<'info, Clock>,
    pub token_program: Program<'info, TokenProgram>,
}

pub fn deposit_reserve_liquidity_and_obligation_collateral<'a, 'b, 'c, 'info>(
    ctx: CpiContext<
        'a,
        'b,
        'c,
        'info,
        DepositReserveLiquidityAndObligationCollateralAccounts<'info>,
    >,
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
        ctx.accounts
            .reserve_liquidity_switchboard_oracle_pubkey
            .key(),
        ctx.accounts.user_transfer_authority_pubkey.key(),
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.source_liquidity_pubkey.to_account_info(),
            ctx.accounts.user_collateral_pubkey.to_account_info(),
            ctx.accounts.reserve_pubkey.to_account_info(),
            ctx.accounts.reserve_liquidity_supply_pubkey.to_account_info(),
            ctx.accounts.reserve_collateral_mint_pubkey.to_account_info(),
            ctx.accounts.lending_market_pubkey.to_account_info(),
            ctx.accounts.lending_market_authority_pubkey.to_account_info(),
            ctx.accounts.destination_deposit_collateral_pubkey.to_account_info(),
            ctx.accounts.obligation_pubkey.to_account_info(),
            ctx.accounts.obligation_owner_pubkey.to_account_info(),
            ctx.accounts.reserve_liquidity_pyth_oracle_pubkey.to_account_info(),
            ctx.accounts
                .reserve_liquidity_switchboard_oracle_pubkey
                .to_account_info(),
            ctx.accounts.user_transfer_authority_pubkey.to_account_info(),
            ctx.accounts.clock_sysvar.to_account_info(),
            ctx.accounts.token_program.to_account_info()
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Accounts)]
pub struct UpdateReserveConfigAccounts<'info> {
    pub reserve_pubkey: AccountInfo<'info>,
    pub lending_market_pubkey: AccountInfo<'info>,
    pub lending_market_authority_pubkey: AccountInfo<'info>,
    pub lending_market_owner_pubkey: Signer<'info>,
    pub pyth_product_pubkey: AccountInfo<'info>,
    pub pyth_price_pubkey: AccountInfo<'info>,
    pub switchboard_feed_pubkey: AccountInfo<'info>,
}

pub fn update_reserve_config<'a, 'b, 'c, 'info>(
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
            ctx.accounts.reserve_pubkey.to_account_info(),
            ctx.accounts.lending_market_pubkey.to_account_info(),
            ctx.accounts.lending_market_authority_pubkey.to_account_info(),
            ctx.accounts.lending_market_owner_pubkey.to_account_info(),
            ctx.accounts.pyth_product_pubkey.to_account_info(),
            ctx.accounts.pyth_price_pubkey.to_account_info(),
            ctx.accounts.switchboard_feed_pubkey.to_account_info(),
        ],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

#[derive(Clone)]
pub struct Solend;

impl anchor_lang::Id for Solend {
    fn id() -> Pubkey {
        solend_token_lending::id()
    }
}


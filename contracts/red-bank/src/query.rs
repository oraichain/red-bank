use cosmwasm_std::{Addr, Decimal, Deps, Env, Order, StdError, StdResult, Uint128};
use cw_storage_plus::Bound;

use mars_outpost::address_provider::{self, MarsContract};
use mars_outpost::error::MarsError;
use mars_outpost::red_bank::{
    ConfigResponse, Market, UncollateralizedLoanLimitResponse, UserCollateralResponse,
    UserDebtResponse, UserHealthStatus, UserPositionResponse,
};

use crate::health;
use crate::helpers::{get_bit, get_uncollaterized_debt};
use crate::interest_rates::{
    get_scaled_debt_amount, get_scaled_liquidity_amount, get_underlying_debt_amount,
    get_underlying_liquidity_amount,
};
use crate::state::{
    CONFIG, DEBTS, GLOBAL_STATE, MARKETS, MARKET_DENOMS_BY_MA_TOKEN, UNCOLLATERALIZED_LOAN_LIMITS,
    USERS,
};

const DEFAULT_LIMIT: u32 = 5;
const MAX_LIMIT: u32 = 10;

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    let money_market = GLOBAL_STATE.load(deps.storage)?;

    Ok(ConfigResponse {
        owner: config.owner.to_string(),
        address_provider: config.address_provider.to_string(),
        ma_token_code_id: config.ma_token_code_id,
        market_count: money_market.market_count,
        close_factor: config.close_factor,
    })
}

pub fn query_market(deps: Deps, denom: String) -> StdResult<Market> {
    MARKETS
        .load(deps.storage, &denom)
        .map_err(|_| StdError::generic_err(format!("failed to load market for: {}", denom)))
}

pub fn query_markets(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Vec<Market>> {
    let start = start_after.map(|denom| Bound::ExclusiveRaw(denom.into_bytes()));
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    MARKETS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| {
            let (_, market) = item?;
            Ok(market)
        })
        .collect()
}

pub fn query_uncollateralized_loan_limit(
    deps: Deps,
    user_addr: Addr,
    denom: String,
) -> StdResult<UncollateralizedLoanLimitResponse> {
    let limit = UNCOLLATERALIZED_LOAN_LIMITS.may_load(deps.storage, (&user_addr, &denom))?;
    Ok(UncollateralizedLoanLimitResponse {
        denom,
        limit: limit.unwrap_or_else(Uint128::zero),
    })
}

pub fn query_uncollateralized_loan_limits(
    deps: Deps,
    user_addr: Addr,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Vec<UncollateralizedLoanLimitResponse>> {
    let start = start_after.map(|denom| Bound::ExclusiveRaw(denom.into_bytes()));
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    UNCOLLATERALIZED_LOAN_LIMITS
        .prefix(&user_addr)
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| {
            let (denom, limit) = item?;
            Ok(UncollateralizedLoanLimitResponse {
                denom,
                limit,
            })
        })
        .collect()
}

pub fn query_user_debt(
    deps: Deps,
    env: Env,
    user_addr: Addr,
    denom: String,
) -> StdResult<UserDebtResponse> {
    let market = MARKETS.load(deps.storage, &denom)?;

    let (amount_scaled, amount) = match DEBTS.may_load(deps.storage, (&user_addr, &denom))? {
        Some(debt) => {
            let amount_scaled = debt.amount_scaled;
            let amount =
                get_underlying_debt_amount(amount_scaled, &market, env.block.time.seconds())?;
            (amount_scaled, amount)
        }

        None => (Uint128::zero(), Uint128::zero()),
    };

    Ok(UserDebtResponse {
        denom,
        amount_scaled,
        amount,
    })
}

pub fn query_user_debts(deps: Deps, env: Env, user_addr: Addr) -> StdResult<Vec<UserDebtResponse>> {
    let user = USERS.may_load(deps.storage, &user_addr)?.unwrap_or_default();

    MARKETS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| {
            let (denom, market) = item?;

            let is_borrowing_asset = get_bit(user.borrowed_assets, market.index)?;
            let (amount_scaled, amount) = if is_borrowing_asset {
                let debt = DEBTS.load(deps.storage, (&user_addr, &denom))?;
                let amount_scaled = debt.amount_scaled;
                let amount =
                    get_underlying_debt_amount(amount_scaled, &market, env.block.time.seconds())?;
                (amount_scaled, amount)
            } else {
                (Uint128::zero(), Uint128::zero())
            };

            Ok(UserDebtResponse {
                denom,
                amount_scaled,
                amount,
            })
        })
        .collect()
}

pub fn query_user_collateral(
    deps: Deps,
    user_addr: Addr,
    denom: String,
) -> StdResult<UserCollateralResponse> {
    let user = USERS.may_load(deps.storage, &user_addr)?.unwrap_or_default();
    let market = MARKETS.load(deps.storage, &denom)?;
    Ok(UserCollateralResponse {
        denom,
        enabled: get_bit(user.collateral_assets, market.index)?,
    })
}

pub fn query_user_collaterals(
    deps: Deps,
    user_addr: Addr,
) -> StdResult<Vec<UserCollateralResponse>> {
    let user = USERS.may_load(deps.storage, &user_addr)?.unwrap_or_default();

    MARKETS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| {
            let (denom, market) = item?;
            Ok(UserCollateralResponse {
                denom,
                enabled: get_bit(user.collateral_assets, market.index)?,
            })
        })
        .collect()
}

pub fn query_scaled_liquidity_amount(
    deps: Deps,
    env: Env,
    denom: String,
    amount: Uint128,
) -> StdResult<Uint128> {
    let market = MARKETS.load(deps.storage, &denom)?;
    get_scaled_liquidity_amount(amount, &market, env.block.time.seconds())
}

pub fn query_scaled_debt_amount(
    deps: Deps,
    env: Env,
    denom: String,
    amount: Uint128,
) -> StdResult<Uint128> {
    let market = MARKETS.load(deps.storage, &denom)?;
    get_scaled_debt_amount(amount, &market, env.block.time.seconds())
}

pub fn query_underlying_liquidity_amount(
    deps: Deps,
    env: Env,
    ma_token: String,
    amount_scaled: Uint128,
) -> StdResult<Uint128> {
    let ma_token_addr = deps.api.addr_validate(&ma_token)?;
    let denom = MARKET_DENOMS_BY_MA_TOKEN.load(deps.storage, &ma_token_addr)?;
    let market = MARKETS.load(deps.storage, &denom)?;
    get_underlying_liquidity_amount(amount_scaled, &market, env.block.time.seconds())
}

pub fn query_underlying_debt_amount(
    deps: Deps,
    env: Env,
    denom: String,
    amount_scaled: Uint128,
) -> StdResult<Uint128> {
    let market = MARKETS.load(deps.storage, &denom)?;
    get_underlying_debt_amount(amount_scaled, &market, env.block.time.seconds())
}

pub fn query_user_position(
    deps: Deps,
    env: Env,
    user_addr: Addr,
) -> Result<UserPositionResponse, MarsError> {
    let config = CONFIG.load(deps.storage)?;
    let user = USERS.may_load(deps.storage, &user_addr)?.unwrap_or_default();
    let oracle_addr = address_provider::helpers::query_address(
        deps,
        &config.address_provider,
        MarsContract::Oracle,
    )?;
    let positions = health::get_user_positions_map(&deps, &env, &user, &user_addr, &oracle_addr)?;

    let health = health::compute_position_health(&positions)?;
    let health_status = if let (Some(max_ltv_hf), Some(liq_threshold_hf)) =
        (health.max_ltv_health_factor, health.liquidation_health_factor)
    {
        UserHealthStatus::Borrowing {
            max_ltv_hf,
            liq_threshold_hf,
        }
    } else {
        UserHealthStatus::NotBorrowing
    };

    let total_uncollateralized_debt = get_uncollaterized_debt(&positions)?;

    Ok(UserPositionResponse {
        total_collateral_value: health.total_collateral_value,
        total_debt_value: health.total_debt_value
            + Decimal::from_ratio(total_uncollateralized_debt, 1u128),
        total_collateralized_debt: health.total_debt_value,
        weighted_max_ltv_collateral: health.max_ltv_adjusted_collateral,
        weighted_liquidation_threshold_collateral: health.liquidation_threshold_adjusted_collateral,
        health_status,
    })
}
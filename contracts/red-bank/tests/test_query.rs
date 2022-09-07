use cosmwasm_std::{Addr, Decimal, Uint128};

use mars_outpost::red_bank::{Debt, Market, User, UserDebtResponse};
use mars_testing::{mock_env, MockEnvParams};

use mars_red_bank::helpers::set_bit;
use mars_red_bank::interest_rates::{get_scaled_debt_amount, get_underlying_debt_amount};
use mars_red_bank::query::{query_user_collaterals, query_user_debt, query_user_debts};
use mars_red_bank::state::{DEBTS, USERS};

use helpers::{th_init_market, th_setup};

mod helpers;

#[test]
fn test_query_collateral() {
    let mut deps = th_setup(&[]);

    let user_addr = Addr::unchecked("user");

    // Setup first market containing a CW20 asset
    let market_1_initial = th_init_market(
        deps.as_mut(),
        "uosmo",
        &Market {
            ..Default::default()
        },
    );

    // Setup second market containing a native asset
    let market_2_initial = th_init_market(
        deps.as_mut(),
        "uusd",
        &Market {
            ..Default::default()
        },
    );

    // Set second market as collateral
    let mut user = User::default();
    set_bit(&mut user.collateral_assets, market_2_initial.index).unwrap();
    USERS.save(deps.as_mut().storage, &user_addr, &user).unwrap();

    // Assert markets correctly return collateral status
    let collaterals = query_user_collaterals(deps.as_ref(), user_addr.clone()).unwrap();
    assert_eq!(collaterals[0].denom, String::from("uosmo"));
    assert!(!collaterals[0].enabled);
    assert_eq!(collaterals[1].denom, String::from("uusd"));
    assert!(collaterals[1].enabled);

    // Set first market as collateral
    set_bit(&mut user.collateral_assets, market_1_initial.index).unwrap();
    USERS.save(deps.as_mut().storage, &user_addr, &user).unwrap();

    // Assert markets correctly return collateral status
    let collaterals = query_user_collaterals(deps.as_ref(), user_addr).unwrap();
    assert_eq!(collaterals[0].denom, String::from("uosmo"));
    assert!(collaterals[0].enabled);
    assert_eq!(collaterals[1].denom, String::from("uusd"));
    assert!(collaterals[1].enabled);
}

#[test]
fn test_query_user_debt() {
    let mut deps = th_setup(&[]);

    let user_addr = Addr::unchecked("user");

    // Setup markets
    let market_1_initial = th_init_market(
        deps.as_mut(),
        "coin_1",
        &Market {
            borrow_index: Decimal::one(),
            borrow_rate: Decimal::one(),
            ..Default::default()
        },
    );
    let _market_2_initial = th_init_market(
        deps.as_mut(),
        "coin_2",
        &Market {
            borrow_index: Decimal::one(),
            borrow_rate: Decimal::one(),
            ..Default::default()
        },
    );
    let market_3_initial = th_init_market(
        deps.as_mut(),
        "coin_3",
        &Market {
            borrow_index: Decimal::one(),
            borrow_rate: Decimal::one(),
            ..Default::default()
        },
    );

    // Set first and third market as borrowing assets
    let mut user = User::default();
    set_bit(&mut user.borrowed_assets, market_1_initial.index).unwrap();
    set_bit(&mut user.borrowed_assets, market_3_initial.index).unwrap();
    USERS.save(deps.as_mut().storage, &user_addr, &user).unwrap();

    let env = mock_env(MockEnvParams::default());

    // Save debt for market 1
    let debt_amount_1 = Uint128::new(1234000u128);
    let debt_amount_scaled_1 =
        get_scaled_debt_amount(debt_amount_1, &market_1_initial, env.block.time.seconds()).unwrap();
    let debt_amount_at_query_1 = get_underlying_debt_amount(
        debt_amount_scaled_1,
        &market_1_initial,
        env.block.time.seconds(),
    )
    .unwrap();
    let debt_1 = Debt {
        amount_scaled: debt_amount_scaled_1,
        uncollateralized: false,
    };
    DEBTS.save(deps.as_mut().storage, (&user_addr, "coin_1"), &debt_1).unwrap();

    // Save debt for market 3
    let debt_amount_3 = Uint128::new(2221u128);
    let debt_amount_scaled_3 =
        get_scaled_debt_amount(debt_amount_3, &market_3_initial, env.block.time.seconds()).unwrap();
    let debt_amount_at_query_3 = get_underlying_debt_amount(
        debt_amount_scaled_3,
        &market_3_initial,
        env.block.time.seconds(),
    )
    .unwrap();
    let debt_3 = Debt {
        amount_scaled: debt_amount_scaled_3,
        uncollateralized: false,
    };
    DEBTS.save(deps.as_mut().storage, (&user_addr, "coin_3"), &debt_3).unwrap();

    let debts = query_user_debts(deps.as_ref(), env, user_addr).unwrap();
    assert_eq!(
        debts[0],
        UserDebtResponse {
            denom: "coin_1".to_string(),
            amount_scaled: debt_amount_scaled_1,
            amount: debt_amount_at_query_1,
        }
    );
    assert_eq!(
        debts[1],
        UserDebtResponse {
            denom: "coin_2".to_string(),
            amount_scaled: Uint128::zero(),
            amount: Uint128::zero()
        }
    );
    assert_eq!(
        debts[2],
        UserDebtResponse {
            denom: "coin_3".to_string(),
            amount_scaled: debt_amount_scaled_3,
            amount: debt_amount_at_query_3
        }
    );
}

#[test]
fn test_query_user_asset_debt() {
    let mut deps = th_setup(&[]);

    let user_addr = Addr::unchecked("user");

    // Setup markets
    let market_1_initial = th_init_market(
        deps.as_mut(),
        "coin_1",
        &Market {
            borrow_index: Decimal::one(),
            borrow_rate: Decimal::one(),
            ..Default::default()
        },
    );
    let _market_2_initial = th_init_market(
        deps.as_mut(),
        "coin_2",
        &Market {
            borrow_index: Decimal::one(),
            borrow_rate: Decimal::one(),
            ..Default::default()
        },
    );

    // Set the first as borrowing asset
    let mut user = User::default();
    set_bit(&mut user.borrowed_assets, market_1_initial.index).unwrap();
    USERS.save(deps.as_mut().storage, &user_addr, &user).unwrap();

    let env = mock_env(MockEnvParams::default());

    // Save debt for market 1
    let debt_amount_1 = Uint128::new(1234567u128);
    let debt_amount_scaled_1 =
        get_scaled_debt_amount(debt_amount_1, &market_1_initial, env.block.time.seconds()).unwrap();
    let debt_amount_at_query_1 = get_underlying_debt_amount(
        debt_amount_scaled_1,
        &market_1_initial,
        env.block.time.seconds(),
    )
    .unwrap();
    let debt_1 = Debt {
        amount_scaled: debt_amount_scaled_1,
        uncollateralized: false,
    };
    DEBTS.save(deps.as_mut().storage, (&user_addr, "coin_1"), &debt_1).unwrap();

    // Check asset with existing debt
    {
        let res =
            query_user_debt(deps.as_ref(), env.clone(), user_addr.clone(), "coin_1".to_string())
                .unwrap();
        assert_eq!(
            res,
            UserDebtResponse {
                denom: "coin_1".to_string(),
                amount_scaled: debt_amount_scaled_1,
                amount: debt_amount_at_query_1
            }
        );
    }

    // Check asset with no debt
    {
        let res = query_user_debt(deps.as_ref(), env, user_addr, "coin_2".to_string()).unwrap();
        assert_eq!(
            res,
            UserDebtResponse {
                denom: "coin_2".to_string(),
                amount_scaled: Uint128::zero(),
                amount: Uint128::zero()
            }
        );
    }
}
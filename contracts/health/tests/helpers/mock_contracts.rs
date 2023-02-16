use cosmwasm_std::Empty;
use cw_multi_test::{Contract, ContractWrapper};

pub fn mock_health_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        mars_rover_health::contract::execute,
        mars_rover_health::contract::instantiate,
        mars_rover_health::contract::query,
    );
    Box::new(contract)
}

pub fn mock_credit_manager_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        mars_mock_credit_manager::contract::execute,
        mars_mock_credit_manager::contract::instantiate,
        mars_mock_credit_manager::contract::query,
    );
    Box::new(contract)
}

pub fn mock_vault_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        mars_mock_vault::contract::execute,
        mars_mock_vault::contract::instantiate,
        mars_mock_vault::contract::query,
    );
    Box::new(contract)
}

pub fn mock_oracle_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        mars_mock_oracle::contract::execute,
        mars_mock_oracle::contract::instantiate,
        mars_mock_oracle::contract::query,
    );
    Box::new(contract)
}

pub fn mock_red_bank_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        mars_mock_red_bank::contract::execute,
        mars_mock_red_bank::contract::instantiate,
        mars_mock_red_bank::contract::query,
    );
    Box::new(contract)
}

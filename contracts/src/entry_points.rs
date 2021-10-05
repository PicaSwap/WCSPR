use alloc::{string::String, vec};

use casper_erc20::entry_points;

use casper_types::{
    URef, U512, U256, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter,
};

pub fn deposit() -> EntryPoint {
    EntryPoint::new(
        String::from("deposit"),
        vec![
            Parameter::new("tmp_purse", URef::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn withdraw() -> EntryPoint {
    EntryPoint::new(
        String::from("withdraw"),
        vec![
            Parameter::new("cspr_amount", U512::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn init() -> EntryPoint {
    EntryPoint::new(
        String::from("init"),
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn contract_cspr_balance() -> EntryPoint {
    EntryPoint::new(
        String::from("contract_cspr_balance"),
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn default() -> EntryPoints {
    let mut contract_entry_points = entry_points::default();
    contract_entry_points.add_entry_point(deposit());
    contract_entry_points.add_entry_point(withdraw());
    contract_entry_points.add_entry_point(contract_cspr_balance());
    contract_entry_points.add_entry_point(init());
    contract_entry_points
}
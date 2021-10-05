#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

mod entry_points;

use casper_types::RuntimeArgs;
use casper_types::ContractHash;
use casper_types::HashAddr;
use casper_types::Key;
use alloc::string::String;
use core::convert::TryInto;

use casper_contract::{contract_api::{runtime, storage, system}, unwrap_or_revert::UnwrapOrRevert};
use casper_erc20::{
    constants::{
        ADDRESS_RUNTIME_ARG_NAME, AMOUNT_RUNTIME_ARG_NAME, DECIMALS_RUNTIME_ARG_NAME,
        NAME_RUNTIME_ARG_NAME, OWNER_RUNTIME_ARG_NAME, RECIPIENT_RUNTIME_ARG_NAME,
        SPENDER_RUNTIME_ARG_NAME, SYMBOL_RUNTIME_ARG_NAME, TOTAL_SUPPLY_RUNTIME_ARG_NAME,
    },
    Address, ERC20, Error
};
use casper_types::{CLValue, CLTyped, U256, URef, U512, bytesrepr::{FromBytes, ToBytes}, system::CallStackElement};

// TODO: understand why changing the name makes everything crash
//const CONTRACT_KEY_NAME: &str = "wcspr_token_contract";
const CONTRACT_KEY_NAME: &str = "erc20_token_contract";

#[no_mangle]
pub extern "C" fn name() {
    let name = ERC20::default().name();
    runtime::ret(CLValue::from_t(name).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn symbol() {
    let symbol = ERC20::default().symbol();
    runtime::ret(CLValue::from_t(symbol).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn decimals() {
    let decimals = ERC20::default().decimals();
    runtime::ret(CLValue::from_t(decimals).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn total_supply() {
    let total_supply = ERC20::default().total_supply();
    runtime::ret(CLValue::from_t(total_supply).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let address: Address = runtime::get_named_arg(ADDRESS_RUNTIME_ARG_NAME);
    let balance = ERC20::default().balance_of(address);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn transfer() {
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    ERC20::default()
        .transfer(recipient, amount)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn approve() {
    let spender: Address = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    ERC20::default().approve(spender, amount).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn allowance() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let spender: Address = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);
    let val = ERC20::default().allowance(owner, spender);
    runtime::ret(CLValue::from_t(val).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn transfer_from() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    ERC20::default()
        .transfer_from(owner, recipient, amount)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn deposit() {

    // Get passed purse from pre_deposit
    let tmp_purse: URef = runtime::get_named_arg("tmp_purse");

    let cspr_amount: U512 = system::get_purse_balance(tmp_purse).unwrap_or_revert();
    let cspr_amount_u256: U256 = U256::from(cspr_amount.as_u128());

    // TODO: why is it not working???
    //let contract_main_purse: URef = get_key("main_purse").unwrap_or_revert();

    let contract_main_purse_key = runtime::get_key("main_purse").unwrap_or_revert();
    let contract_main_purse = contract_main_purse_key.as_uref().unwrap_or_revert();

    // TODO: this line causes test crash with ForgedReference URef
    // Save CSPR provided by user into our contract
    let _ = system::transfer_from_purse_to_purse(tmp_purse, *contract_main_purse, cspr_amount, None);

    // Get account of the user who called the contract
    let sender = get_immediate_caller_address().unwrap_or_revert();

    // Issue WCSPR tokens to the sender
    ERC20::default().mint(sender, cspr_amount_u256).unwrap_or_revert()

}

#[no_mangle]
pub extern "C" fn withdraw() {
    // how many wcspr tokens to withdraw
    let cspr_amount: U512 = runtime::get_named_arg("cspr_amount");
    let cspr_amount_u256: U256 = U256::from(cspr_amount.as_u128());

    // Get account of the user who called the contract
    let sender = get_immediate_caller_address().unwrap_or_revert();

    let balance = ERC20::default().balance_of(sender);

    let contract_main_purse_key = runtime::get_key("main_purse").unwrap_or_revert();
    let contract_main_purse = contract_main_purse_key.as_uref().unwrap_or_revert();

    if balance >= cspr_amount_u256 {
        system::transfer_from_purse_to_account(
            *contract_main_purse, 
            *sender.as_account_hash().unwrap_or_revert(), 
            cspr_amount, 
            None
        ).unwrap_or_revert();
        ERC20::default().burn(sender, cspr_amount_u256).unwrap_or_revert();
    }
}

#[no_mangle]
pub extern "C" fn init() {
    let value: Option<bool> = get_key("initialized"); 
    match value {
        Some(_) => {},
        None => {
            set_key("main_purse", system::create_purse());
            set_key("initialized", true);
        },
    }
}

#[no_mangle]
fn call() {
    let name: String = runtime::get_named_arg(NAME_RUNTIME_ARG_NAME);
    let symbol: String = runtime::get_named_arg(SYMBOL_RUNTIME_ARG_NAME);
    let decimals = runtime::get_named_arg(DECIMALS_RUNTIME_ARG_NAME);
    let initial_supply = runtime::get_named_arg(TOTAL_SUPPLY_RUNTIME_ARG_NAME);

    let _ = ERC20::install_custom(
        name,
        symbol,
        decimals,
        initial_supply,
        CONTRACT_KEY_NAME,
        entry_points::default(),
    );

    let key: Key = runtime::get_key(CONTRACT_KEY_NAME).unwrap_or_revert();
    let hash: HashAddr = key.into_hash().unwrap_or_revert();
    let contract_hash = ContractHash::new(hash);

    let _: () = runtime::call_contract(contract_hash, "init", RuntimeArgs::new());

}


// Helper functions


fn get_key<T: FromBytes + CLTyped>(name: &str) -> Option<T> {
    match runtime::get_key(name) {
        None => None,
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            let result = storage::read(key).unwrap_or_revert().unwrap_or_revert();
            Some(result)
        }
    }
}

fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

/// Gets the immediate call stack element of the current execution.
fn get_immediate_call_stack_item() -> Option<CallStackElement> {
    let call_stack = runtime::get_call_stack();
    call_stack.into_iter().rev().nth(1)
}

/// Returns address based on a [`CallStackElement`].
///
/// For `Session` and `StoredSession` variants it will return account hash, and for `StoredContract`
/// case it will use contract hash as the address.
fn call_stack_element_to_address(call_stack_element: CallStackElement) -> Address {
    match call_stack_element {
        CallStackElement::Session { account_hash } => Address::from(account_hash),
        CallStackElement::StoredSession { account_hash, .. } => {
            // Stored session code acts in account's context, so if stored session wants to interact
            // with an ERC20 token caller's address will be used.
            Address::from(account_hash)
        }
        CallStackElement::StoredContract {
            contract_package_hash,
            ..
        } => Address::from(contract_package_hash),
    }
}

/// Gets the immediate session caller of the current execution.
///
/// This function ensures that only session code can execute this function, and disallows stored
/// session/stored contracts.
pub(crate) fn get_immediate_caller_address() -> Result<Address, Error> {
    get_immediate_call_stack_item()
        .map(call_stack_element_to_address)
        .ok_or(Error::InvalidContext)
}

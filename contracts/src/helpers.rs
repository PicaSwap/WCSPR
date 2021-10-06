use casper_erc20::Error;
use casper_erc20::Address;
use casper_types::bytesrepr::ToBytes;
use casper_types::CLTyped;
use casper_types::bytesrepr::FromBytes;
use casper_contract::{contract_api::{runtime, storage}, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{Key, URef, system::CallStackElement};
use core::convert::TryInto;

// Helper functions

pub fn set_main_purse(purse: URef) {
    runtime::put_key("main_purse", Key::from(purse))
}

pub fn get_main_purse() -> URef {
    let contract_main_purse_key = runtime::get_key("main_purse").unwrap_or_revert();
    let contract_main_purse = contract_main_purse_key.as_uref().unwrap_or_revert();
    *contract_main_purse 
}

pub fn get_key<T: FromBytes + CLTyped>(name: &str) -> Option<T> {
    match runtime::get_key(name) {
        None => None,
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            let result = storage::read(key).unwrap_or_revert().unwrap_or_revert();
            Some(result)
        }
    }
}

pub fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
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
pub fn get_immediate_call_stack_item() -> Option<CallStackElement> {
    let call_stack = runtime::get_call_stack();
    call_stack.into_iter().rev().nth(1)
}

/// Returns address based on a [`CallStackElement`].
///
/// For `Session` and `StoredSession` variants it will return account hash, and for `StoredContract`
/// case it will use contract hash as the address.
pub fn call_stack_element_to_address(call_stack_element: CallStackElement) -> Address {
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
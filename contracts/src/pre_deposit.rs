#![no_main]

use casper_contract::{
    contract_api::{account, runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_erc20::Address;
/*
use casper_erc20::Error;
use casper_types::system::CallStackElement;
*/
use casper_types::RuntimeArgs;
use casper_types::{runtime_args, ApiError, ContractHash, HashAddr, Key, URef, U256, U512};
pub enum DepositError {
    ExceedUserLimit = 0,
    ExceedContractLimit = 1,
}

impl From<DepositError> for ApiError {
    fn from(error: DepositError) -> ApiError {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
fn call() {
    // This function is used to accept CSPR from the user and pass to the
    // main contract that going to swap it to WCSPR
    // We need it because casper don't allow us to access `get_main_purse` from main contract

    // how many cspr tokens to transfer
    let cspr_amount: U512 = runtime::get_named_arg("cspr_amount");
    let cspr_amount_u256: U256 = U256::from(cspr_amount.as_u128());

    // WCSPR contract hash address passed as an argument to this contract
    let wcspr_contract_key: Key = runtime::get_named_arg("wcspr_contract_hash_key");
    let _wcspr_contract_hash: HashAddr = wcspr_contract_key.into_hash().unwrap_or_revert();
    let wcspr_contract_hash: ContractHash = ContractHash::new(_wcspr_contract_hash);

    // Get total WCSPR supply
    let total_supply: U256 =
        runtime::call_contract::<U256>(wcspr_contract_hash, "total_supply", runtime_args! {});

    // Stop deposit if total supply exceed contract limits
    // 200 CSPR (200 * 10**9)
    // let multiplier: U256 = U256:from(10.pow(9));
    let multiplier: U256 = (U256::from(10)).pow(U256::from(9));
    let contract_limit: U256 = U256::from(200) * multiplier;

    if (total_supply + cspr_amount_u256) > contract_limit {
        runtime::revert(DepositError::ExceedContractLimit)
    };

    // Get Address (AccountHash) of the user who called the contract
    let sender: Address = Address::from(runtime::get_caller());

    // Get user's WCSPR balance
    let sender_wcspr_balance: U256 = runtime::call_contract::<U256>(
        wcspr_contract_hash,
        "balance_of",
        runtime_args! {
            "address" => sender
        },
    );

    // Stop deposit if user exceeds personal limits
    // 100 CSPR (100 * 10**9)
    let user_limit: U256 = U256::from(100) * multiplier;

    if (sender_wcspr_balance + cspr_amount_u256) > user_limit {
        runtime::revert(DepositError::ExceedUserLimit)
    };

    // Purse with CSPR tokens of the user who call the contract
    let sender_purse: URef = account::get_main_purse();

    // Here we put tokens we want to transfer to the contract
    let tmp_purse: URef = system::create_purse();

    // move from sender to tmp purse, so we can use tmp purse in the next contract
    let _ = system::transfer_from_purse_to_purse(sender_purse, tmp_purse, cspr_amount, None);

    runtime::call_contract(
        wcspr_contract_hash,
        "deposit",
        runtime_args! {
            "tmp_purse" => tmp_purse
        },
    )
}

/*
/// Gets the first call stack element of the current execution.
pub fn get_first_call_stack_item() -> Option<CallStackElement> {
    let call_stack = runtime::get_call_stack();
    call_stack.into_iter().rev().nth(0)
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

pub(crate) fn get_first_caller_address_() -> Result<Address, Error> {
    get_first_call_stack_item()
        .map(call_stack_element_to_address)
        .ok_or(Error::InvalidContext)
}

#[no_mangle]
fn call() {
    let sender: Address = get_first_caller_address_().unwrap_or_revert();
}
*/

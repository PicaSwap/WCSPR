#![no_main]

use casper_contract::{contract_api::{account, runtime, system}, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{ContractHash, HashAddr, Key, U512, URef, runtime_args};
use casper_types::RuntimeArgs;


#[no_mangle]
fn call() {
    // This function is used to accept CSPR from the user and pass to the 
    // main contract that going to swap it to WCSPR
    // We need it because casper don't allow us to access `get_main_purse` from main contract

    // Purse with CSPR tokens of the user who call the contract
    let sender_purse: URef = account::get_main_purse(); 

    // Here we put tokens we want to transfer to the contract
    let tmp_purse: URef = system::create_purse(); 
    
    // how many cspr tokens to transfer
    let cspr_amount: U512 = runtime::get_named_arg("cspr_amount");

    // move from sender to tmp purse, so we can use tmp purse in the next contract
    let _ = system::transfer_from_purse_to_purse(sender_purse, tmp_purse, cspr_amount, None);

    // WCSPR contract hash address passed as an argument to this contract
    let wcspr_contract_key: Key = runtime::get_named_arg("wcspr_contract_hash_key");
    let _wcspr_contract_hash: HashAddr  = wcspr_contract_key.into_hash().unwrap_or_revert();
    let wcspr_contract_hash: ContractHash = ContractHash::new(_wcspr_contract_hash);
    runtime::call_contract(wcspr_contract_hash, "deposit", runtime_args!{
        "tmp_purse" => tmp_purse
    })

}
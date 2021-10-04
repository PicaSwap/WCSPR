#[cfg(test)]
mod tests {
    
    use casper_engine_test_support::{Code, SessionBuilder, TestContextBuilder};
    use casper_types::{account::AccountHash, runtime_args, PublicKey, RuntimeArgs, U512, AsymmetricType};

    const CONTRACT_PRE_DEPOSIT: &str = "pre_deposit.wasm";

    #[test]
    fn call_should_work() {
        let cspr_amount:U512 = U512::from(10_000_000_000u64);
        //const wcspr_contract_hash:ContractHash = ;
        
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();

        let mut context = TestContextBuilder::new()
            .with_public_key(ali.clone(), U512::from(500_000_000_000_000_000u64))
            .build();

        let session_code = Code::from(CONTRACT_PRE_DEPOSIT);
        let session_args = runtime_args! {
            "cspr_amount" => cspr_amount
            //"wcspr_contract_hash" => wcspr_contract_hash
        };

        let session = SessionBuilder::new(session_code, session_args)
            .with_address(ali.to_account_hash())
            .with_authorization_keys(&[ali.to_account_hash()])
            .build();

        context.run(session);
        let ali_account_hash: AccountHash = ali.to_account_hash();
    }

}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}

#[cfg(test)]
mod tests {
    use casper_engine_test_support::{Code, SessionBuilder, TestContext, TestContextBuilder};
    use casper_types::{RuntimeArgs, runtime_args, U512, PublicKey};

    #[test]
    fn call_should_work() {
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let mut context = TestContextBuilder::new()
            .with_public_key(ali.clone(), U512::from(500_000_000_000_000_000u64))
            .build();

        let pre_deposit_code = Code::from("pre_deposit.wasm");
        let session_args = runtime_args! {
        };
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(MY_ACCOUNT)
            .with_authorization_keys(&[MY_ACCOUNT])
            .build();
        context.run(session);

    }

}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}

prepare:
	rustup target add wasm32-unknown-unknown

build-erc20:
	cd wcspr && cargo build --release --target wasm32-unknown-unknown
	wasm-strip wcspr/target/wasm32-unknown-unknown/release/wcspr.wasm 2>/dev/null | true

test: build-erc20
	mkdir -p tests/wasm
	cp wcspr/target/wasm32-unknown-unknown/release/wcspr.wasm tests/wasm
	cp wcspr/target/wasm32-unknown-unknown/release/pre_deposit.wasm tests/wasm
	cd tests && cargo test

clippy:
	cd wcspr && cargo clippy --all-targets -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd wcspr && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cd wcspr && cargo fmt
	cd tests && cargo fmt

clean:
	cd wcspr && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm

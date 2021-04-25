echo "checks that this builds on std+no_std + that all tests run + that all features compile"

# make sure that this builds on Windows too => add targets first
rustup target add x86_64-unknown-linux-gnu
rustup target add thumbv7em-none-eabihf

cargo check --all-targets --target x86_64-unknown-linux-gnu
cargo check --target thumbv7em-none-eabihf --no-default-features # no_std, not for all targets because example needs std

# doesn't work on Windows
cargo test --all-targets --target x86_64-unknown-linux-gnu


source get_env.sh

cargo build --target x86_64-unknown-linux-gnu --release
mkdir -p dist/x86_64-unknown-linux-gnu/
cp target/x86_64-unknown-linux-gnu/release/mnemonic-split-otp dist/x86_64-unknown-linux-gnu/

cargo build --release
mkdir -p dist/local/
cp target/release/mnemonic-split-otp dist/local/

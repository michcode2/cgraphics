rm -rf builds
mkdir builds
cargo clean
echo mac arm
cargo build --release --target=aarch64-apple-darwin
cp target/aarch64-apple-darwin/release/cgraphics builds/mac-arm
echo mac intel
cargo build --release --target=x86_64-apple-darwin
cp target/x86_64-apple-darwin/release/cgraphics builds/mac-intel
echo windows intel
cargo build --release --target=x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/cgraphics builds/windows-intel
echo linux intel 64
cargo build --release --target=x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/cgraphics builds/linux-intel64
echo linux intel 32
cargo build --release --target=i686-unknown-linux-gnu
cp target/i686-unknown-linux-gnu/release/cgraphics builds/linux-intel64

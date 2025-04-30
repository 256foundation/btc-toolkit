run:
	cargo run

build:
	cargo build --release --target x86_64-unknown-linux-gnu
	cargo build --release --target x86_64-pc-windows-gnu # On Linux Systems Requires The `gcc-mingw-w64` Dep For `mimalloc`

clean:
	cargo clean

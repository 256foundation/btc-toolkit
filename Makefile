run:
	cargo run

install:
	cargo install --path .

build-linux:
	cargo build --release --target x86_64-unknown-linux-gnu

build-windows:
# On Linux Systems Requires The `gcc-mingw-w64` Dependency For `mimalloc`
	cargo build --release --target x86_64-pc-windows-gnu

clean:
	cargo clean

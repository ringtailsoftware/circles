all:
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/macroquad.wasm circles.wasm

run:
	cargo install basic-http-server
	basic-http-server .

clean:
	rm -f Cargo.lock
	rm -rf target

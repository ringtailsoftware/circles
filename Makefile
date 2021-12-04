all:
	rm -rf demo
	mkdir -p demo
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/macroquad.wasm demo/circles.wasm
	cp index.html demo
	cp mq_js_bundle.js demo

run:
	cargo install basic-http-server
	basic-http-server demo

clean:
	rm -f Cargo.lock
	rm -rf demo
	rm -rf target

all:
	rm -rf artefacts
	mkdir -p artefacts
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/macroquad.wasm artefacts/circles.wasm
	cp index.html artefacts
	cp mq_js_bundle.js artefacts

run:
	cargo install basic-http-server
	basic-http-server artefacts

clean:
	rm -f Cargo.lock
	rm -rf artefacts
	rm -rf target

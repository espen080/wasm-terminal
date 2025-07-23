build:
	wasm-pack build --target web --no-typescript --no-pack --release

serve:
	python3 -m http.server


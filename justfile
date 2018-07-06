watch:
	cargo watch -x fmt -x test

serve:
	cd static && ../bin/serve

wat target:
	wat2wasm misc/{{target}}.wat -o misc/{{target}}.wasm
	wasm-interp misc/{{target}}.wasm --run-all-exports --host-print

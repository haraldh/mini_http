cargo +nightly build --target wasm32-wasi --example hello_wasi
wasmtime run --tcplisten 127.0.0.1:3000 --env 'FD_COUNT=1' target/wasm32-wasi/debug/examples/hello_wasi.wasm

# Wasmtime Demo — Running Wasm, WASI, and Host Imports

This demo showcases:
- Loading and running a WAT module exporting `add`.
- Running a Rust-compiled WASI module with sandboxed filesystem access.
- Importing a host function and calling it from WebAssembly.
- Measuring cold vs warm compile times and runtime.

## Quickstart (Local)

```bash
# Ensure Rust toolchain and wasm32-wasi target
rustup target add wasm32-wasi

# Build the WASI module
cargo build --release --manifest-path demo/wasi/Cargo.toml --target wasm32-wasi

# Run the demo host
cargo run --release --manifest-path demo/host/Cargo.toml -- --wasi demo/wasi/target/wasm32-wasi/release/wasi_demo.wasm --save
```

Artifacts will be in `demo/out/`:
- `timing.json`: compile/instantiate/run timing data

## What You'll See

- `add.wat` compiled and called from Rust via Wasmtime.
- WASI program prints `/data/input.txt` and environment variable `WASI_DEMO_ENV`.
- Host import demo prints from WebAssembly through a host-provided `log` function.

## Notes

- The demo uses the `wasmtime` crate from crates.io within `demo/host`.
- For WASI, we preopen `demo/data` as `/data` and inherit stdio/env.
- You can also run via CLI:
  ```bash
  wasmtime demo/wasi/target/wasm32-wasi/release/wasi_demo.wasm --dir=demo/data --env=WASI_DEMO_ENV=hello
  ```

## Troubleshooting

- Missing `wasm32-wasi` target: run `rustup target add wasm32-wasi`.
- Windows path issues: use PowerShell-friendly paths; ensure Rust toolchain installed.
- If `wasmtime` CLI is missing, install with `cargo install wasmtime-cli`.

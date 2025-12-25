# Dev container with Rust toolchain and wasm32-wasi target
FROM mcr.microsoft.com/devcontainers/rust:1

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y wget gnupg lsb-release software-properties-common && \
    # Useful tools for WASM/WASI workflows
    apt-get install -y pkg-config libssl-dev && \
    rustup target add wasm32-wasip1 && \
    # Optional tools - install if possible, but don't fail the build
    (cargo install wasmtime-cli --locked 2>&1 | tee /tmp/wasmtime-install.log || echo "wasmtime-cli installation skipped") && \
    (cargo install wasm-tools --locked 2>&1 | tee /tmp/wasm-tools-install.log || echo "wasm-tools installation skipped") && \
    (cargo install wit-bindgen-cli --locked 2>&1 | tee /tmp/wit-bindgen-install.log || echo "wit-bindgen-cli installation skipped") && \
    rm -rf /var/lib/apt/lists/*

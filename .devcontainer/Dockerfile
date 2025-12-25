# Dev container with Rust toolchain and wasm32-wasi target
FROM mcr.microsoft.com/devcontainers/rust:1

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y wget gnupg lsb-release software-properties-common && \
    # Useful tools for WASM/WASI workflows
    apt-get install -y pkg-config libssl-dev && \
    rustup target add wasm32-wasip1 && \
    cargo install wasmtime-cli --locked || true && \
    cargo install wasm-tools --locked || true && \
    cargo install wit-bindgen-cli --locked || true && \
    rm -rf /var/lib/apt/lists/*

# Use the official Rust image as the base
FROM rust:1.84-bookworm

# Install system dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    libssl-dev \
    libgcc-s1 \
    libstdc++6 \
    curl \
    clang \
    lld \
    && rm -rf /var/lib/apt/lists/*

# Add the wasm32-unknown-unknown target for Rust
RUN rustup target add wasm32-unknown-unknown

# Install TailwindCSS
RUN mkdir -p /root/.local/bin && \
    curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v4.0.6/tailwindcss-linux-x64 && \
    chmod +x tailwindcss-linux-x64 && \
    mv tailwindcss-linux-x64 /root/.local/bin/tailwindcss

# Install Trunk using cargo-binstall
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash && \
    cargo binstall trunk --no-confirm

# Set up the working directory
WORKDIR /app

# Set the PATH to include .local/bin
ENV PATH="/root/.local/bin:${PATH}"


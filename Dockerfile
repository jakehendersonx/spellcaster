 FROM ubuntu:22.04

# Install dependencies
RUN apt update && apt install -y \
    build-essential cmake python3 python3-pip curl git \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set working directory
WORKDIR /app


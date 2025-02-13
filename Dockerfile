# Use a base image with Rust already installed (e.g., Rust official image)
FROM rust:latest

# Install dependencies required by Raylib and CMake
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    git \
    libglfw3-dev \
    libpng-dev \
    libopenal-dev \
    libxcursor-dev \
    libxrandr-dev \
    libxi-dev \
    libxinerama-dev \
    pkg-config \
    curl \
    wget

# Clone the Raylib repository (or you can use the rust crate)
RUN git clone https://github.com/raysan5/raylib.git /raylib

# Set the working directory to the Raylib project
WORKDIR /raylib

# Build Raylib (if you want to compile from source)
RUN cmake . && make

# Set up the working directory for your Rust project
WORKDIR /app

# Copy the cargo.toml and cargo.lock to the container
COPY spellcaster-rs/Cargo.toml spellcaster-rs/Cargo.lock ./

# Build your project dependencies
RUN cargo fetch

# Copy the rest of your project files into the container
COPY . .

# Build your Rust project
RUN cargo build --release

# Run the project (optional, you can also run it with `cargo run` in a separate step)
CMD ["cargo", "run"]

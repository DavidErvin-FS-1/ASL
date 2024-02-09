# Use an image with Rust installed
FROM rust:latest

# Create a new directory for our Rust project
WORKDIR /app

# Copy the source code into the container
COPY hello.rs .
COPY Cargo.toml .

# Compile the Rust program
RUN cargo build --release

# Run the program when the container starts
CMD ["./target/release/hello"]

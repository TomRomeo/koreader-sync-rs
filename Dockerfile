
# Stage 1: Build the Rust binary
FROM rust:1-bullseye AS builder

# Set the working directory
WORKDIR /app

# Copy the project files
COPY . .

# Build the release binary
RUN cargo build --release

# Stage 2: Create a minimal image with the compiled binary
FROM rust:1-bullseye

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/koreader-sync-rs /app/koreader-sync-rs

# Expose the application port
EXPOSE 3000

# Command to run the binary
ENTRYPOINT ["/app/koreader-sync-rs"]

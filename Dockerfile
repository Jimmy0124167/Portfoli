# Build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copy backend code
COPY backend/Cargo.toml backend/Cargo.lock* ./
COPY backend/src ./src

# Build the Rust app
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install required runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from builder
COPY --from=builder /app/target/release/kitso_portfolio_backend .

# Copy frontend files
COPY frontend ./frontend

# Expose port (Railway will override with PORT env var)
EXPOSE 8080

# Run the app
CMD ["./kitso_portfolio_backend"]

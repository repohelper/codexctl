# PolyCLI - Docker Image
# Multi-stage build for minimal size

# Stage 1: Build
FROM rust:1.94-slim-bookworm AS builder

WORKDIR /usr/src/polycli

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build release binary
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

LABEL maintainer="Bhanu Korthiwada"
LABEL description="PolyCLI - Universal AI CLI Profile Manager"
LABEL version="0.1.0"

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create non-root user
RUN useradd -m -s /bin/bash polycli

# Copy binary from builder
COPY --from=builder /usr/src/polycli/target/release/polycli /usr/local/bin/polycli
COPY --from=builder /usr/src/polycli/target/release/poly /usr/local/bin/poly

# Set permissions
RUN chmod +x /usr/local/bin/polycli /usr/local/bin/poly

# Create profiles directory
RUN mkdir -p /home/polycli/.local/share/polycli && \
    chown -R polycli:polycli /home/polycli

# Switch to non-root user
USER polycli

# Set environment
ENV POLYCLI_DIR=/home/polycli/.local/share/polycli
ENV RUST_LOG=info

# Volume for persistent profiles
VOLUME ["/home/polycli/.local/share/polycli"]

# Default command
ENTRYPOINT ["polycli"]
CMD ["--help"]

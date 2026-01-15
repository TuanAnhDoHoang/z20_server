# -------------------
#  Builder Stage
# -------------------
FROM ubuntu:24.04 AS builder

ENV DEBIAN_FRONTEND=noninteractive

# Install build dependencies in one layer
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    wget \
    tar \
    pkg-config \
    libssl-dev \
    build-essential \
    git \
    && rm -rf /var/lib/apt/lists/* /var/cache/apt/*

WORKDIR /tmp

ARG SYSTEM=ubuntu-x86_64
ARG SUI_TAG=testnet-v1.57.0

# ── Download Walrus CLI ───────────────────────────────────────────────
RUN curl -L "https://storage.googleapis.com/mysten-walrus-binaries/walrus-testnet-latest-${SYSTEM}" \
    -o /usr/local/bin/walrus && chmod +x /usr/local/bin/walrus

# ── Download Site Builder ─────────────────────────────────────────────
RUN curl -L "https://storage.googleapis.com/mysten-walrus-binaries/site-builder-testnet-latest-${SYSTEM}" \
    -o /usr/local/bin/site-builder && chmod +x /usr/local/bin/site-builder

# ── Download & extract Sui CLI ────────────────────────────────────────
RUN wget -q https://github.com/MystenLabs/sui/releases/download/${SUI_TAG}/sui-${SUI_TAG}-${SYSTEM}.tgz && \
    tar -xzf sui-${SUI_TAG}-${SYSTEM}.tgz -C /usr/local/bin && \
    chmod +x /usr/local/bin/sui && \
    rm sui-${SUI_TAG}-${SYSTEM}.tgz

# ── Configure Walrus & Sui ────────────────────────────────────────────
RUN mkdir -p /root/.config/walrus /etc/walrus

RUN curl -L https://docs.wal.app/setup/client_config.yaml \
    -o /root/.config/walrus/client_config.yaml

COPY <<EOF /etc/walrus/sites-config.yaml
contexts:
  testnet:
    package: 0xf99aee9f21493e1590e7e5a9aea6f343a1f381031a04a732724871fc294be799
    staking_object: 0xbe46180321c30aab2f8b3501e24048377287fa708018a5b7c2792b35fe339ee3
    general:
      wallet_env: testnet
      walrus_context: testnet
default_context: testnet
EOF

# ── Initialize Sui client with testnet config ─────────────────────────
RUN sui genesis --force && \
    sui client new-address ed25519 && \
    sui keytool import \
        'suiprivkey1qr4sxtdmf7hkl7qarnrs46dm2xdd0l9aran6x45zq5hjvd7vp5leq8xe9uy' \
        ed25519 \
        --alias imported-key && \
    sui client new-env --alias testnet --rpc https://fullnode.testnet.sui.io:443 && \
    sui client switch --env testnet && \
    sui client switch --address 0xf2b8341fc93d683292ba428dccf83ba443c15ee19b9f0719bdd0a7f75218c926

# ── Install Rust toolchain ────────────────────────────────────────────
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"

# ── Build Rust application ────────────────────────────────────────────
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Pre-cache dependencies
RUN cargo build --release && rm -rf target/release/deps/server* target/release/.fingerprint/server*

# Build application
RUN cargo build --release --bin server

# -------------------
#  Runtime Stage
# -------------------
FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/* /var/cache/apt/*

# Copy binaries and configurations from builder
COPY --from=builder /usr/local/bin/sui          /usr/local/bin/
COPY --from=builder /usr/local/bin/walrus       /usr/local/bin/
COPY --from=builder /usr/local/bin/site-builder /usr/local/bin/
COPY --from=builder /root/.config/walrus        /root/.config/walrus/
COPY --from=builder /root/.sui                  /root/.sui/
COPY --from=builder /etc/walrus                 /etc/walrus/
COPY --from=builder /app/target/release/server  /usr/local/bin/server

# Create non-root user for security
# RUN groupadd -r appuser && useradd -r -g appuser -m -d /home/appuser appuser && \
#     chown -R appuser:appuser /home/appuser

# WORKDIR /home/appuser
# USER appuser

# Create dist directory
RUN mkdir -p /root/dist

WORKDIR /root

# Environment configuration
ENV CARGO_ENV=production \
    APP_HOST=0.0.0.0 \
    APP_PORT=5000 \
    RUST_LOG=info

EXPOSE 5000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:5000/api/v1/ || exit 1

ENTRYPOINT ["server", "--cargo-env", "production", "--app-host", "0.0.0.0", "--app-port", "5000"]
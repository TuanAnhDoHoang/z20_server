FROM ubuntu:24.04

RUN apt-get update && apt-get install -y curl ca-certificates wget tar && rm -rf /var/lib/apt/lists/*

ARG SYSTEM=ubuntu-x86_64
ARG SUI_TAG=testnet-v1.57.0  # Latest testnet tag từ releases (check github.com/MystenLabs/sui/releases để update nếu cần)

# Download walrus CLI testnet latest
RUN curl https://storage.googleapis.com/mysten-walrus-binaries/walrus-testnet-latest-${SYSTEM} -o /usr/local/bin/walrus && \
    chmod +x /usr/local/bin/walrus

# Download site-builder testnet latest
RUN curl https://storage.googleapis.com/mysten-walrus-binaries/site-builder-testnet-latest-${SYSTEM} -o /usr/local/bin/site-builder && \
    chmod +x /usr/local/bin/site-builder

# Download và extract Sui CLI testnet từ GitHub releases (.tgz)
RUN wget https://github.com/MystenLabs/sui/releases/download/${SUI_TAG}/sui-${SUI_TAG}-${SYSTEM}.tgz && \
    tar -xzf sui-${SUI_TAG}-${SYSTEM}.tgz && \
    mv sui /usr/local/bin/sui && \
    chmod +x /usr/local/bin/sui && \
    rm sui-${SUI_TAG}-${SYSTEM}.tgz

# Walrus client_config.yaml
RUN mkdir -p /root/.config/walrus && \
    curl https://docs.wal.app/setup/client_config.yaml -o /root/.config/walrus/client_config.yaml

# sites-config.yaml hardcode testnet
RUN mkdir -p /etc/walrus && cat <<EOF > /etc/walrus/sites-config.yaml
contexts:
  testnet:
    package: 0xf99aee9f21493e1590e7e5a9aea6f343a1f381031a04a732724871fc294be799
    staking_object: 0xbe46180321c30aab2f8b3501e24048377287fa708018a5b7c2792b35fe339ee3
    general:
      wallet_env: testnet
      walrus_context: testnet
default_context: testnet
EOF

WORKDIR /site

ENTRYPOINT ["site-builder", "--config", "/etc/walrus/sites-config.yaml", "--wallet", "/root/.sui/sui_config/client1.yaml"]

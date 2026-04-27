ARG TAG
FROM clux/muslrust:${TAG:-1.85.1-stable-2025-03-29}

# ==========================================================
# 1. 追加ツールのインストール（最小限）
# ==========================================================
# - curl: cargo-chefインストール用
# - sccache: ビルドキャッシュ
# ==========================================================
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl ca-certificates xz-utils \
    && rm -rf /var/lib/apt/lists/*

# ==========================================================
# 2. cargoツールのインストール
# ==========================================================
# cargo install は ~/.cargo/bin に入るため PATH 追加
ENV PATH="/root/.cargo/bin:${PATH}"

ENV SCCACHE_VERSION=v0.12.0

RUN curl -L https://github.com/mozilla/sccache/releases/download/${SCCACHE_VERSION}/sccache-${SCCACHE_VERSION}-x86_64-unknown-linux-musl.tar.gz \
    | tar xz && mv sccache-*/sccache /usr/local/bin/sccache && chmod +x /usr/local/bin/sccache && rm -rf sccache-*

RUN curl -L https://github.com/LukeMathWalker/cargo-chef/releases/latest/download/cargo-chef-x86_64-unknown-linux-musl.tar.xz \
    | tar xJ && mv cargo-chef-x86_64-unknown-linux-musl/cargo-chef /usr/local/bin/cargo-chef && chmod +x /usr/local/bin/cargo-chef

# ==========================================================
# 3. sccache の設定
# ==========================================================
ENV RUSTC_WRAPPER=sccache \
    SCCACHE_DIR=/var/cache/sccache \
    SCCACHE_CACHE_SIZE="10G" \
    CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=rust-lld \
    CARGO_BUILD_RUSTC_LINKER=rust-lld \
    CARGO_BUILD_RUSTC=rustc

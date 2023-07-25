# ------------- build ----------------
FROM clux/muslrust:1.67.1 as builder

ENV CARGO_HOME=/cargo

RUN mkdir -p /rust && mkdir -p /cargo
WORKDIR /rust

# ソースコードのコピー
COPY Cargo.toml Cargo.lock /rust/
COPY src /rust/src

# 複数バイナリ作成される場合等は修正すること
RUN --mount=type=cache,target=/rust/target \
--mount=type=cache,target=/root/.cargo/registry \
--mount=type=cache,target=/root/.cargo/git \
cargo build --release && \
cp /rust/target/x86_64-unknown-linux-musl/release/tcp-logger /app


# ------------- runtime ----------------
FROM scratch

# バイナリのコピー
COPY --from=builder /app /app

ENTRYPOINT [ "/app" ]

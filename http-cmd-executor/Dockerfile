# ------------- build ----------------
FROM clux/muslrust:1.85.1-stable-2025-03-29 AS builder

RUN mkdir -p /rust && mkdir -p /cargo
WORKDIR /rust

RUN groupadd -g 999 app && \
    useradd -d /app -s /bin/bash -u 999 -g 999 app

# ソースコードのコピー
COPY Cargo.toml Cargo.lock /rust/
COPY src /rust/src

# バイナリ生成
RUN --mount=type=cache,target=/rust/target \
    --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/root/.cargo/git \
    cargo build --release --bin http-cmd-executor && \
    cp /rust/target/x86_64-unknown-linux-musl/release/http-cmd-executor /http-cmd-executor

# ------------- server ----------------
FROM scratch AS server

ENV RUST_LOG=info,sqlx::query=error

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /http-cmd-executor /http-cmd-executor

USER 999
EXPOSE 8000
ENTRYPOINT [ "/http-cmd-executor" ]

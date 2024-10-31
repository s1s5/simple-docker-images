# ------------- build ----------------
FROM clux/muslrust:1.82.0-stable AS builder

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
    cargo build --release --bin dropbox-upload-rs && \
    cp /rust/target/x86_64-unknown-linux-musl/release/dropbox-upload-rs /app

# ------------- server ----------------
FROM scratch AS server

ENV RUST_LOG=info

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /app /app

USER 999
EXPOSE 8000
ENTRYPOINT [ "/app" ]

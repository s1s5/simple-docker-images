FROM clux/muslrust:1.82.0-stable AS builder

# 作業用ディレクトリ
RUN mkdir -p /rust && mkdir -p /cargo
WORKDIR /rust

# サービスを走らせるためのユーザー
RUN groupadd -g 999 app && \
    useradd -d /app -s /bin/bash -u 999 -g 999 app

# ソースコードのコピー
COPY Cargo.toml Cargo.lock /rust/
COPY src /rust/src

# バイナリ名を変更すること
RUN --mount=type=cache,target=/rust/target \
    --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/root/.cargo/git \
    cargo build --release && \
    cp /rust/target/x86_64-unknown-linux-musl/release/heart-beater-rust /app

# ------------- runtime ----------------
FROM scratch

# sentryなど外部サービスに接続する際に証明書が必要な場合はコピー
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

# バイナリのコピー
COPY --from=builder /app /app

# ユーザー設定のコピー
COPY --from=builder /etc/passwd /etc/passwd

# k8sでrunAsNonRootを設定するときには数字で指定するほうが都合がいい
USER 999
ENV RUST_LOG=info
ENTRYPOINT [ "/app" ]

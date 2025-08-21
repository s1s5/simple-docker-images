# export RUST_VERSION=1.86; docker buildx build --platform linux/amd64,linux/arm64 --build-arg RUST_VERSION=${RUST_VERSION} -t s1s5/rust-dev:${RUST_VERSION} --push .

ARG RUST_VERSION=1.86

FROM rust:${RUST_VERSION}-bullseye
RUN apt update && apt install -y musl-dev

# install mold:https://github.com/rui314/mold
ENV mold_version=2.40.2
RUN cd /tmp && \
    wget https://github.com/rui314/mold/releases/download/v${mold_version}/mold-${mold_version}-x86_64-linux.tar.gz && \
    tar zxf mold-${mold_version}-x86_64-linux.tar.gz && \
    cp -r mold-${mold_version}-x86_64-linux/* /usr/local/ && \
    rm -rf /tmp/mold-${mold_version}-x86_64-linux

RUN set -eux; \
    apt-get update; \
    apt-get install -y gosu clang-16; \
    rm -rf /var/lib/apt/lists/*; \
    gosu nobody true

RUN ln -s /usr/bin/clang-16 /usr/bin/clang
RUN ln -s /usr/bin/clang++-16 /usr/bin/clang++

# install packages
RUN cargo install cargo-watch
# RUN cargo install sea-orm-cli
RUN rustup component add rustfmt clippy

# usr for dev
ENV USER_ID=1000
ENV GROUP_ID=1000
ENV HOME=/home/dev
ENV CARGO_HOME=/home/dev/.cargo

RUN mkdir -p /home/dev/.cargo
VOLUME /home/dev/.cargo
WORKDIR /usr/src

ENV TINI_VERSION=v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini /tini
RUN chmod +x /tini

ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="clang"
ENV RUSTFLAGS="-C link-arg=-fuse-ld=/usr/local/bin/mold"
ENV RUST_LOG=debug
ENV RUST_BACKTRACE=full
ENV CARGO_TERM_COLOR=auto
ENV CARGO_TERM_PROGRESS_WHEN=never

# mount project/ => /usr/src
# COPY ./entrypoint.sh /entrypoint.sh
# ENTRYPOINT ["/tini", "--", "/entrypoint.sh"]
# CMD ["cargo", "watch", "-x", "fmt", "-s", "mold -run cargo run"]

ENV CARGO_HOME=/opt/cargo
RUN mkdir -p /opt/cargo && chmod 1777 /opt/cargo
VOLUME /opt/cargo
ENTRYPOINT ["/tini", "--"]

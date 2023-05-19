FROM rust:1.69-buster
RUN apt update && apt install -y musl-dev

# install mold:https://github.com/rui314/mold
ENV mold_version=1.10.1
RUN cd /tmp && \
    wget https://github.com/rui314/mold/releases/download/v${mold_version}/mold-${mold_version}-x86_64-linux.tar.gz && \
    tar zxf mold-${mold_version}-x86_64-linux.tar.gz && \
    cp -r mold-${mold_version}-x86_64-linux/* /usr/local/ && \
    rm -rf /tmp/mold-${mold_version}-x86_64-linux

RUN set -eux; \
    apt-get update; \
    apt-get install -y gosu; \
    rm -rf /var/lib/apt/lists/*; \
    gosu nobody true

# install packages
RUN cargo install cargo-watch
RUN cargo install sea-orm-cli
RUN rustup component add rustfmt

# usr for dev
ENV USER_NAME=dev
ENV GROUP_NAME=dev
ENV USER_ID=1000
ENV GROUP_ID=1000

WORKDIR /usr/src

ENV TINI_VERSION v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini /tini
RUN chmod +x /tini

# mount project/ => /usr/src
COPY ./entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/tini", "--", "/entrypoint.sh"]
CMD ["cargo", "watch", "-x", "fmt", "-s", "mold -run cargo run"]

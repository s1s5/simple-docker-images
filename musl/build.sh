#!/bin/bash
# -*- mode: shell-script -*-

set -eu  # <= 0以外が返るものがあったら止まる, 未定義の変数を使おうとしたときに打ち止め

docker pull messense/rust-musl-cross:x86_64-musl
docker image tag messense/rust-musl-cross:x86_64-musl s1s5/musl:amd64
docker push s1s5/musl:amd64

docker pull messense/rust-musl-cross:aarch64-musl
docker image tag messense/rust-musl-cross:aarch64-musl s1s5/musl:arm64
docker push s1s5/musl:arm64

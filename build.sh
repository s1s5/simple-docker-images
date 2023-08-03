#!/bin/bash
# -*- mode: shell-script -*-

set -eu  # <= 0以外が返るものがあったら止まる, 未定義の変数を使おうとしたときに打ち止め


for foldar in dropbox-download dropbox-upload gpg-encrypt-asym gpg-encrypt-sym gql-schema-dumper rover-supergraph-compose s3-download-to-tar http-debugger rust-dev tcp-logger http-echo-logger node-watchfiles s3-download-latest; do
    docker build ${foldar} -t s1s5/${foldar}
    docker push s1s5/${foldar}
done


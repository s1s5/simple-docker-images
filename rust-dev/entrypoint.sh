#!/bin/bash
# -*- mode: shell-script -*-

set -eu  # <= 0以外が返るものがあったら止まる, 未定義の変数を使おうとしたときに打ち止め


if getent group dev >/dev/null; then
    :
else
    groupadd -g $GROUP_ID dev
    useradd -m -s /bin/bash -u $USER_ID -g $GROUP_ID dev

    echo "[target.`rustc -vV | grep host | awk '{print $2}'`]" > /home/dev/.cargo/config.toml
    echo 'linker = "clang"' >> /home/dev/.cargo/config.toml
    echo 'rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/mold"]' >> /home/dev/.cargo/config.toml

    chown -R dev:dev /home/dev
fi

if [ -e /usr/src/target ]; then
    chown -R $USER_ID:$GROUP_ID /usr/src/target
fi

exec gosu dev "$@"

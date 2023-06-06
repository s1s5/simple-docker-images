#!/bin/bash
# -*- mode: shell-script -*-

set -eu  # <= 0以外が返るものがあったら止まる, 未定義の変数を使おうとしたときに打ち止め


if getent group dev >/dev/null; then
    :
else
    groupadd -g $GROUP_ID dev
    useradd -m -s /bin/bash -u $USER_ID -g $GROUP_ID dev

    chown -R dev:dev /home/dev
fi

exec gosu dev "$@"
